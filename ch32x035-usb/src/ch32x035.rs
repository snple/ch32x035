//! CH32X035 USBFS device-mode bus implementation.
//!
//! The WCH USBFS controller is a packet-oriented full-speed device: the SIE
//! owns the bus protocol (handshakes, retries) and exposes one interrupt per
//! completed token, while the CPU supplies 64-byte endpoint buffers through
//! per-endpoint DMA address registers.
//!
//! # Endpoint mapping
//!
//! EP0 is the control endpoint. EP4 shares its buffer with EP0 on this part,
//! so it is not offered for allocation; the remaining endpoints (1, 2, 3, 5,
//! 6 and 7) can each carry an IN and an OUT direction simultaneously.
//!
//! # Data toggle
//!
//! The controller has manual data-toggle control. Every acknowledged packet
//! toggles the endpoint's `T_TOG`/`R_TOG` bit (EP0 starts at DATA1 after a
//! SETUP, all other endpoints at DATA0), matching the WCH SDK.

use core::cell::{Cell, UnsafeCell};
use core::ptr;

use ch32x035::afio::Afio;
use ch32x035::pac;
use ch32x035::rcc::{AhbPeriph, AhbReset, Rcc};

use crate::device::bus::{PollResult, UsbBus};
use crate::device::error::{UsbError, UsbResult};
use crate::endpoint::{Direction, EndpointAddress, EndpointType};

// ─── Hardware constants (WCH `ch32x035_usb.h`) ───────────────────────────────

const EP_SIZE: usize = 64;
const NUM_EP: usize = 8;
/// Largest control OUT data stage the bus stages before delivering it.
const CONTROL_STAGE_MAX: usize = 256;
/// Endpoints available for class allocation (EP0 is reserved, EP4 shares
/// EP0's buffer on this part).
const ALLOC_ORDER: [u8; 6] = [1, 2, 3, 5, 6, 7];

// BASE_CTRL
const UC_DEV_PU_EN: u32 = 0x20;
const UC_INT_BUSY: u32 = 0x08;
const UC_RST_SIE: u32 = 0x04;
const UC_CLR_ALL: u32 = 0x02;
const UC_DMA_EN: u32 = 0x01;

// UDEV_CTRL
const UD_PD_DIS: u32 = 0x80;
const UD_PORT_EN: u32 = 0x01;

// INT_EN
const UIE_SUSPEND: u32 = 0x04;
const UIE_TRANSFER: u32 = 0x02;
const UIE_BUS_RST: u32 = 0x01;

// INT_FG (write 1 to clear)
const UIF_FIFO_OV: u32 = 0x10;
const UIF_SUSPEND: u32 = 0x04;
const UIF_TRANSFER: u32 = 0x02;
const UIF_BUS_RST: u32 = 0x01;

// INT_ST
const UIS_TOG_OK: u32 = 0x40;
const UIS_TOKEN_MASK: u32 = 0x30;
const UIS_TOKEN_OUT: u32 = 0x00;
const UIS_TOKEN_SOF: u32 = 0x10;
const UIS_TOKEN_IN: u32 = 0x20;
const UIS_TOKEN_SETUP: u32 = 0x30;
const UIS_ENDP_MASK: u32 = 0x0f;

// MIS_ST
const UMS_SUSPEND: u32 = 0x04;

// UEPn_CTRL_H
const UEP_R_TOG: u16 = 1 << 7;
const UEP_T_TOG: u16 = 1 << 6;
const UEP_R_RES_MASK: u16 = 3 << 2;
const UEP_T_RES_MASK: u16 = 3;
const UEP_R_RES_ACK: u16 = 0 << 2;
const UEP_R_RES_NAK: u16 = 2 << 2;
const UEP_R_RES_STALL: u16 = 3 << 2;
const UEP_T_RES_ACK: u16 = 0;
const UEP_T_RES_NAK: u16 = 2;
const UEP_T_RES_STALL: u16 = 3;

// Endpoint state bit maps: bit `n*2` is the IN direction of endpoint `n`,
// bit `n*2 + 1` the OUT direction.
const DIR_IN: u16 = 1 << 0;
const DIR_OUT: u16 = 1 << 1;

#[inline(always)]
const fn ep_bit(n: usize, dir: u16) -> u16 {
    dir << (n * 2)
}

// ─── Endpoint RAM ────────────────────────────────────────────────────────────

/// Endpoint RAM layout.
///
/// Each endpoint owns a 128-byte region. When both directions are enabled the
/// SIE places the OUT buffer at offset 0 and the IN buffer at offset 64
/// (single-buffer mode); endpoints using one direction only start at offset 0.
/// EP0 uses the first 64 bytes of its region for both directions.
#[repr(C, align(4))]
struct Buffers {
    ep: [[u8; 2 * EP_SIZE]; NUM_EP],
}

struct BuffersCell(UnsafeCell<Buffers>);
// SAFETY: the controller is a singleton and all access happens from one core;
// concurrent access is serialized by the driver (one `&UsbfsBus` at a time).
unsafe impl Sync for BuffersCell {}

static BUFFERS: BuffersCell = BuffersCell(UnsafeCell::new(Buffers {
    ep: [[0; 2 * EP_SIZE]; NUM_EP],
}));

// ─── Configuration ───────────────────────────────────────────────────────────

/// Bus configuration.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Config {
    /// Board supply, which selects the internal D+ pull-up value.
    pub supply: ch32x035::afio::UsbPhySupply,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            supply: ch32x035::afio::UsbPhySupply::V33,
        }
    }
}

// ─── Bus ─────────────────────────────────────────────────────────────────────

/// USBFS device-mode bus.
pub struct UsbfsBus {
    usbfs: pac::USBFS,
    regs: &'static pac::usbfs::RegisterBlock,
    /// Bitmap of allocated directions per endpoint.
    ep_alloc: Cell<u16>,
    /// Bitmap of directions whose buffer currently holds a packet.
    ep_pending: Cell<u16>,
    /// Bitmap of stalled directions per endpoint.
    ep_stalled: Cell<u16>,
    /// Per-endpoint maximum packet size.
    ep_max: [u8; NUM_EP],
    /// Last OUT packet length per endpoint.
    ep_rx_len: Cell<[u16; NUM_EP]>,
    /// SETUP packet staged by the bus.
    setup: Cell<[u8; 8]>,
    setup_ready: Cell<bool>,
    /// Control OUT data stage staging buffer.
    ctrl_buf: Cell<[u8; CONTROL_STAGE_MAX]>,
    ctrl_ready: Cell<bool>,
    /// `true` when the current control transfer's data stage is IN.
    ctrl_dir_in: Cell<bool>,
    /// Requested control data stage length (`wLength`).
    ctrl_wlength: Cell<u16>,
    /// Bytes of the control data stage received so far.
    ctrl_count: Cell<u16>,
}

// SAFETY: the USBFS peripheral is a singleton owned by this struct and only
// accessed from a single core.
unsafe impl Send for UsbfsBus {}
unsafe impl Sync for UsbfsBus {}

impl UsbfsBus {
    /// Create and configure the USBFS bus.
    ///
    /// Enables the USBFS clock, resets the controller and configures the USB
    /// pins' PHY (`afio`). The pull-up is only enabled by
    /// [`UsbBus::enable`], which the allocator calls once all endpoints have
    /// been allocated, so the device does not enumerate with a stale
    /// descriptor set.
    ///
    /// The application must configure PC16 (D−) and PC17 (D+) as inputs.
    pub fn new(usbfs: pac::USBFS, afio: &mut Afio, rcc: &mut Rcc, config: Config) -> Self {
        rcc.ahb_enable(AhbPeriph::Usbfs);
        rcc.ahb_reset(AhbReset::Usbfs);
        rcc.ahb_clear_reset(AhbReset::Usbfs);

        afio.configure_usb_phy(config.supply);

        // SAFETY: USBFS is a singleton peripheral at a fixed address; the
        // register block lives for the whole program.
        let regs: &'static pac::usbfs::RegisterBlock =
            unsafe { &*(pac::usbfs::BASE as *const pac::usbfs::RegisterBlock) };

        // Reset the SIE and flush the endpoint FIFOs.
        regs.base_ctrl.write_bits(UC_RST_SIE | UC_CLR_ALL);
        short_delay();
        regs.base_ctrl.write_bits(0);

        Self {
            usbfs,
            regs,
            ep_alloc: Cell::new(0),
            ep_pending: Cell::new(0),
            ep_stalled: Cell::new(0),
            ep_max: [EP_SIZE as u8; NUM_EP],
            ep_rx_len: Cell::new([0; NUM_EP]),
            setup: Cell::new([0; 8]),
            setup_ready: Cell::new(false),
            ctrl_buf: Cell::new([0; CONTROL_STAGE_MAX]),
            ctrl_ready: Cell::new(false),
            ctrl_dir_in: Cell::new(false),
            ctrl_wlength: Cell::new(0),
            ctrl_count: Cell::new(0),
        }
    }

    /// Escape hatch: the underlying PAC peripheral.
    pub fn regs(&self) -> &'static pac::usbfs::RegisterBlock {
        self.regs
    }

    /// Consume the driver and return the PAC peripheral.
    pub fn release(self) -> pac::USBFS {
        self.usbfs
    }

    // ── Register helpers ──────────────────────────────────────────────────

    #[inline(always)]
    fn dma_reg(&self, n: usize) -> &'static pac::common::VolatileCell<u32> {
        match n {
            1 => &self.regs.uep1_dma,
            2 => &self.regs.uep2_dma_host_rx_dma,
            3 => &self.regs.uep3_dma_host_tx_dma,
            5 => &self.regs.uep5_dma,
            6 => &self.regs.uep6_dma,
            _ => &self.regs.uep7_dma,
        }
    }

    #[inline(always)]
    fn tx_len_reg(&self, n: usize) -> &'static pac::common::VolatileCellU16 {
        match n {
            0 => &self.regs.uep0_tx_len,
            1 => &self.regs.uep1_tx_len,
            2 => &self.regs.uep2_tx_len,
            3 => &self.regs.uep3_tx_len,
            5 => &self.regs.uep5_tx_len,
            6 => &self.regs.uep6_tx_len,
            _ => &self.regs.uep7_tx_len,
        }
    }

    #[inline(always)]
    fn ctrl_reg(&self, n: usize) -> &'static pac::common::VolatileCellU16 {
        match n {
            0 => &self.regs.uep0_ctrl_h,
            1 => &self.regs.uep1_ctrl_h_host_setup,
            2 => &self.regs.uep2_ctrl_h,
            3 => &self.regs.uep3_ctrl_h,
            5 => &self.regs.uep5_ctrl_h,
            6 => &self.regs.uep6_ctrl_h,
            _ => &self.regs.uep7_ctrl_h,
        }
    }

    /// `(mod register index, field shift, field width, TX bit, RX bit)` for
    /// the endpoint buffer-mode bits.
    ///
    /// EP1/2/3 use a 4-bit field (TX = bit 2, RX = bit 3); EP4 shares EP0's
    /// buffer and is never enabled; EP5/6/7 use 2-bit fields (TX = bit 0,
    /// RX = bit 1).
    #[inline(always)]
    fn mod_field(n: usize) -> (u8, u32, u32, u32, u32) {
        match n {
            1 => (0, 4, 4, 1 << 2, 1 << 3),
            2 => (1, 0, 4, 1 << 2, 1 << 3),
            3 => (1, 4, 4, 1 << 2, 1 << 3),
            5 => (2, 0, 2, 1 << 0, 1 << 1),
            6 => (2, 2, 2, 1 << 0, 1 << 1),
            _ => (2, 4, 2, 1 << 0, 1 << 1),
        }
    }

    fn set_mod_bits(&self, index: u8, shift: u32, width: u32, bits: u32) {
        let mask = ((1u32 << width) - 1) << shift;
        let value = (bits & ((1 << width) - 1)) << shift;
        match index {
            0 => self.regs.uep4_1_mod.modify_bits(|v| (v & !mask) | value),
            1 => self
                .regs
                .uep2_3_mod_host_ep_mod
                .modify_bits(|v| (v & !mask) | value),
            _ => self.regs.uep567_mod.modify_bits(|v| (v & !mask) | value),
        }
    }

    #[inline(always)]
    fn ep_base(n: usize) -> *mut u8 {
        // SAFETY: `BUFFERS` is a singleton and the pointer is only used while
        // the driver holds exclusive access to the endpoint buffer.
        unsafe { (*BUFFERS.0.get()).ep[n].as_mut_ptr() }
    }

    #[inline(always)]
    fn buf_in_ptr(&self, n: usize) -> *mut u8 {
        // SAFETY: see `ep_base`.
        unsafe {
            if n != 0 && self.is_out_used(n) {
                Self::ep_base(n).add(EP_SIZE)
            } else {
                Self::ep_base(n)
            }
        }
    }

    #[inline(always)]
    fn buf_out_ptr(n: usize) -> *mut u8 {
        Self::ep_base(n)
    }

    // ── Endpoint control ──────────────────────────────────────────────────

    fn ctrl_read(&self, n: usize) -> u16 {
        self.ctrl_reg(n).read_bits() as u16
    }

    fn write_ctrl(&self, n: usize, value: u16) {
        self.ctrl_reg(n).write_bits(value as u32);
    }

    /// Program the handshake responses and data toggles of an endpoint.
    fn set_resp(&self, n: usize, t_res: u16, r_res: u16) {
        let value = (self.ctrl_read(n) & !(UEP_T_RES_MASK | UEP_R_RES_MASK))
            | (t_res & UEP_T_RES_MASK)
            | (r_res & UEP_R_RES_MASK);
        self.write_ctrl(n, value);
    }

    #[inline(always)]
    fn is_used(&self, n: usize, dir: u16) -> bool {
        self.ep_alloc.get() & ep_bit(n, dir) != 0
    }

    #[inline(always)]
    fn is_in_used(&self, n: usize) -> bool {
        self.is_used(n, DIR_IN)
    }

    #[inline(always)]
    fn is_out_used(&self, n: usize) -> bool {
        self.is_used(n, DIR_OUT)
    }

    fn set_bit(cell: &Cell<u16>, bit: u16, on: bool) {
        let mut value = cell.get();
        if on {
            value |= bit;
        } else {
            value &= !bit;
        }
        cell.set(value);
    }

    fn ep_reset(&self) {
        self.ep_pending.set(0);
        self.ep_stalled.set(0);
        self.setup_ready.set(false);
        self.ctrl_ready.set(false);
        self.ctrl_count.set(0);
        self.ctrl_wlength.set(0);

        // Buffer-mode configuration for every allocated endpoint.
        for n in ALLOC_ORDER {
            let n = n as usize;
            let (index, shift, width, tx_bit, rx_bit) = Self::mod_field(n);
            let mut bits = 0;
            if self.is_in_used(n) {
                bits |= tx_bit;
            }
            if self.is_out_used(n) {
                bits |= rx_bit;
            }
            self.set_mod_bits(index, shift, width, bits);
        }
        // EP4 shares EP0's buffer on this part and stays disabled.
        self.set_mod_bits(0, 0, 4, 0);

        // EP0 always uses the shared 64-byte buffer.
        self.regs.uep0_dma.write_bits(Self::buf_out_ptr(0) as u32);
        for n in ALLOC_ORDER {
            let n = n as usize;
            if self.is_in_used(n) || self.is_out_used(n) {
                self.dma_reg(n).write_bits(Self::buf_out_ptr(n) as u32);
            }
            self.tx_len_reg(n).write_bits(0);
        }

        // EP0: accept SETUP with DATA1 toggles, no data pending.
        self.write_ctrl(0, UEP_T_TOG | UEP_T_RES_NAK | UEP_R_TOG | UEP_R_RES_ACK);

        for n in ALLOC_ORDER {
            let n = n as usize;
            let mut value = 0;
            if self.is_in_used(n) {
                value |= UEP_T_RES_NAK;
            }
            if self.is_out_used(n) {
                value |= UEP_R_RES_ACK;
            }
            self.write_ctrl(n, value);
        }

        self.regs.dev_addr.write_bits(0);
    }

    // ── Token handlers ────────────────────────────────────────────────────

    fn handle_setup(&self) {
        let mut raw = [0u8; 8];
        // SAFETY: single-core access; the SETUP packet sits in the EP0 buffer.
        unsafe {
            ptr::copy_nonoverlapping(Self::buf_out_ptr(0), raw.as_mut_ptr(), 8);
        }
        let wlength = u16::from_le_bytes([raw[6], raw[7]]);
        let dir_in = raw[0] & 0x80 != 0;

        self.setup.set(raw);
        self.setup_ready.set(true);
        self.ctrl_dir_in.set(dir_in);
        Self::set_bit(&self.ep_stalled, ep_bit(0, DIR_IN), false);
        Self::set_bit(&self.ep_stalled, ep_bit(0, DIR_OUT), false);
        self.ctrl_wlength.set(wlength);
        self.ctrl_count.set(0);
        self.ctrl_ready.set(false);

        // Hold both directions with DATA1 toggles until the request handler
        // decides how to answer.
        self.write_ctrl(0, UEP_T_TOG | UEP_T_RES_NAK | UEP_R_TOG | UEP_R_RES_NAK);

        if dir_in {
            // The data (or zero-length) IN packet is armed by `write`.
        } else if wlength as usize > CONTROL_STAGE_MAX {
            // Larger than the staging buffer: stall the request.
            self.set_resp(0, UEP_T_RES_STALL, UEP_R_RES_STALL);
        } else if wlength > 0 {
            // OUT data stage: accept the first packet.
            self.write_ctrl(0, UEP_T_TOG | UEP_T_RES_NAK | UEP_R_TOG | UEP_R_RES_ACK);
        } else {
            // No data stage: the status stage is an IN zero-length packet.
            self.tx_len_reg(0).write_bits(0);
            self.write_ctrl(0, UEP_T_TOG | UEP_T_RES_ACK | UEP_R_TOG | UEP_R_RES_NAK);
        }
    }

    fn handle_in(&self, n: usize) {
        if n == 0 {
            // Toggle for the next data packet, then wait for `write`.
            let value = ((self.ctrl_read(0) ^ UEP_T_TOG) & !UEP_T_RES_MASK) | UEP_T_RES_NAK;
            // After a data/status IN packet the host may send the status OUT
            // (DATA1), so keep EP0's OUT side armed.
            let value = (value & !UEP_R_RES_MASK) | UEP_R_RES_ACK | UEP_R_TOG;
            self.write_ctrl(0, value);
            return;
        }
        // Acknowledge to the class, then wait for the next packet.
        let value = ((self.ctrl_read(n) ^ UEP_T_TOG) & !UEP_T_RES_MASK) | UEP_T_RES_NAK;
        self.write_ctrl(n, value);
        Self::set_bit(&self.ep_pending, ep_bit(n, DIR_IN), false);
    }

    fn handle_out(&self, n: usize) {
        let len = (self.regs.rx_len.read_bits() as u16).min(EP_SIZE as u16);

        if n == 0 {
            if self.ctrl_dir_in.get() {
                // Status stage of a control IN transfer (the host's OUT
                // zero-length packet), or an early transfer abort. Hand the
                // packet to the control pipe and re-arm for the next SETUP.
                self.ctrl_count.set(0);
                self.ctrl_ready.set(true);
                self.write_ctrl(0, UEP_T_TOG | UEP_T_RES_NAK | UEP_R_TOG | UEP_R_RES_ACK);
                return;
            }
            let count = self.ctrl_count.get() as usize;
            let space = (CONTROL_STAGE_MAX - count).min(len as usize);
            // SAFETY: single-core access; copy before re-arming the endpoint.
            unsafe {
                let mut buf = self.ctrl_buf.get();
                ptr::copy_nonoverlapping(Self::buf_out_ptr(0), buf.as_mut_ptr().add(count), space);
                self.ctrl_buf.set(buf);
            }
            let count = (count + len as usize).min(CONTROL_STAGE_MAX) as u16;
            self.ctrl_count.set(count);

            // Advance the expected OUT toggle.
            let value = self.ctrl_read(0) ^ UEP_R_TOG;

            if count < self.ctrl_wlength.get() && (count as usize) < CONTROL_STAGE_MAX {
                let value =
                    (value & !(UEP_R_RES_MASK | UEP_T_RES_MASK)) | UEP_R_RES_ACK | UEP_T_RES_NAK;
                self.write_ctrl(0, value);
            } else {
                // Data stage complete: send the IN zero-length status packet.
                self.ctrl_ready.set(true);
                self.tx_len_reg(0).write_bits(0);
                let value = (value & !(UEP_R_RES_MASK | UEP_T_RES_MASK))
                    | UEP_R_RES_NAK
                    | UEP_T_TOG
                    | UEP_T_RES_ACK;
                self.write_ctrl(0, value);
            }
            return;
        }

        let mut lens = self.ep_rx_len.get();
        lens[n] = len;
        self.ep_rx_len.set(lens);
        Self::set_bit(&self.ep_pending, ep_bit(n, DIR_OUT), true);
        // Hold the packet until the class reads it.
        self.set_resp(n, UEP_T_RES_NAK, UEP_R_RES_NAK);
    }

    // ── Data path ─────────────────────────────────────────────────────────

    fn write_ep0(&self, buf: &[u8]) -> UsbResult<usize> {
        if buf.len() > EP_SIZE {
            return Err(UsbError::BufferOverflow);
        }
        // SAFETY: single-core access to the shared EP0 buffer.
        unsafe {
            ptr::copy_nonoverlapping(buf.as_ptr(), Self::buf_out_ptr(0), buf.len());
        }
        self.tx_len_reg(0).write_bits(buf.len() as u32);
        let value = (self.ctrl_read(0) & !(UEP_T_RES_MASK | UEP_R_RES_MASK))
            | UEP_T_RES_ACK
            | UEP_R_RES_NAK;
        self.write_ctrl(0, value);
        Ok(buf.len())
    }

    fn write_ep(&self, n: usize, buf: &[u8]) -> UsbResult<usize> {
        if buf.len() > self.ep_max[n] as usize {
            return Err(UsbError::BufferOverflow);
        }
        if self.ep_pending.get() & ep_bit(n, DIR_IN) != 0 {
            return Err(UsbError::WouldBlock);
        }
        // SAFETY: single-core access to this endpoint's IN buffer.
        unsafe {
            ptr::copy_nonoverlapping(buf.as_ptr(), self.buf_in_ptr(n), buf.len());
        }
        self.tx_len_reg(n).write_bits(buf.len() as u32);
        self.set_resp(n, UEP_T_RES_ACK, UEP_R_RES_NAK);
        Self::set_bit(&self.ep_pending, ep_bit(n, DIR_IN), true);
        Ok(buf.len())
    }

    fn read_ep0(&self, buf: &mut [u8]) -> UsbResult<usize> {
        if self.setup_ready.get() {
            let setup = self.setup.get();
            if buf.len() < setup.len() {
                return Err(UsbError::BufferOverflow);
            }
            buf[..setup.len()].copy_from_slice(&setup);
            self.setup_ready.set(false);
            return Ok(setup.len());
        }
        if !self.ctrl_ready.get() {
            return Err(UsbError::WouldBlock);
        }
        let staged = self.ctrl_buf.get();
        let len = self.ctrl_count.get() as usize;
        if buf.len() < len {
            return Err(UsbError::BufferOverflow);
        }
        buf[..len].copy_from_slice(&staged[..len]);
        self.ctrl_ready.set(false);
        Ok(len)
    }

    fn read_ep(&self, n: usize, buf: &mut [u8]) -> UsbResult<usize> {
        if self.ep_pending.get() & ep_bit(n, DIR_OUT) == 0 {
            return Err(UsbError::WouldBlock);
        }
        let len = self.ep_rx_len.get()[n] as usize;
        if buf.len() < len {
            return Err(UsbError::BufferOverflow);
        }
        // SAFETY: single-core access to this endpoint's OUT buffer.
        unsafe {
            ptr::copy_nonoverlapping(Self::buf_out_ptr(n), buf.as_mut_ptr(), len);
        }
        Self::set_bit(&self.ep_pending, ep_bit(n, DIR_OUT), false);
        // Advance the expected OUT toggle and re-arm the endpoint.
        let value = self.ctrl_read(n) ^ UEP_R_TOG;
        self.write_ctrl(n, (value & !UEP_R_RES_MASK) | UEP_R_RES_ACK);
        Ok(len)
    }
}

fn short_delay() {
    for _ in 0..4_000 {
        core::hint::spin_loop();
    }
}

// ─── UsbBus implementation ───────────────────────────────────────────────────

impl UsbBus for UsbfsBus {
    fn alloc_ep(
        &mut self,
        dir: Direction,
        addr: Option<EndpointAddress>,
        ep_type: EndpointType,
        max_packet: u16,
        _interval: u8,
    ) -> UsbResult<EndpointAddress> {
        if !matches!(
            ep_type,
            EndpointType::Control | EndpointType::Bulk | EndpointType::Interrupt
        ) {
            return Err(UsbError::Unsupported);
        }
        if max_packet == 0 || max_packet > EP_SIZE as u16 {
            return Err(UsbError::BufferOverflow);
        }

        let number = match addr {
            Some(a) => a.number() as usize,
            None => {
                // Prefer a number already in use in the other direction so
                // that paired endpoints (e.g. CDC bulk IN/OUT) share a number.
                let wanted = match dir {
                    Direction::In => DIR_IN,
                    Direction::Out => DIR_OUT,
                };
                let other = match dir {
                    Direction::In => DIR_OUT,
                    Direction::Out => DIR_IN,
                };
                let state = self.ep_alloc.get();
                let paired = ALLOC_ORDER.iter().copied().find(|&n| {
                    let n = n as usize;
                    state & ep_bit(n, wanted) == 0 && state & ep_bit(n, other) != 0
                });
                match paired {
                    Some(n) => n as usize,
                    None => ALLOC_ORDER
                        .iter()
                        .copied()
                        .map(|n| n as usize)
                        .find(|&n| self.ep_alloc.get() & ep_bit(n, wanted) == 0)
                        .ok_or(UsbError::EndpointOverflow)?,
                }
            }
        };

        if number >= NUM_EP || number == 4 {
            return Err(UsbError::InvalidEndpoint);
        }
        if number == 0 {
            if !matches!(ep_type, EndpointType::Control) {
                return Err(UsbError::InvalidState);
            }
        } else if matches!(ep_type, EndpointType::Control) {
            return Err(UsbError::InvalidEndpoint);
        }

        let flag = match dir {
            Direction::In => DIR_IN,
            Direction::Out => DIR_OUT,
        };
        if self.ep_alloc.get() & ep_bit(number, flag) != 0 {
            return Err(UsbError::InvalidEndpoint);
        }
        Self::set_bit(&self.ep_alloc, ep_bit(number, flag), true);
        self.ep_max[number] = max_packet as u8;

        Ok(EndpointAddress::new(number as u8, dir))
    }

    fn enable(&mut self) {
        self.ep_reset();

        // Attach: enable the internal D+ pull-up and the SIE.
        self.regs
            .base_ctrl
            .write_bits(UC_DEV_PU_EN | UC_INT_BUSY | UC_DMA_EN);
        self.regs.int_fg.write_bits(0xff);
        self.regs
            .host_ctrl_udev_ctrl
            .write_bits(UD_PD_DIS | UD_PORT_EN);
        self.regs
            .int_en
            .write_bits(UIE_SUSPEND | UIE_TRANSFER | UIE_BUS_RST);
    }

    fn reset(&self) {
        self.ep_reset();
    }

    fn set_device_address(&self, addr: u8) {
        self.regs.dev_addr.write_bits((addr & 0x7f) as u32);
    }

    fn poll(&self) -> PollResult {
        let flags = self.regs.int_fg.read_bits();

        if flags & UIF_BUS_RST != 0 {
            self.regs.int_fg.write_bits(UIF_BUS_RST);
            self.ep_reset();
            return PollResult::Reset;
        }
        if flags & UIF_SUSPEND != 0 {
            self.regs.int_fg.write_bits(UIF_SUSPEND);
            return if self.regs.mis_st.read_bits() & UMS_SUSPEND != 0 {
                PollResult::Suspend
            } else {
                PollResult::Resume
            };
        }

        let mut ep_out = 0u16;
        let mut ep_in_complete = 0u16;
        let mut ep_setup = 0u16;

        loop {
            let flags = self.regs.int_fg.read_bits();
            if flags & UIF_FIFO_OV != 0 {
                self.regs.int_fg.write_bits(UIF_FIFO_OV);
            }
            if flags & UIF_TRANSFER == 0 {
                break;
            }

            let status = self.regs.int_st.read_bits();
            let number = (status & UIS_ENDP_MASK) as usize;
            match status & UIS_TOKEN_MASK {
                UIS_TOKEN_SETUP => {
                    // The WCH SIE does not report a meaningful endpoint number
                    // for SETUP transactions; they always target EP0.
                    self.handle_setup();
                    ep_setup |= 1;
                }
                UIS_TOKEN_IN => {
                    if number < NUM_EP && (self.is_in_used(number) || number == 0) {
                        self.handle_in(number);
                        ep_in_complete |= 1 << number;
                    }
                }
                UIS_TOKEN_OUT => {
                    if number < NUM_EP
                        && status & UIS_TOG_OK != 0
                        && (self.is_out_used(number) || number == 0)
                    {
                        self.handle_out(number);
                        if number != 0 || self.ctrl_ready.get() {
                            ep_out |= 1 << number;
                        }
                    }
                }
                UIS_TOKEN_SOF => {}
                _ => {}
            }

            // Write-one-to-clear.
            self.regs.int_fg.write_bits(UIF_TRANSFER);
        }

        if ep_out | ep_in_complete | ep_setup != 0 {
            PollResult::Data {
                ep_out,
                ep_in_complete,
                ep_setup,
            }
        } else {
            PollResult::None
        }
    }

    fn write(&self, addr: EndpointAddress, buf: &[u8]) -> UsbResult<usize> {
        let n = addr.number() as usize;
        if n >= NUM_EP {
            return Err(UsbError::InvalidEndpoint);
        }
        if !addr.is_in() {
            return Err(UsbError::InvalidEndpoint);
        }
        if n == 0 {
            self.write_ep0(buf)
        } else if self.is_in_used(n) {
            self.write_ep(n, buf)
        } else {
            Err(UsbError::InvalidEndpoint)
        }
    }

    fn read(&self, addr: EndpointAddress, buf: &mut [u8]) -> UsbResult<usize> {
        let n = addr.number() as usize;
        if n >= NUM_EP {
            return Err(UsbError::InvalidEndpoint);
        }
        if !addr.is_out() {
            return Err(UsbError::InvalidEndpoint);
        }
        if n == 0 {
            self.read_ep0(buf)
        } else if self.is_out_used(n) {
            self.read_ep(n, buf)
        } else {
            Err(UsbError::InvalidEndpoint)
        }
    }

    fn set_stalled(&self, addr: EndpointAddress, stalled: bool) {
        let n = addr.number() as usize;
        if n >= NUM_EP {
            return;
        }
        let dir = if addr.is_in() { DIR_IN } else { DIR_OUT };
        Self::set_bit(&self.ep_stalled, ep_bit(n, dir), stalled);

        if n == 0 {
            // The bus owns EP0 arming; only drive the STALL response here.
            if stalled {
                self.set_resp(0, UEP_T_RES_STALL, UEP_R_RES_STALL);
            }
            return;
        }

        if addr.is_in() {
            let value = self.ctrl_read(n);
            let value = if stalled {
                (value & !UEP_T_RES_MASK) | UEP_T_RES_STALL
            } else {
                // Clearing a halt resets the data toggle (USB 2.0 §9.4.5).
                (value & !(UEP_T_RES_MASK | UEP_T_TOG)) | UEP_T_RES_NAK
            };
            self.write_ctrl(n, value);
        } else {
            let value = self.ctrl_read(n);
            let value = if stalled {
                (value & !UEP_R_RES_MASK) | UEP_R_RES_STALL
            } else {
                (value & !(UEP_R_RES_MASK | UEP_R_TOG)) | UEP_R_RES_ACK
            };
            self.write_ctrl(n, value);
        }
    }

    fn is_stalled(&self, addr: EndpointAddress) -> bool {
        let n = addr.number() as usize;
        if n >= NUM_EP {
            return false;
        }
        let dir = if addr.is_in() { DIR_IN } else { DIR_OUT };
        self.ep_stalled.get() & ep_bit(n, dir) != 0
    }

    fn suspend(&self) {
        // The SIE suspends automatically; nothing to do.
    }

    fn resume(&self) {
        // The SIE resumes automatically; nothing to do.
    }
}
