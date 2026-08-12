//! USBFS
//! USB register

#![allow(dead_code)]

pub const BASE: u32 = 0x40023400;

/// BASE_CTRL Register
///
/// USB base control
///
/// Address offset: 0x0000
/// Reset value: 0x00000006
///
/// ## Fields
///
/// - **RB_UC_HOST_MODE** `[7]` (RW)
///
///   enable USB host mode: 0=device mode, 1=host mode
///
/// - **RB_UC_LOW_SPEED** `[6]` (RW)
///
///   RB_UC_LOW_SPEED
///
/// - **RB_SYS_MODE** `[5:4]` (RW)
///
///   RB_SYS_MODE
///
/// - **RB_UC_INT_BUSY** `[3]` (RW)
///
///   enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid
///
/// - **RB_UC_RST_SIE** `[2]` (RW)
///
///   force reset USB SIE, need software clear
///
/// - **RB_UC_CLR_ALL** `[1]` (RW)
///
///   force clear FIFO and count of USB
///
/// - **RB_UC_DMA_EN** `[0]` (RW)
///
///   DMA enable and DMA interrupt enable for USB
#[repr(transparent)]
pub struct BaseCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BaseCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BaseCtrl {
    // ======== Field Constants ========

    /// **RB_UC_HOST_MODE**
    ///
    /// enable USB host mode: 0=device mode, 1=host mode
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const RB_UC_HOST_MODE_OFFSET: u32 = 7;
    pub const RB_UC_HOST_MODE_MASK: u32 = 0x00000080;
    pub const RB_UC_HOST_MODE: u32 = 1 << 7;

    /// **RB_UC_LOW_SPEED**
    ///
    /// RB_UC_LOW_SPEED
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const RB_UC_LOW_SPEED_OFFSET: u32 = 6;
    pub const RB_UC_LOW_SPEED_MASK: u32 = 0x00000040;
    pub const RB_UC_LOW_SPEED: u32 = 1 << 6;

    /// **RB_SYS_MODE**
    ///
    /// RB_SYS_MODE
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const RB_SYS_MODE_OFFSET: u32 = 4;
    pub const RB_SYS_MODE_MASK: u32 = 0x00000030;

    /// **RB_UC_INT_BUSY**
    ///
    /// enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const RB_UC_INT_BUSY_OFFSET: u32 = 3;
    pub const RB_UC_INT_BUSY_MASK: u32 = 0x00000008;
    pub const RB_UC_INT_BUSY: u32 = 1 << 3;

    /// **RB_UC_RST_SIE**
    ///
    /// force reset USB SIE, need software clear
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RB_UC_RST_SIE_OFFSET: u32 = 2;
    pub const RB_UC_RST_SIE_MASK: u32 = 0x00000004;
    pub const RB_UC_RST_SIE: u32 = 1 << 2;

    /// **RB_UC_CLR_ALL**
    ///
    /// force clear FIFO and count of USB
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RB_UC_CLR_ALL_OFFSET: u32 = 1;
    pub const RB_UC_CLR_ALL_MASK: u32 = 0x00000002;
    pub const RB_UC_CLR_ALL: u32 = 1 << 1;

    /// **RB_UC_DMA_EN**
    ///
    /// DMA enable and DMA interrupt enable for USB
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RB_UC_DMA_EN_OFFSET: u32 = 0;
    pub const RB_UC_DMA_EN_MASK: u32 = 0x00000001;
    pub const RB_UC_DMA_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RB_UC_HOST_MODE** `[7]` (RW)
    ///
    /// enable USB host mode: 0=device mode, 1=host mode
    #[inline(always)]
    pub fn rb_uc_host_mode(&self) -> bool {
        (self.value.read_bits() & Self::RB_UC_HOST_MODE_MASK) != 0
    }

    /// Set RB_UC_HOST_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rb_uc_host_mode(&self) {
        self.value.modify_bits(|r| r | Self::RB_UC_HOST_MODE_MASK);
    }

    /// Clear RB_UC_HOST_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rb_uc_host_mode(&self) {
        self.value.modify_bits(|r| r & !Self::RB_UC_HOST_MODE_MASK);
    }

    /// **RB_UC_LOW_SPEED** `[6]` (RW)
    ///
    /// RB_UC_LOW_SPEED
    #[inline(always)]
    pub fn rb_uc_low_speed(&self) -> bool {
        (self.value.read_bits() & Self::RB_UC_LOW_SPEED_MASK) != 0
    }

    /// Set RB_UC_LOW_SPEED bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rb_uc_low_speed(&self) {
        self.value.modify_bits(|r| r | Self::RB_UC_LOW_SPEED_MASK);
    }

    /// Clear RB_UC_LOW_SPEED bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rb_uc_low_speed(&self) {
        self.value.modify_bits(|r| r & !Self::RB_UC_LOW_SPEED_MASK);
    }

    /// **RB_SYS_MODE** `[5:4]` (RW)
    ///
    /// RB_SYS_MODE
    #[inline(always)]
    pub fn rb_sys_mode(&self) -> u32 {
        (self.value.read_bits() & Self::RB_SYS_MODE_MASK) >> Self::RB_SYS_MODE_OFFSET
    }

    /// Write **RB_SYS_MODE** `[5:4]` (RW)
    ///
    /// RB_SYS_MODE
    #[inline(always)]
    pub fn write_rb_sys_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RB_SYS_MODE_MASK)
                | ((value << Self::RB_SYS_MODE_OFFSET) & Self::RB_SYS_MODE_MASK)
        });
    }

    /// **RB_UC_INT_BUSY** `[3]` (RW)
    ///
    /// enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid
    #[inline(always)]
    pub fn rb_uc_int_busy(&self) -> bool {
        (self.value.read_bits() & Self::RB_UC_INT_BUSY_MASK) != 0
    }

    /// Set RB_UC_INT_BUSY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rb_uc_int_busy(&self) {
        self.value.modify_bits(|r| r | Self::RB_UC_INT_BUSY_MASK);
    }

    /// Clear RB_UC_INT_BUSY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rb_uc_int_busy(&self) {
        self.value.modify_bits(|r| r & !Self::RB_UC_INT_BUSY_MASK);
    }

    /// **RB_UC_RST_SIE** `[2]` (RW)
    ///
    /// force reset USB SIE, need software clear
    #[inline(always)]
    pub fn rb_uc_rst_sie(&self) -> bool {
        (self.value.read_bits() & Self::RB_UC_RST_SIE_MASK) != 0
    }

    /// Set RB_UC_RST_SIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rb_uc_rst_sie(&self) {
        self.value.modify_bits(|r| r | Self::RB_UC_RST_SIE_MASK);
    }

    /// Clear RB_UC_RST_SIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rb_uc_rst_sie(&self) {
        self.value.modify_bits(|r| r & !Self::RB_UC_RST_SIE_MASK);
    }

    /// **RB_UC_CLR_ALL** `[1]` (RW)
    ///
    /// force clear FIFO and count of USB
    #[inline(always)]
    pub fn rb_uc_clr_all(&self) -> bool {
        (self.value.read_bits() & Self::RB_UC_CLR_ALL_MASK) != 0
    }

    /// Set RB_UC_CLR_ALL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rb_uc_clr_all(&self) {
        self.value.modify_bits(|r| r | Self::RB_UC_CLR_ALL_MASK);
    }

    /// Clear RB_UC_CLR_ALL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rb_uc_clr_all(&self) {
        self.value.modify_bits(|r| r & !Self::RB_UC_CLR_ALL_MASK);
    }

    /// **RB_UC_DMA_EN** `[0]` (RW)
    ///
    /// DMA enable and DMA interrupt enable for USB
    #[inline(always)]
    pub fn rb_uc_dma_en(&self) -> bool {
        (self.value.read_bits() & Self::RB_UC_DMA_EN_MASK) != 0
    }

    /// Set RB_UC_DMA_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rb_uc_dma_en(&self) {
        self.value.modify_bits(|r| r | Self::RB_UC_DMA_EN_MASK);
    }

    /// Clear RB_UC_DMA_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rb_uc_dma_en(&self) {
        self.value.modify_bits(|r| r & !Self::RB_UC_DMA_EN_MASK);
    }
}

/// BASE_CTRL value type for batch read/modify/write operations
///
/// Use `basectrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `basectrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BaseCtrlVal {
    bits: u32,
}

impl BaseCtrlVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000006 }
    }

    /// Read **RB_UC_HOST_MODE** `[7:7]`
    ///
    /// enable USB host mode: 0=device mode, 1=host mode
    #[inline(always)]
    pub const fn rb_uc_host_mode(&self) -> bool {
        (self.bits & BaseCtrl::RB_UC_HOST_MODE_MASK) != 0
    }

    /// Read **RB_UC_LOW_SPEED** `[6:6]`
    ///
    /// RB_UC_LOW_SPEED
    #[inline(always)]
    pub const fn rb_uc_low_speed(&self) -> bool {
        (self.bits & BaseCtrl::RB_UC_LOW_SPEED_MASK) != 0
    }

    /// Read **RB_SYS_MODE** `[5:4]`
    ///
    /// RB_SYS_MODE
    #[inline(always)]
    pub const fn rb_sys_mode(&self) -> u32 {
        (self.bits & BaseCtrl::RB_SYS_MODE_MASK) >> BaseCtrl::RB_SYS_MODE_OFFSET
    }

    /// Read **RB_UC_INT_BUSY** `[3:3]`
    ///
    /// enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid
    #[inline(always)]
    pub const fn rb_uc_int_busy(&self) -> bool {
        (self.bits & BaseCtrl::RB_UC_INT_BUSY_MASK) != 0
    }

    /// Read **RB_UC_RST_SIE** `[2:2]`
    ///
    /// force reset USB SIE, need software clear
    #[inline(always)]
    pub const fn rb_uc_rst_sie(&self) -> bool {
        (self.bits & BaseCtrl::RB_UC_RST_SIE_MASK) != 0
    }

    /// Read **RB_UC_CLR_ALL** `[1:1]`
    ///
    /// force clear FIFO and count of USB
    #[inline(always)]
    pub const fn rb_uc_clr_all(&self) -> bool {
        (self.bits & BaseCtrl::RB_UC_CLR_ALL_MASK) != 0
    }

    /// Read **RB_UC_DMA_EN** `[0:0]`
    ///
    /// DMA enable and DMA interrupt enable for USB
    #[inline(always)]
    pub const fn rb_uc_dma_en(&self) -> bool {
        (self.bits & BaseCtrl::RB_UC_DMA_EN_MASK) != 0
    }

    /// Set **RB_UC_HOST_MODE** `[7:7]`
    ///
    /// enable USB host mode: 0=device mode, 1=host mode
    #[inline(always)]
    pub const fn with_rb_uc_host_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BaseCtrl::RB_UC_HOST_MODE_MASK
            } else {
                self.bits & !BaseCtrl::RB_UC_HOST_MODE_MASK
            },
        }
    }

    /// Set **RB_UC_LOW_SPEED** `[6:6]`
    ///
    /// RB_UC_LOW_SPEED
    #[inline(always)]
    pub const fn with_rb_uc_low_speed(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BaseCtrl::RB_UC_LOW_SPEED_MASK
            } else {
                self.bits & !BaseCtrl::RB_UC_LOW_SPEED_MASK
            },
        }
    }

    /// Set **RB_SYS_MODE** `[5:4]`
    ///
    /// RB_SYS_MODE
    #[inline(always)]
    pub const fn with_rb_sys_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BaseCtrl::RB_SYS_MODE_MASK)
                | ((value << BaseCtrl::RB_SYS_MODE_OFFSET) & BaseCtrl::RB_SYS_MODE_MASK),
        }
    }

    /// Set **RB_UC_INT_BUSY** `[3:3]`
    ///
    /// enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid
    #[inline(always)]
    pub const fn with_rb_uc_int_busy(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BaseCtrl::RB_UC_INT_BUSY_MASK
            } else {
                self.bits & !BaseCtrl::RB_UC_INT_BUSY_MASK
            },
        }
    }

    /// Set **RB_UC_RST_SIE** `[2:2]`
    ///
    /// force reset USB SIE, need software clear
    #[inline(always)]
    pub const fn with_rb_uc_rst_sie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BaseCtrl::RB_UC_RST_SIE_MASK
            } else {
                self.bits & !BaseCtrl::RB_UC_RST_SIE_MASK
            },
        }
    }

    /// Set **RB_UC_CLR_ALL** `[1:1]`
    ///
    /// force clear FIFO and count of USB
    #[inline(always)]
    pub const fn with_rb_uc_clr_all(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BaseCtrl::RB_UC_CLR_ALL_MASK
            } else {
                self.bits & !BaseCtrl::RB_UC_CLR_ALL_MASK
            },
        }
    }

    /// Set **RB_UC_DMA_EN** `[0:0]`
    ///
    /// DMA enable and DMA interrupt enable for USB
    #[inline(always)]
    pub const fn with_rb_uc_dma_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BaseCtrl::RB_UC_DMA_EN_MASK
            } else {
                self.bits & !BaseCtrl::RB_UC_DMA_EN_MASK
            },
        }
    }
}

impl Default for BaseCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl BaseCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BaseCtrlVal {
        BaseCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BaseCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BaseCtrlVal) -> BaseCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// HOST_CTRL_UDEV_CTRL Register
///
/// USB HOST control_UDEV_CTRL
///
/// Address offset: 0x0001
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RB_UH_PD_DIS** `[7]` (RW)
///
///   RB_UH_PD_DIS
///
/// - **RB_UH_DP_PIN** `[5]` (RO)
///
///   RB_UH_DP_PIN
///
/// - **RB_UH_DM_PIN** `[4]` (RO)
///
///   RB_UH_DM_PIN
///
/// - **RB_UH_LOW_SPEED** `[2]` (RW)
///
///   RB_UH_LOW_SPEED
///
/// - **RB_UH_BUS_RESET** `[1]` (RW)
///
///   RB_UH_BUS_RESET
///
/// - **RB_UH_PORT_EN** `[0]` (RW)
///
///   RB_UH_PORT_EN
#[repr(transparent)]
pub struct HostCtrlUdevCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HostCtrlUdevCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HostCtrlUdevCtrl {
    // ======== Field Constants ========

    /// **RB_UH_PD_DIS**
    ///
    /// RB_UH_PD_DIS
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const RB_UH_PD_DIS_OFFSET: u32 = 7;
    pub const RB_UH_PD_DIS_MASK: u32 = 0x00000080;
    pub const RB_UH_PD_DIS: u32 = 1 << 7;

    /// **RB_UH_DP_PIN**
    ///
    /// RB_UH_DP_PIN
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const RB_UH_DP_PIN_OFFSET: u32 = 5;
    pub const RB_UH_DP_PIN_MASK: u32 = 0x00000020;
    pub const RB_UH_DP_PIN: u32 = 1 << 5;

    /// **RB_UH_DM_PIN**
    ///
    /// RB_UH_DM_PIN
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RB_UH_DM_PIN_OFFSET: u32 = 4;
    pub const RB_UH_DM_PIN_MASK: u32 = 0x00000010;
    pub const RB_UH_DM_PIN: u32 = 1 << 4;

    /// **RB_UH_LOW_SPEED**
    ///
    /// RB_UH_LOW_SPEED
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RB_UH_LOW_SPEED_OFFSET: u32 = 2;
    pub const RB_UH_LOW_SPEED_MASK: u32 = 0x00000004;
    pub const RB_UH_LOW_SPEED: u32 = 1 << 2;

    /// **RB_UH_BUS_RESET**
    ///
    /// RB_UH_BUS_RESET
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RB_UH_BUS_RESET_OFFSET: u32 = 1;
    pub const RB_UH_BUS_RESET_MASK: u32 = 0x00000002;
    pub const RB_UH_BUS_RESET: u32 = 1 << 1;

    /// **RB_UH_PORT_EN**
    ///
    /// RB_UH_PORT_EN
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RB_UH_PORT_EN_OFFSET: u32 = 0;
    pub const RB_UH_PORT_EN_MASK: u32 = 0x00000001;
    pub const RB_UH_PORT_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RB_UH_PD_DIS** `[7]` (RW)
    ///
    /// RB_UH_PD_DIS
    #[inline(always)]
    pub fn rb_uh_pd_dis(&self) -> bool {
        (self.value.read_bits() & Self::RB_UH_PD_DIS_MASK) != 0
    }

    /// Set RB_UH_PD_DIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rb_uh_pd_dis(&self) {
        self.value.modify_bits(|r| r | Self::RB_UH_PD_DIS_MASK);
    }

    /// Clear RB_UH_PD_DIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rb_uh_pd_dis(&self) {
        self.value.modify_bits(|r| r & !Self::RB_UH_PD_DIS_MASK);
    }

    /// **RB_UH_DP_PIN** `[5]` (RO)
    ///
    /// RB_UH_DP_PIN
    #[inline(always)]
    pub fn rb_uh_dp_pin(&self) -> bool {
        (self.value.read_bits() & Self::RB_UH_DP_PIN_MASK) != 0
    }

    /// **RB_UH_DM_PIN** `[4]` (RO)
    ///
    /// RB_UH_DM_PIN
    #[inline(always)]
    pub fn rb_uh_dm_pin(&self) -> bool {
        (self.value.read_bits() & Self::RB_UH_DM_PIN_MASK) != 0
    }

    /// **RB_UH_LOW_SPEED** `[2]` (RW)
    ///
    /// RB_UH_LOW_SPEED
    #[inline(always)]
    pub fn rb_uh_low_speed(&self) -> bool {
        (self.value.read_bits() & Self::RB_UH_LOW_SPEED_MASK) != 0
    }

    /// Set RB_UH_LOW_SPEED bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rb_uh_low_speed(&self) {
        self.value.modify_bits(|r| r | Self::RB_UH_LOW_SPEED_MASK);
    }

    /// Clear RB_UH_LOW_SPEED bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rb_uh_low_speed(&self) {
        self.value.modify_bits(|r| r & !Self::RB_UH_LOW_SPEED_MASK);
    }

    /// **RB_UH_BUS_RESET** `[1]` (RW)
    ///
    /// RB_UH_BUS_RESET
    #[inline(always)]
    pub fn rb_uh_bus_reset(&self) -> bool {
        (self.value.read_bits() & Self::RB_UH_BUS_RESET_MASK) != 0
    }

    /// Set RB_UH_BUS_RESET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rb_uh_bus_reset(&self) {
        self.value.modify_bits(|r| r | Self::RB_UH_BUS_RESET_MASK);
    }

    /// Clear RB_UH_BUS_RESET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rb_uh_bus_reset(&self) {
        self.value.modify_bits(|r| r & !Self::RB_UH_BUS_RESET_MASK);
    }

    /// **RB_UH_PORT_EN** `[0]` (RW)
    ///
    /// RB_UH_PORT_EN
    #[inline(always)]
    pub fn rb_uh_port_en(&self) -> bool {
        (self.value.read_bits() & Self::RB_UH_PORT_EN_MASK) != 0
    }

    /// Set RB_UH_PORT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rb_uh_port_en(&self) {
        self.value.modify_bits(|r| r | Self::RB_UH_PORT_EN_MASK);
    }

    /// Clear RB_UH_PORT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rb_uh_port_en(&self) {
        self.value.modify_bits(|r| r & !Self::RB_UH_PORT_EN_MASK);
    }
}

/// HOST_CTRL_UDEV_CTRL value type for batch read/modify/write operations
///
/// Use `hostctrludevctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `hostctrludevctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct HostCtrlUdevCtrlVal {
    bits: u32,
}

impl HostCtrlUdevCtrlVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **RB_UH_PD_DIS** `[7:7]`
    ///
    /// RB_UH_PD_DIS
    #[inline(always)]
    pub const fn rb_uh_pd_dis(&self) -> bool {
        (self.bits & HostCtrlUdevCtrl::RB_UH_PD_DIS_MASK) != 0
    }

    /// Read **RB_UH_DP_PIN** `[5:5]`
    ///
    /// RB_UH_DP_PIN
    #[inline(always)]
    pub const fn rb_uh_dp_pin(&self) -> bool {
        (self.bits & HostCtrlUdevCtrl::RB_UH_DP_PIN_MASK) != 0
    }

    /// Read **RB_UH_DM_PIN** `[4:4]`
    ///
    /// RB_UH_DM_PIN
    #[inline(always)]
    pub const fn rb_uh_dm_pin(&self) -> bool {
        (self.bits & HostCtrlUdevCtrl::RB_UH_DM_PIN_MASK) != 0
    }

    /// Read **RB_UH_LOW_SPEED** `[2:2]`
    ///
    /// RB_UH_LOW_SPEED
    #[inline(always)]
    pub const fn rb_uh_low_speed(&self) -> bool {
        (self.bits & HostCtrlUdevCtrl::RB_UH_LOW_SPEED_MASK) != 0
    }

    /// Read **RB_UH_BUS_RESET** `[1:1]`
    ///
    /// RB_UH_BUS_RESET
    #[inline(always)]
    pub const fn rb_uh_bus_reset(&self) -> bool {
        (self.bits & HostCtrlUdevCtrl::RB_UH_BUS_RESET_MASK) != 0
    }

    /// Read **RB_UH_PORT_EN** `[0:0]`
    ///
    /// RB_UH_PORT_EN
    #[inline(always)]
    pub const fn rb_uh_port_en(&self) -> bool {
        (self.bits & HostCtrlUdevCtrl::RB_UH_PORT_EN_MASK) != 0
    }

    /// Set **RB_UH_PD_DIS** `[7:7]`
    ///
    /// RB_UH_PD_DIS
    #[inline(always)]
    pub const fn with_rb_uh_pd_dis(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | HostCtrlUdevCtrl::RB_UH_PD_DIS_MASK
            } else {
                self.bits & !HostCtrlUdevCtrl::RB_UH_PD_DIS_MASK
            },
        }
    }

    /// Set **RB_UH_LOW_SPEED** `[2:2]`
    ///
    /// RB_UH_LOW_SPEED
    #[inline(always)]
    pub const fn with_rb_uh_low_speed(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | HostCtrlUdevCtrl::RB_UH_LOW_SPEED_MASK
            } else {
                self.bits & !HostCtrlUdevCtrl::RB_UH_LOW_SPEED_MASK
            },
        }
    }

    /// Set **RB_UH_BUS_RESET** `[1:1]`
    ///
    /// RB_UH_BUS_RESET
    #[inline(always)]
    pub const fn with_rb_uh_bus_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | HostCtrlUdevCtrl::RB_UH_BUS_RESET_MASK
            } else {
                self.bits & !HostCtrlUdevCtrl::RB_UH_BUS_RESET_MASK
            },
        }
    }

    /// Set **RB_UH_PORT_EN** `[0:0]`
    ///
    /// RB_UH_PORT_EN
    #[inline(always)]
    pub const fn with_rb_uh_port_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | HostCtrlUdevCtrl::RB_UH_PORT_EN_MASK
            } else {
                self.bits & !HostCtrlUdevCtrl::RB_UH_PORT_EN_MASK
            },
        }
    }
}

impl Default for HostCtrlUdevCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl HostCtrlUdevCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> HostCtrlUdevCtrlVal {
        HostCtrlUdevCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: HostCtrlUdevCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(HostCtrlUdevCtrlVal) -> HostCtrlUdevCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_EN Register
///
/// USB interrupt enable
///
/// Address offset: 0x0002
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RB_UIE_DEV_SOF** `[7]` (RW)
///
///   RB_UIE_DEV_SOF
///
/// - **RB_UIE_DEV_NAK** `[6]` (RW)
///
///   RB_UIE_DEV_NAK
///
/// - **RB_UIE_FIFO_OV** `[4]` (RW)
///
///   enable interrupt for FIFO overflow
///
/// - **RB_UIE_SOF_ACT** `[3]` (RW)
///
///   indicate host SOF timer action status for USB host
///
/// - **RB_UIE_SUSPEND** `[2]` (RW)
///
///   enable interrupt for USB suspend or resume event
///
/// - **RB_UIE_TRANSFER** `[1]` (RW)
///
///   enable interrupt for USB transfer completion
///
/// - **RB_UIE_BUS_RST__UIE_DETECT** `[0]` (RW)
///
///   enable interrupt for USB bus reset event for USB device mode;enable interrupt for USB device detected event for USB host mode
#[repr(transparent)]
pub struct IntEn {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntEn {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntEn {
    // ======== Field Constants ========

    /// **RB_UIE_DEV_SOF**
    ///
    /// RB_UIE_DEV_SOF
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const RB_UIE_DEV_SOF_OFFSET: u32 = 7;
    pub const RB_UIE_DEV_SOF_MASK: u32 = 0x00000080;
    pub const RB_UIE_DEV_SOF: u32 = 1 << 7;

    /// **RB_UIE_DEV_NAK**
    ///
    /// RB_UIE_DEV_NAK
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const RB_UIE_DEV_NAK_OFFSET: u32 = 6;
    pub const RB_UIE_DEV_NAK_MASK: u32 = 0x00000040;
    pub const RB_UIE_DEV_NAK: u32 = 1 << 6;

    /// **RB_UIE_FIFO_OV**
    ///
    /// enable interrupt for FIFO overflow
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RB_UIE_FIFO_OV_OFFSET: u32 = 4;
    pub const RB_UIE_FIFO_OV_MASK: u32 = 0x00000010;
    pub const RB_UIE_FIFO_OV: u32 = 1 << 4;

    /// **RB_UIE_SOF_ACT**
    ///
    /// indicate host SOF timer action status for USB host
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const RB_UIE_SOF_ACT_OFFSET: u32 = 3;
    pub const RB_UIE_SOF_ACT_MASK: u32 = 0x00000008;
    pub const RB_UIE_SOF_ACT: u32 = 1 << 3;

    /// **RB_UIE_SUSPEND**
    ///
    /// enable interrupt for USB suspend or resume event
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RB_UIE_SUSPEND_OFFSET: u32 = 2;
    pub const RB_UIE_SUSPEND_MASK: u32 = 0x00000004;
    pub const RB_UIE_SUSPEND: u32 = 1 << 2;

    /// **RB_UIE_TRANSFER**
    ///
    /// enable interrupt for USB transfer completion
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RB_UIE_TRANSFER_OFFSET: u32 = 1;
    pub const RB_UIE_TRANSFER_MASK: u32 = 0x00000002;
    pub const RB_UIE_TRANSFER: u32 = 1 << 1;

    /// **RB_UIE_BUS_RST__UIE_DETECT**
    ///
    /// enable interrupt for USB bus reset event for USB device mode;enable interrupt for USB device detected event for USB host mode
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RB_UIE_BUS_RST_UIE_DETECT_OFFSET: u32 = 0;
    pub const RB_UIE_BUS_RST_UIE_DETECT_MASK: u32 = 0x00000001;
    pub const RB_UIE_BUS_RST_UIE_DETECT: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RB_UIE_DEV_SOF** `[7]` (RW)
    ///
    /// RB_UIE_DEV_SOF
    #[inline(always)]
    pub fn rb_uie_dev_sof(&self) -> bool {
        (self.value.read_bits() & Self::RB_UIE_DEV_SOF_MASK) != 0
    }

    /// Set RB_UIE_DEV_SOF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rb_uie_dev_sof(&self) {
        self.value.modify_bits(|r| r | Self::RB_UIE_DEV_SOF_MASK);
    }

    /// Clear RB_UIE_DEV_SOF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rb_uie_dev_sof(&self) {
        self.value.modify_bits(|r| r & !Self::RB_UIE_DEV_SOF_MASK);
    }

    /// **RB_UIE_DEV_NAK** `[6]` (RW)
    ///
    /// RB_UIE_DEV_NAK
    #[inline(always)]
    pub fn rb_uie_dev_nak(&self) -> bool {
        (self.value.read_bits() & Self::RB_UIE_DEV_NAK_MASK) != 0
    }

    /// Set RB_UIE_DEV_NAK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rb_uie_dev_nak(&self) {
        self.value.modify_bits(|r| r | Self::RB_UIE_DEV_NAK_MASK);
    }

    /// Clear RB_UIE_DEV_NAK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rb_uie_dev_nak(&self) {
        self.value.modify_bits(|r| r & !Self::RB_UIE_DEV_NAK_MASK);
    }

    /// **RB_UIE_FIFO_OV** `[4]` (RW)
    ///
    /// enable interrupt for FIFO overflow
    #[inline(always)]
    pub fn rb_uie_fifo_ov(&self) -> bool {
        (self.value.read_bits() & Self::RB_UIE_FIFO_OV_MASK) != 0
    }

    /// Set RB_UIE_FIFO_OV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rb_uie_fifo_ov(&self) {
        self.value.modify_bits(|r| r | Self::RB_UIE_FIFO_OV_MASK);
    }

    /// Clear RB_UIE_FIFO_OV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rb_uie_fifo_ov(&self) {
        self.value.modify_bits(|r| r & !Self::RB_UIE_FIFO_OV_MASK);
    }

    /// **RB_UIE_SOF_ACT** `[3]` (RW)
    ///
    /// indicate host SOF timer action status for USB host
    #[inline(always)]
    pub fn rb_uie_sof_act(&self) -> bool {
        (self.value.read_bits() & Self::RB_UIE_SOF_ACT_MASK) != 0
    }

    /// Set RB_UIE_SOF_ACT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rb_uie_sof_act(&self) {
        self.value.modify_bits(|r| r | Self::RB_UIE_SOF_ACT_MASK);
    }

    /// Clear RB_UIE_SOF_ACT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rb_uie_sof_act(&self) {
        self.value.modify_bits(|r| r & !Self::RB_UIE_SOF_ACT_MASK);
    }

    /// **RB_UIE_SUSPEND** `[2]` (RW)
    ///
    /// enable interrupt for USB suspend or resume event
    #[inline(always)]
    pub fn rb_uie_suspend(&self) -> bool {
        (self.value.read_bits() & Self::RB_UIE_SUSPEND_MASK) != 0
    }

    /// Set RB_UIE_SUSPEND bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rb_uie_suspend(&self) {
        self.value.modify_bits(|r| r | Self::RB_UIE_SUSPEND_MASK);
    }

    /// Clear RB_UIE_SUSPEND bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rb_uie_suspend(&self) {
        self.value.modify_bits(|r| r & !Self::RB_UIE_SUSPEND_MASK);
    }

    /// **RB_UIE_TRANSFER** `[1]` (RW)
    ///
    /// enable interrupt for USB transfer completion
    #[inline(always)]
    pub fn rb_uie_transfer(&self) -> bool {
        (self.value.read_bits() & Self::RB_UIE_TRANSFER_MASK) != 0
    }

    /// Set RB_UIE_TRANSFER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rb_uie_transfer(&self) {
        self.value.modify_bits(|r| r | Self::RB_UIE_TRANSFER_MASK);
    }

    /// Clear RB_UIE_TRANSFER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rb_uie_transfer(&self) {
        self.value.modify_bits(|r| r & !Self::RB_UIE_TRANSFER_MASK);
    }

    /// **RB_UIE_BUS_RST__UIE_DETECT** `[0]` (RW)
    ///
    /// enable interrupt for USB bus reset event for USB device mode;enable interrupt for USB device detected event for USB host mode
    #[inline(always)]
    pub fn rb_uie_bus_rst_uie_detect(&self) -> bool {
        (self.value.read_bits() & Self::RB_UIE_BUS_RST_UIE_DETECT_MASK) != 0
    }

    /// Set RB_UIE_BUS_RST__UIE_DETECT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rb_uie_bus_rst_uie_detect(&self) {
        self.value
            .modify_bits(|r| r | Self::RB_UIE_BUS_RST_UIE_DETECT_MASK);
    }

    /// Clear RB_UIE_BUS_RST__UIE_DETECT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rb_uie_bus_rst_uie_detect(&self) {
        self.value
            .modify_bits(|r| r & !Self::RB_UIE_BUS_RST_UIE_DETECT_MASK);
    }
}

/// INT_EN value type for batch read/modify/write operations
///
/// Use `inten.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inten.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntEnVal {
    bits: u32,
}

impl IntEnVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **RB_UIE_DEV_SOF** `[7:7]`
    ///
    /// RB_UIE_DEV_SOF
    #[inline(always)]
    pub const fn rb_uie_dev_sof(&self) -> bool {
        (self.bits & IntEn::RB_UIE_DEV_SOF_MASK) != 0
    }

    /// Read **RB_UIE_DEV_NAK** `[6:6]`
    ///
    /// RB_UIE_DEV_NAK
    #[inline(always)]
    pub const fn rb_uie_dev_nak(&self) -> bool {
        (self.bits & IntEn::RB_UIE_DEV_NAK_MASK) != 0
    }

    /// Read **RB_UIE_FIFO_OV** `[4:4]`
    ///
    /// enable interrupt for FIFO overflow
    #[inline(always)]
    pub const fn rb_uie_fifo_ov(&self) -> bool {
        (self.bits & IntEn::RB_UIE_FIFO_OV_MASK) != 0
    }

    /// Read **RB_UIE_SOF_ACT** `[3:3]`
    ///
    /// indicate host SOF timer action status for USB host
    #[inline(always)]
    pub const fn rb_uie_sof_act(&self) -> bool {
        (self.bits & IntEn::RB_UIE_SOF_ACT_MASK) != 0
    }

    /// Read **RB_UIE_SUSPEND** `[2:2]`
    ///
    /// enable interrupt for USB suspend or resume event
    #[inline(always)]
    pub const fn rb_uie_suspend(&self) -> bool {
        (self.bits & IntEn::RB_UIE_SUSPEND_MASK) != 0
    }

    /// Read **RB_UIE_TRANSFER** `[1:1]`
    ///
    /// enable interrupt for USB transfer completion
    #[inline(always)]
    pub const fn rb_uie_transfer(&self) -> bool {
        (self.bits & IntEn::RB_UIE_TRANSFER_MASK) != 0
    }

    /// Read **RB_UIE_BUS_RST__UIE_DETECT** `[0:0]`
    ///
    /// enable interrupt for USB bus reset event for USB device mode;enable interrupt for USB device detected event for USB host mode
    #[inline(always)]
    pub const fn rb_uie_bus_rst_uie_detect(&self) -> bool {
        (self.bits & IntEn::RB_UIE_BUS_RST_UIE_DETECT_MASK) != 0
    }

    /// Set **RB_UIE_DEV_SOF** `[7:7]`
    ///
    /// RB_UIE_DEV_SOF
    #[inline(always)]
    pub const fn with_rb_uie_dev_sof(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEn::RB_UIE_DEV_SOF_MASK
            } else {
                self.bits & !IntEn::RB_UIE_DEV_SOF_MASK
            },
        }
    }

    /// Set **RB_UIE_DEV_NAK** `[6:6]`
    ///
    /// RB_UIE_DEV_NAK
    #[inline(always)]
    pub const fn with_rb_uie_dev_nak(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEn::RB_UIE_DEV_NAK_MASK
            } else {
                self.bits & !IntEn::RB_UIE_DEV_NAK_MASK
            },
        }
    }

    /// Set **RB_UIE_FIFO_OV** `[4:4]`
    ///
    /// enable interrupt for FIFO overflow
    #[inline(always)]
    pub const fn with_rb_uie_fifo_ov(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEn::RB_UIE_FIFO_OV_MASK
            } else {
                self.bits & !IntEn::RB_UIE_FIFO_OV_MASK
            },
        }
    }

    /// Set **RB_UIE_SOF_ACT** `[3:3]`
    ///
    /// indicate host SOF timer action status for USB host
    #[inline(always)]
    pub const fn with_rb_uie_sof_act(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEn::RB_UIE_SOF_ACT_MASK
            } else {
                self.bits & !IntEn::RB_UIE_SOF_ACT_MASK
            },
        }
    }

    /// Set **RB_UIE_SUSPEND** `[2:2]`
    ///
    /// enable interrupt for USB suspend or resume event
    #[inline(always)]
    pub const fn with_rb_uie_suspend(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEn::RB_UIE_SUSPEND_MASK
            } else {
                self.bits & !IntEn::RB_UIE_SUSPEND_MASK
            },
        }
    }

    /// Set **RB_UIE_TRANSFER** `[1:1]`
    ///
    /// enable interrupt for USB transfer completion
    #[inline(always)]
    pub const fn with_rb_uie_transfer(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEn::RB_UIE_TRANSFER_MASK
            } else {
                self.bits & !IntEn::RB_UIE_TRANSFER_MASK
            },
        }
    }

    /// Set **RB_UIE_BUS_RST__UIE_DETECT** `[0:0]`
    ///
    /// enable interrupt for USB bus reset event for USB device mode;enable interrupt for USB device detected event for USB host mode
    #[inline(always)]
    pub const fn with_rb_uie_bus_rst_uie_detect(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEn::RB_UIE_BUS_RST_UIE_DETECT_MASK
            } else {
                self.bits & !IntEn::RB_UIE_BUS_RST_UIE_DETECT_MASK
            },
        }
    }
}

impl Default for IntEnVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IntEn {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntEnVal {
        IntEnVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntEnVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntEnVal) -> IntEnVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DEV_ADDR Register
///
/// USB device address
///
/// Address offset: 0x0003
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **MASK_USB_ADDR** `[6:0]` (RW)
///
///   bit mask for USB device address
#[repr(transparent)]
pub struct DevAddr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DevAddr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevAddr {
    // ======== Field Constants ========

    /// **MASK_USB_ADDR**
    ///
    /// bit mask for USB device address
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const MASK_USB_ADDR_OFFSET: u32 = 0;
    pub const MASK_USB_ADDR_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **MASK_USB_ADDR** `[6:0]` (RW)
    ///
    /// bit mask for USB device address
    #[inline(always)]
    pub fn mask_usb_addr(&self) -> u32 {
        (self.value.read_bits() & Self::MASK_USB_ADDR_MASK) >> Self::MASK_USB_ADDR_OFFSET
    }

    /// Write **MASK_USB_ADDR** `[6:0]` (RW)
    ///
    /// bit mask for USB device address
    #[inline(always)]
    pub fn write_mask_usb_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MASK_USB_ADDR_MASK)
                | ((value << Self::MASK_USB_ADDR_OFFSET) & Self::MASK_USB_ADDR_MASK)
        });
    }
}

/// DEV_ADDR value type for batch read/modify/write operations
///
/// Use `devaddr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devaddr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevAddrVal {
    bits: u32,
}

impl DevAddrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **MASK_USB_ADDR** `[6:0]`
    ///
    /// bit mask for USB device address
    #[inline(always)]
    pub const fn mask_usb_addr(&self) -> u32 {
        (self.bits & DevAddr::MASK_USB_ADDR_MASK) >> DevAddr::MASK_USB_ADDR_OFFSET
    }

    /// Set **MASK_USB_ADDR** `[6:0]`
    ///
    /// bit mask for USB device address
    #[inline(always)]
    pub const fn with_mask_usb_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevAddr::MASK_USB_ADDR_MASK)
                | ((value << DevAddr::MASK_USB_ADDR_OFFSET) & DevAddr::MASK_USB_ADDR_MASK),
        }
    }
}

impl Default for DevAddrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DevAddr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevAddrVal {
        DevAddrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DevAddrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DevAddrVal) -> DevAddrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// MIS_ST Register
///
/// USB_MIS_ST
///
/// Address offset: 0x0005
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RB_UMS_SOF_PRES** `[7]` (RW)
///
///   RB_UMS_SOF_PRES
///
/// - **RB_UMS_SOF_ACT** `[6]` (RW)
///
///   RB_UMS_SOF_ACT
///
/// - **RB_UMS_SIE_FREE** `[5]` (RW)
///
///   RB_UMS_SIE_FREE
///
/// - **RB_UMS_R_FIFO_RDY** `[4]` (RW)
///
///   RB_UMS_R_FIFO_RDY
///
/// - **RB_UMS_BUS_RST** `[3]` (RW)
///
///   RB_UMS_BUS_RST
///
/// - **RB_UMS_SUSPEND** `[2]` (RW)
///
///   RB_UMS_SUSPEND
///
/// - **RB_UMS_DM_LEVEL** `[1]` (RW)
///
///   RB_UMS_DM_LEVEL
///
/// - **RB_UMS_DEV_ATTACH** `[0]` (RW)
///
///   RB_UMS_DEV_ATTACH
#[repr(transparent)]
pub struct MisSt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MisSt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MisSt {
    // ======== Field Constants ========

    /// **RB_UMS_SOF_PRES**
    ///
    /// RB_UMS_SOF_PRES
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const RB_UMS_SOF_PRES_OFFSET: u32 = 7;
    pub const RB_UMS_SOF_PRES_MASK: u32 = 0x00000080;
    pub const RB_UMS_SOF_PRES: u32 = 1 << 7;

    /// **RB_UMS_SOF_ACT**
    ///
    /// RB_UMS_SOF_ACT
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const RB_UMS_SOF_ACT_OFFSET: u32 = 6;
    pub const RB_UMS_SOF_ACT_MASK: u32 = 0x00000040;
    pub const RB_UMS_SOF_ACT: u32 = 1 << 6;

    /// **RB_UMS_SIE_FREE**
    ///
    /// RB_UMS_SIE_FREE
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const RB_UMS_SIE_FREE_OFFSET: u32 = 5;
    pub const RB_UMS_SIE_FREE_MASK: u32 = 0x00000020;
    pub const RB_UMS_SIE_FREE: u32 = 1 << 5;

    /// **RB_UMS_R_FIFO_RDY**
    ///
    /// RB_UMS_R_FIFO_RDY
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RB_UMS_R_FIFO_RDY_OFFSET: u32 = 4;
    pub const RB_UMS_R_FIFO_RDY_MASK: u32 = 0x00000010;
    pub const RB_UMS_R_FIFO_RDY: u32 = 1 << 4;

    /// **RB_UMS_BUS_RST**
    ///
    /// RB_UMS_BUS_RST
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const RB_UMS_BUS_RST_OFFSET: u32 = 3;
    pub const RB_UMS_BUS_RST_MASK: u32 = 0x00000008;
    pub const RB_UMS_BUS_RST: u32 = 1 << 3;

    /// **RB_UMS_SUSPEND**
    ///
    /// RB_UMS_SUSPEND
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RB_UMS_SUSPEND_OFFSET: u32 = 2;
    pub const RB_UMS_SUSPEND_MASK: u32 = 0x00000004;
    pub const RB_UMS_SUSPEND: u32 = 1 << 2;

    /// **RB_UMS_DM_LEVEL**
    ///
    /// RB_UMS_DM_LEVEL
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RB_UMS_DM_LEVEL_OFFSET: u32 = 1;
    pub const RB_UMS_DM_LEVEL_MASK: u32 = 0x00000002;
    pub const RB_UMS_DM_LEVEL: u32 = 1 << 1;

    /// **RB_UMS_DEV_ATTACH**
    ///
    /// RB_UMS_DEV_ATTACH
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RB_UMS_DEV_ATTACH_OFFSET: u32 = 0;
    pub const RB_UMS_DEV_ATTACH_MASK: u32 = 0x00000001;
    pub const RB_UMS_DEV_ATTACH: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RB_UMS_SOF_PRES** `[7]` (RW)
    ///
    /// RB_UMS_SOF_PRES
    #[inline(always)]
    pub fn rb_ums_sof_pres(&self) -> bool {
        (self.value.read_bits() & Self::RB_UMS_SOF_PRES_MASK) != 0
    }

    /// **RB_UMS_SOF_ACT** `[6]` (RW)
    ///
    /// RB_UMS_SOF_ACT
    #[inline(always)]
    pub fn rb_ums_sof_act(&self) -> bool {
        (self.value.read_bits() & Self::RB_UMS_SOF_ACT_MASK) != 0
    }

    /// **RB_UMS_SIE_FREE** `[5]` (RW)
    ///
    /// RB_UMS_SIE_FREE
    #[inline(always)]
    pub fn rb_ums_sie_free(&self) -> bool {
        (self.value.read_bits() & Self::RB_UMS_SIE_FREE_MASK) != 0
    }

    /// **RB_UMS_R_FIFO_RDY** `[4]` (RW)
    ///
    /// RB_UMS_R_FIFO_RDY
    #[inline(always)]
    pub fn rb_ums_r_fifo_rdy(&self) -> bool {
        (self.value.read_bits() & Self::RB_UMS_R_FIFO_RDY_MASK) != 0
    }

    /// **RB_UMS_BUS_RST** `[3]` (RW)
    ///
    /// RB_UMS_BUS_RST
    #[inline(always)]
    pub fn rb_ums_bus_rst(&self) -> bool {
        (self.value.read_bits() & Self::RB_UMS_BUS_RST_MASK) != 0
    }

    /// **RB_UMS_SUSPEND** `[2]` (RW)
    ///
    /// RB_UMS_SUSPEND
    #[inline(always)]
    pub fn rb_ums_suspend(&self) -> bool {
        (self.value.read_bits() & Self::RB_UMS_SUSPEND_MASK) != 0
    }

    /// **RB_UMS_DM_LEVEL** `[1]` (RW)
    ///
    /// RB_UMS_DM_LEVEL
    #[inline(always)]
    pub fn rb_ums_dm_level(&self) -> bool {
        (self.value.read_bits() & Self::RB_UMS_DM_LEVEL_MASK) != 0
    }

    /// **RB_UMS_DEV_ATTACH** `[0]` (RW)
    ///
    /// RB_UMS_DEV_ATTACH
    #[inline(always)]
    pub fn rb_ums_dev_attach(&self) -> bool {
        (self.value.read_bits() & Self::RB_UMS_DEV_ATTACH_MASK) != 0
    }
}

/// MIS_ST value type for batch read/modify/write operations
///
/// Use `misst.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `misst.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MisStVal {
    bits: u32,
}

impl MisStVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **RB_UMS_SOF_PRES** `[7:7]`
    ///
    /// RB_UMS_SOF_PRES
    #[inline(always)]
    pub const fn rb_ums_sof_pres(&self) -> bool {
        (self.bits & MisSt::RB_UMS_SOF_PRES_MASK) != 0
    }

    /// Read **RB_UMS_SOF_ACT** `[6:6]`
    ///
    /// RB_UMS_SOF_ACT
    #[inline(always)]
    pub const fn rb_ums_sof_act(&self) -> bool {
        (self.bits & MisSt::RB_UMS_SOF_ACT_MASK) != 0
    }

    /// Read **RB_UMS_SIE_FREE** `[5:5]`
    ///
    /// RB_UMS_SIE_FREE
    #[inline(always)]
    pub const fn rb_ums_sie_free(&self) -> bool {
        (self.bits & MisSt::RB_UMS_SIE_FREE_MASK) != 0
    }

    /// Read **RB_UMS_R_FIFO_RDY** `[4:4]`
    ///
    /// RB_UMS_R_FIFO_RDY
    #[inline(always)]
    pub const fn rb_ums_r_fifo_rdy(&self) -> bool {
        (self.bits & MisSt::RB_UMS_R_FIFO_RDY_MASK) != 0
    }

    /// Read **RB_UMS_BUS_RST** `[3:3]`
    ///
    /// RB_UMS_BUS_RST
    #[inline(always)]
    pub const fn rb_ums_bus_rst(&self) -> bool {
        (self.bits & MisSt::RB_UMS_BUS_RST_MASK) != 0
    }

    /// Read **RB_UMS_SUSPEND** `[2:2]`
    ///
    /// RB_UMS_SUSPEND
    #[inline(always)]
    pub const fn rb_ums_suspend(&self) -> bool {
        (self.bits & MisSt::RB_UMS_SUSPEND_MASK) != 0
    }

    /// Read **RB_UMS_DM_LEVEL** `[1:1]`
    ///
    /// RB_UMS_DM_LEVEL
    #[inline(always)]
    pub const fn rb_ums_dm_level(&self) -> bool {
        (self.bits & MisSt::RB_UMS_DM_LEVEL_MASK) != 0
    }

    /// Read **RB_UMS_DEV_ATTACH** `[0:0]`
    ///
    /// RB_UMS_DEV_ATTACH
    #[inline(always)]
    pub const fn rb_ums_dev_attach(&self) -> bool {
        (self.bits & MisSt::RB_UMS_DEV_ATTACH_MASK) != 0
    }

    /// Set **RB_UMS_SOF_PRES** `[7:7]`
    ///
    /// RB_UMS_SOF_PRES
    #[inline(always)]
    pub const fn with_rb_ums_sof_pres(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MisSt::RB_UMS_SOF_PRES_MASK
            } else {
                self.bits & !MisSt::RB_UMS_SOF_PRES_MASK
            },
        }
    }

    /// Set **RB_UMS_SOF_ACT** `[6:6]`
    ///
    /// RB_UMS_SOF_ACT
    #[inline(always)]
    pub const fn with_rb_ums_sof_act(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MisSt::RB_UMS_SOF_ACT_MASK
            } else {
                self.bits & !MisSt::RB_UMS_SOF_ACT_MASK
            },
        }
    }

    /// Set **RB_UMS_SIE_FREE** `[5:5]`
    ///
    /// RB_UMS_SIE_FREE
    #[inline(always)]
    pub const fn with_rb_ums_sie_free(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MisSt::RB_UMS_SIE_FREE_MASK
            } else {
                self.bits & !MisSt::RB_UMS_SIE_FREE_MASK
            },
        }
    }

    /// Set **RB_UMS_R_FIFO_RDY** `[4:4]`
    ///
    /// RB_UMS_R_FIFO_RDY
    #[inline(always)]
    pub const fn with_rb_ums_r_fifo_rdy(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MisSt::RB_UMS_R_FIFO_RDY_MASK
            } else {
                self.bits & !MisSt::RB_UMS_R_FIFO_RDY_MASK
            },
        }
    }

    /// Set **RB_UMS_BUS_RST** `[3:3]`
    ///
    /// RB_UMS_BUS_RST
    #[inline(always)]
    pub const fn with_rb_ums_bus_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MisSt::RB_UMS_BUS_RST_MASK
            } else {
                self.bits & !MisSt::RB_UMS_BUS_RST_MASK
            },
        }
    }

    /// Set **RB_UMS_SUSPEND** `[2:2]`
    ///
    /// RB_UMS_SUSPEND
    #[inline(always)]
    pub const fn with_rb_ums_suspend(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MisSt::RB_UMS_SUSPEND_MASK
            } else {
                self.bits & !MisSt::RB_UMS_SUSPEND_MASK
            },
        }
    }

    /// Set **RB_UMS_DM_LEVEL** `[1:1]`
    ///
    /// RB_UMS_DM_LEVEL
    #[inline(always)]
    pub const fn with_rb_ums_dm_level(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MisSt::RB_UMS_DM_LEVEL_MASK
            } else {
                self.bits & !MisSt::RB_UMS_DM_LEVEL_MASK
            },
        }
    }

    /// Set **RB_UMS_DEV_ATTACH** `[0:0]`
    ///
    /// RB_UMS_DEV_ATTACH
    #[inline(always)]
    pub const fn with_rb_ums_dev_attach(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MisSt::RB_UMS_DEV_ATTACH_MASK
            } else {
                self.bits & !MisSt::RB_UMS_DEV_ATTACH_MASK
            },
        }
    }
}

impl Default for MisStVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl MisSt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> MisStVal {
        MisStVal::from_bits(self.value.read_bits())
    }
}

/// INT_FG Register
///
/// USB_INT_FG
///
/// Address offset: 0x0006
/// Reset value: 0x00000020
///
/// ## Fields
///
/// - **RB_U_IS_NAK** `[7]` (RO)
///
///   RB_U_IS_NAK
///
/// - **TOG_MATCH_SYNC** `[6]` (RO)
///
///   TOG_MATCH_SYNC
///
/// - **RB_U_SIE_FREE** `[5]` (RO)
///
///   RB_U_SIE_FREE
///
/// - **RB_UIF_FIFO_OV** `[4]` (RW)
///
///   RB_UIF_FIFO_OV
///
/// - **RB_UIF_HST_SOF** `[3]` (RW)
///
///   RB_UIF_HST_SOF
///
/// - **RB_UIF_SUSPEND** `[2]` (RW)
///
///   RB_UIF_SUSPEND
///
/// - **RB_UIF_TRANSFER** `[1]` (RW)
///
///   RB_UIF_TRANSFER
///
/// - **RB_UIF_BUS_RST_RB_UIF_DETECT** `[0]` (RW)
///
///   RB_UIF_BUS_RST_RB_UIF_DETECT
#[repr(transparent)]
pub struct IntFg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntFg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntFg {
    // ======== Field Constants ========

    /// **RB_U_IS_NAK**
    ///
    /// RB_U_IS_NAK
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const RB_U_IS_NAK_OFFSET: u32 = 7;
    pub const RB_U_IS_NAK_MASK: u32 = 0x00000080;
    pub const RB_U_IS_NAK: u32 = 1 << 7;

    /// **TOG_MATCH_SYNC**
    ///
    /// TOG_MATCH_SYNC
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const TOG_MATCH_SYNC_OFFSET: u32 = 6;
    pub const TOG_MATCH_SYNC_MASK: u32 = 0x00000040;
    pub const TOG_MATCH_SYNC: u32 = 1 << 6;

    /// **RB_U_SIE_FREE**
    ///
    /// RB_U_SIE_FREE
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const RB_U_SIE_FREE_OFFSET: u32 = 5;
    pub const RB_U_SIE_FREE_MASK: u32 = 0x00000020;
    pub const RB_U_SIE_FREE: u32 = 1 << 5;

    /// **RB_UIF_FIFO_OV**
    ///
    /// RB_UIF_FIFO_OV
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RB_UIF_FIFO_OV_OFFSET: u32 = 4;
    pub const RB_UIF_FIFO_OV_MASK: u32 = 0x00000010;
    pub const RB_UIF_FIFO_OV: u32 = 1 << 4;

    /// **RB_UIF_HST_SOF**
    ///
    /// RB_UIF_HST_SOF
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const RB_UIF_HST_SOF_OFFSET: u32 = 3;
    pub const RB_UIF_HST_SOF_MASK: u32 = 0x00000008;
    pub const RB_UIF_HST_SOF: u32 = 1 << 3;

    /// **RB_UIF_SUSPEND**
    ///
    /// RB_UIF_SUSPEND
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RB_UIF_SUSPEND_OFFSET: u32 = 2;
    pub const RB_UIF_SUSPEND_MASK: u32 = 0x00000004;
    pub const RB_UIF_SUSPEND: u32 = 1 << 2;

    /// **RB_UIF_TRANSFER**
    ///
    /// RB_UIF_TRANSFER
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RB_UIF_TRANSFER_OFFSET: u32 = 1;
    pub const RB_UIF_TRANSFER_MASK: u32 = 0x00000002;
    pub const RB_UIF_TRANSFER: u32 = 1 << 1;

    /// **RB_UIF_BUS_RST_RB_UIF_DETECT**
    ///
    /// RB_UIF_BUS_RST_RB_UIF_DETECT
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RB_UIF_BUS_RST_RB_UIF_DETECT_OFFSET: u32 = 0;
    pub const RB_UIF_BUS_RST_RB_UIF_DETECT_MASK: u32 = 0x00000001;
    pub const RB_UIF_BUS_RST_RB_UIF_DETECT: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RB_U_IS_NAK** `[7]` (RO)
    ///
    /// RB_U_IS_NAK
    #[inline(always)]
    pub fn rb_u_is_nak(&self) -> bool {
        (self.value.read_bits() & Self::RB_U_IS_NAK_MASK) != 0
    }

    /// **TOG_MATCH_SYNC** `[6]` (RO)
    ///
    /// TOG_MATCH_SYNC
    #[inline(always)]
    pub fn tog_match_sync(&self) -> bool {
        (self.value.read_bits() & Self::TOG_MATCH_SYNC_MASK) != 0
    }

    /// **RB_U_SIE_FREE** `[5]` (RO)
    ///
    /// RB_U_SIE_FREE
    #[inline(always)]
    pub fn rb_u_sie_free(&self) -> bool {
        (self.value.read_bits() & Self::RB_U_SIE_FREE_MASK) != 0
    }

    /// **RB_UIF_FIFO_OV** `[4]` (RW)
    ///
    /// RB_UIF_FIFO_OV
    #[inline(always)]
    pub fn rb_uif_fifo_ov(&self) -> bool {
        (self.value.read_bits() & Self::RB_UIF_FIFO_OV_MASK) != 0
    }

    /// Set RB_UIF_FIFO_OV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rb_uif_fifo_ov(&self) {
        self.value.modify_bits(|r| r | Self::RB_UIF_FIFO_OV_MASK);
    }

    /// Clear RB_UIF_FIFO_OV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rb_uif_fifo_ov(&self) {
        self.value.modify_bits(|r| r & !Self::RB_UIF_FIFO_OV_MASK);
    }

    /// **RB_UIF_HST_SOF** `[3]` (RW)
    ///
    /// RB_UIF_HST_SOF
    #[inline(always)]
    pub fn rb_uif_hst_sof(&self) -> bool {
        (self.value.read_bits() & Self::RB_UIF_HST_SOF_MASK) != 0
    }

    /// Set RB_UIF_HST_SOF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rb_uif_hst_sof(&self) {
        self.value.modify_bits(|r| r | Self::RB_UIF_HST_SOF_MASK);
    }

    /// Clear RB_UIF_HST_SOF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rb_uif_hst_sof(&self) {
        self.value.modify_bits(|r| r & !Self::RB_UIF_HST_SOF_MASK);
    }

    /// **RB_UIF_SUSPEND** `[2]` (RW)
    ///
    /// RB_UIF_SUSPEND
    #[inline(always)]
    pub fn rb_uif_suspend(&self) -> bool {
        (self.value.read_bits() & Self::RB_UIF_SUSPEND_MASK) != 0
    }

    /// Set RB_UIF_SUSPEND bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rb_uif_suspend(&self) {
        self.value.modify_bits(|r| r | Self::RB_UIF_SUSPEND_MASK);
    }

    /// Clear RB_UIF_SUSPEND bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rb_uif_suspend(&self) {
        self.value.modify_bits(|r| r & !Self::RB_UIF_SUSPEND_MASK);
    }

    /// **RB_UIF_TRANSFER** `[1]` (RW)
    ///
    /// RB_UIF_TRANSFER
    #[inline(always)]
    pub fn rb_uif_transfer(&self) -> bool {
        (self.value.read_bits() & Self::RB_UIF_TRANSFER_MASK) != 0
    }

    /// Set RB_UIF_TRANSFER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rb_uif_transfer(&self) {
        self.value.modify_bits(|r| r | Self::RB_UIF_TRANSFER_MASK);
    }

    /// Clear RB_UIF_TRANSFER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rb_uif_transfer(&self) {
        self.value.modify_bits(|r| r & !Self::RB_UIF_TRANSFER_MASK);
    }

    /// **RB_UIF_BUS_RST_RB_UIF_DETECT** `[0]` (RW)
    ///
    /// RB_UIF_BUS_RST_RB_UIF_DETECT
    #[inline(always)]
    pub fn rb_uif_bus_rst_rb_uif_detect(&self) -> bool {
        (self.value.read_bits() & Self::RB_UIF_BUS_RST_RB_UIF_DETECT_MASK) != 0
    }

    /// Set RB_UIF_BUS_RST_RB_UIF_DETECT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rb_uif_bus_rst_rb_uif_detect(&self) {
        self.value
            .modify_bits(|r| r | Self::RB_UIF_BUS_RST_RB_UIF_DETECT_MASK);
    }

    /// Clear RB_UIF_BUS_RST_RB_UIF_DETECT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rb_uif_bus_rst_rb_uif_detect(&self) {
        self.value
            .modify_bits(|r| r & !Self::RB_UIF_BUS_RST_RB_UIF_DETECT_MASK);
    }
}

/// INT_FG value type for batch read/modify/write operations
///
/// Use `intfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntFgVal {
    bits: u32,
}

impl IntFgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000020 }
    }

    /// Read **RB_U_IS_NAK** `[7:7]`
    ///
    /// RB_U_IS_NAK
    #[inline(always)]
    pub const fn rb_u_is_nak(&self) -> bool {
        (self.bits & IntFg::RB_U_IS_NAK_MASK) != 0
    }

    /// Read **TOG_MATCH_SYNC** `[6:6]`
    ///
    /// TOG_MATCH_SYNC
    #[inline(always)]
    pub const fn tog_match_sync(&self) -> bool {
        (self.bits & IntFg::TOG_MATCH_SYNC_MASK) != 0
    }

    /// Read **RB_U_SIE_FREE** `[5:5]`
    ///
    /// RB_U_SIE_FREE
    #[inline(always)]
    pub const fn rb_u_sie_free(&self) -> bool {
        (self.bits & IntFg::RB_U_SIE_FREE_MASK) != 0
    }

    /// Read **RB_UIF_FIFO_OV** `[4:4]`
    ///
    /// RB_UIF_FIFO_OV
    #[inline(always)]
    pub const fn rb_uif_fifo_ov(&self) -> bool {
        (self.bits & IntFg::RB_UIF_FIFO_OV_MASK) != 0
    }

    /// Read **RB_UIF_HST_SOF** `[3:3]`
    ///
    /// RB_UIF_HST_SOF
    #[inline(always)]
    pub const fn rb_uif_hst_sof(&self) -> bool {
        (self.bits & IntFg::RB_UIF_HST_SOF_MASK) != 0
    }

    /// Read **RB_UIF_SUSPEND** `[2:2]`
    ///
    /// RB_UIF_SUSPEND
    #[inline(always)]
    pub const fn rb_uif_suspend(&self) -> bool {
        (self.bits & IntFg::RB_UIF_SUSPEND_MASK) != 0
    }

    /// Read **RB_UIF_TRANSFER** `[1:1]`
    ///
    /// RB_UIF_TRANSFER
    #[inline(always)]
    pub const fn rb_uif_transfer(&self) -> bool {
        (self.bits & IntFg::RB_UIF_TRANSFER_MASK) != 0
    }

    /// Read **RB_UIF_BUS_RST_RB_UIF_DETECT** `[0:0]`
    ///
    /// RB_UIF_BUS_RST_RB_UIF_DETECT
    #[inline(always)]
    pub const fn rb_uif_bus_rst_rb_uif_detect(&self) -> bool {
        (self.bits & IntFg::RB_UIF_BUS_RST_RB_UIF_DETECT_MASK) != 0
    }

    /// Set **RB_UIF_FIFO_OV** `[4:4]`
    ///
    /// RB_UIF_FIFO_OV
    #[inline(always)]
    pub const fn with_rb_uif_fifo_ov(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntFg::RB_UIF_FIFO_OV_MASK
            } else {
                self.bits & !IntFg::RB_UIF_FIFO_OV_MASK
            },
        }
    }

    /// Set **RB_UIF_HST_SOF** `[3:3]`
    ///
    /// RB_UIF_HST_SOF
    #[inline(always)]
    pub const fn with_rb_uif_hst_sof(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntFg::RB_UIF_HST_SOF_MASK
            } else {
                self.bits & !IntFg::RB_UIF_HST_SOF_MASK
            },
        }
    }

    /// Set **RB_UIF_SUSPEND** `[2:2]`
    ///
    /// RB_UIF_SUSPEND
    #[inline(always)]
    pub const fn with_rb_uif_suspend(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntFg::RB_UIF_SUSPEND_MASK
            } else {
                self.bits & !IntFg::RB_UIF_SUSPEND_MASK
            },
        }
    }

    /// Set **RB_UIF_TRANSFER** `[1:1]`
    ///
    /// RB_UIF_TRANSFER
    #[inline(always)]
    pub const fn with_rb_uif_transfer(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntFg::RB_UIF_TRANSFER_MASK
            } else {
                self.bits & !IntFg::RB_UIF_TRANSFER_MASK
            },
        }
    }

    /// Set **RB_UIF_BUS_RST_RB_UIF_DETECT** `[0:0]`
    ///
    /// RB_UIF_BUS_RST_RB_UIF_DETECT
    #[inline(always)]
    pub const fn with_rb_uif_bus_rst_rb_uif_detect(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntFg::RB_UIF_BUS_RST_RB_UIF_DETECT_MASK
            } else {
                self.bits & !IntFg::RB_UIF_BUS_RST_RB_UIF_DETECT_MASK
            },
        }
    }
}

impl Default for IntFgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IntFg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntFgVal {
        IntFgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntFgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntFgVal) -> IntFgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_ST Register
///
/// USB_INT_ST
///
/// Address offset: 0x0007
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SETUP_ACT** `[7]` (RO)
///
///   SETUP_ACT
///
/// - **RB_UIS_TOG_OK** `[6]` (RO)
///
///   RB_UIS_TOG_OK
///
/// - **MASK_UIS_TOKEN** `[5:4]` (RO)
///
///   MASK_UIS_TOKEN
///
/// - **MASK_UIS_H_RES_MASK_UIS_ENDP** `[3:0]` (RO)
///
///   MASK_UIS_H_RES_MASK_UIS_ENDP
#[repr(transparent)]
pub struct IntSt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntSt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntSt {
    // ======== Field Constants ========

    /// **SETUP_ACT**
    ///
    /// SETUP_ACT
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SETUP_ACT_OFFSET: u32 = 7;
    pub const SETUP_ACT_MASK: u32 = 0x00000080;
    pub const SETUP_ACT: u32 = 1 << 7;

    /// **RB_UIS_TOG_OK**
    ///
    /// RB_UIS_TOG_OK
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const RB_UIS_TOG_OK_OFFSET: u32 = 6;
    pub const RB_UIS_TOG_OK_MASK: u32 = 0x00000040;
    pub const RB_UIS_TOG_OK: u32 = 1 << 6;

    /// **MASK_UIS_TOKEN**
    ///
    /// MASK_UIS_TOKEN
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const MASK_UIS_TOKEN_OFFSET: u32 = 4;
    pub const MASK_UIS_TOKEN_MASK: u32 = 0x00000030;

    /// **MASK_UIS_H_RES_MASK_UIS_ENDP**
    ///
    /// MASK_UIS_H_RES_MASK_UIS_ENDP
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const MASK_UIS_H_RES_MASK_UIS_ENDP_OFFSET: u32 = 0;
    pub const MASK_UIS_H_RES_MASK_UIS_ENDP_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **SETUP_ACT** `[7]` (RO)
    ///
    /// SETUP_ACT
    #[inline(always)]
    pub fn setup_act(&self) -> bool {
        (self.value.read_bits() & Self::SETUP_ACT_MASK) != 0
    }

    /// **RB_UIS_TOG_OK** `[6]` (RO)
    ///
    /// RB_UIS_TOG_OK
    #[inline(always)]
    pub fn rb_uis_tog_ok(&self) -> bool {
        (self.value.read_bits() & Self::RB_UIS_TOG_OK_MASK) != 0
    }

    /// **MASK_UIS_TOKEN** `[5:4]` (RO)
    ///
    /// MASK_UIS_TOKEN
    #[inline(always)]
    pub fn mask_uis_token(&self) -> u32 {
        (self.value.read_bits() & Self::MASK_UIS_TOKEN_MASK) >> Self::MASK_UIS_TOKEN_OFFSET
    }

    /// **MASK_UIS_H_RES_MASK_UIS_ENDP** `[3:0]` (RO)
    ///
    /// MASK_UIS_H_RES_MASK_UIS_ENDP
    #[inline(always)]
    pub fn mask_uis_h_res_mask_uis_endp(&self) -> u32 {
        (self.value.read_bits() & Self::MASK_UIS_H_RES_MASK_UIS_ENDP_MASK)
            >> Self::MASK_UIS_H_RES_MASK_UIS_ENDP_OFFSET
    }
}

/// INT_ST value type for batch read/modify/write operations
///
/// Use `intst.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intst.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntStVal {
    bits: u32,
}

impl IntStVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **SETUP_ACT** `[7:7]`
    ///
    /// SETUP_ACT
    #[inline(always)]
    pub const fn setup_act(&self) -> bool {
        (self.bits & IntSt::SETUP_ACT_MASK) != 0
    }

    /// Read **RB_UIS_TOG_OK** `[6:6]`
    ///
    /// RB_UIS_TOG_OK
    #[inline(always)]
    pub const fn rb_uis_tog_ok(&self) -> bool {
        (self.bits & IntSt::RB_UIS_TOG_OK_MASK) != 0
    }

    /// Read **MASK_UIS_TOKEN** `[5:4]`
    ///
    /// MASK_UIS_TOKEN
    #[inline(always)]
    pub const fn mask_uis_token(&self) -> u32 {
        (self.bits & IntSt::MASK_UIS_TOKEN_MASK) >> IntSt::MASK_UIS_TOKEN_OFFSET
    }

    /// Read **MASK_UIS_H_RES_MASK_UIS_ENDP** `[3:0]`
    ///
    /// MASK_UIS_H_RES_MASK_UIS_ENDP
    #[inline(always)]
    pub const fn mask_uis_h_res_mask_uis_endp(&self) -> u32 {
        (self.bits & IntSt::MASK_UIS_H_RES_MASK_UIS_ENDP_MASK)
            >> IntSt::MASK_UIS_H_RES_MASK_UIS_ENDP_OFFSET
    }
}

impl Default for IntStVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IntSt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntStVal {
        IntStVal::from_bits(self.value.read_bits())
    }
}

/// RX_LEN Register
///
/// USB_RX_LEN
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **USB_RX_LEN** `[6:0]` (RO)
///
///   USB_RX_LEN
#[repr(transparent)]
pub struct RxLen {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RxLen {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RxLen {
    // ======== Field Constants ========

    /// **USB_RX_LEN**
    ///
    /// USB_RX_LEN
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const USB_RX_LEN_OFFSET: u32 = 0;
    pub const USB_RX_LEN_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **USB_RX_LEN** `[6:0]` (RO)
    ///
    /// USB_RX_LEN
    #[inline(always)]
    pub fn usb_rx_len(&self) -> u32 {
        (self.value.read_bits() & Self::USB_RX_LEN_MASK) >> Self::USB_RX_LEN_OFFSET
    }
}

/// RX_LEN value type for batch read/modify/write operations
///
/// Use `rxlen.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxlen.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RxLenVal {
    bits: u32,
}

impl RxLenVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **USB_RX_LEN** `[6:0]`
    ///
    /// USB_RX_LEN
    #[inline(always)]
    pub const fn usb_rx_len(&self) -> u32 {
        (self.bits & RxLen::USB_RX_LEN_MASK) >> RxLen::USB_RX_LEN_OFFSET
    }
}

impl Default for RxLenVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RxLen {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RxLenVal {
        RxLenVal::from_bits(self.value.read_bits())
    }
}

/// UEP4_1_MOD Register
///
/// UEP4_1_MOD
///
/// Address offset: 0x000c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EP1_R_EN** `[7]` (RW)
///
///   EP1_R_EN
///
/// - **EP1_T_EN** `[6]` (RW)
///
///   EP1_T_EN
///
/// - **EP1_BUF_MOD** `[4]` (RW)
///
///   EP1_BUF_MOD
///
/// - **EP4_R_EN** `[3]` (RW)
///
///   EP4_R_EN
///
/// - **EP4_T_EN** `[2]` (RW)
///
///   EP4_T_EN
#[repr(transparent)]
pub struct Uep41Mod {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uep41Mod {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uep41Mod {
    // ======== Field Constants ========

    /// **EP1_R_EN**
    ///
    /// EP1_R_EN
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const EP1_R_EN_OFFSET: u32 = 7;
    pub const EP1_R_EN_MASK: u32 = 0x00000080;
    pub const EP1_R_EN: u32 = 1 << 7;

    /// **EP1_T_EN**
    ///
    /// EP1_T_EN
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const EP1_T_EN_OFFSET: u32 = 6;
    pub const EP1_T_EN_MASK: u32 = 0x00000040;
    pub const EP1_T_EN: u32 = 1 << 6;

    /// **EP1_BUF_MOD**
    ///
    /// EP1_BUF_MOD
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const EP1_BUF_MOD_OFFSET: u32 = 4;
    pub const EP1_BUF_MOD_MASK: u32 = 0x00000010;
    pub const EP1_BUF_MOD: u32 = 1 << 4;

    /// **EP4_R_EN**
    ///
    /// EP4_R_EN
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const EP4_R_EN_OFFSET: u32 = 3;
    pub const EP4_R_EN_MASK: u32 = 0x00000008;
    pub const EP4_R_EN: u32 = 1 << 3;

    /// **EP4_T_EN**
    ///
    /// EP4_T_EN
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const EP4_T_EN_OFFSET: u32 = 2;
    pub const EP4_T_EN_MASK: u32 = 0x00000004;
    pub const EP4_T_EN: u32 = 1 << 2;

    // ======== Field Accessors ========

    /// **EP1_R_EN** `[7]` (RW)
    ///
    /// EP1_R_EN
    #[inline(always)]
    pub fn ep1_r_en(&self) -> bool {
        (self.value.read_bits() & Self::EP1_R_EN_MASK) != 0
    }

    /// Set EP1_R_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep1_r_en(&self) {
        self.value.modify_bits(|r| r | Self::EP1_R_EN_MASK);
    }

    /// Clear EP1_R_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep1_r_en(&self) {
        self.value.modify_bits(|r| r & !Self::EP1_R_EN_MASK);
    }

    /// **EP1_T_EN** `[6]` (RW)
    ///
    /// EP1_T_EN
    #[inline(always)]
    pub fn ep1_t_en(&self) -> bool {
        (self.value.read_bits() & Self::EP1_T_EN_MASK) != 0
    }

    /// Set EP1_T_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep1_t_en(&self) {
        self.value.modify_bits(|r| r | Self::EP1_T_EN_MASK);
    }

    /// Clear EP1_T_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep1_t_en(&self) {
        self.value.modify_bits(|r| r & !Self::EP1_T_EN_MASK);
    }

    /// **EP1_BUF_MOD** `[4]` (RW)
    ///
    /// EP1_BUF_MOD
    #[inline(always)]
    pub fn ep1_buf_mod(&self) -> bool {
        (self.value.read_bits() & Self::EP1_BUF_MOD_MASK) != 0
    }

    /// Set EP1_BUF_MOD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep1_buf_mod(&self) {
        self.value.modify_bits(|r| r | Self::EP1_BUF_MOD_MASK);
    }

    /// Clear EP1_BUF_MOD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep1_buf_mod(&self) {
        self.value.modify_bits(|r| r & !Self::EP1_BUF_MOD_MASK);
    }

    /// **EP4_R_EN** `[3]` (RW)
    ///
    /// EP4_R_EN
    #[inline(always)]
    pub fn ep4_r_en(&self) -> bool {
        (self.value.read_bits() & Self::EP4_R_EN_MASK) != 0
    }

    /// Set EP4_R_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep4_r_en(&self) {
        self.value.modify_bits(|r| r | Self::EP4_R_EN_MASK);
    }

    /// Clear EP4_R_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep4_r_en(&self) {
        self.value.modify_bits(|r| r & !Self::EP4_R_EN_MASK);
    }

    /// **EP4_T_EN** `[2]` (RW)
    ///
    /// EP4_T_EN
    #[inline(always)]
    pub fn ep4_t_en(&self) -> bool {
        (self.value.read_bits() & Self::EP4_T_EN_MASK) != 0
    }

    /// Set EP4_T_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep4_t_en(&self) {
        self.value.modify_bits(|r| r | Self::EP4_T_EN_MASK);
    }

    /// Clear EP4_T_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep4_t_en(&self) {
        self.value.modify_bits(|r| r & !Self::EP4_T_EN_MASK);
    }
}

/// UEP4_1_MOD value type for batch read/modify/write operations
///
/// Use `uep41mod.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uep41mod.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uep41ModVal {
    bits: u32,
}

impl Uep41ModVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **EP1_R_EN** `[7:7]`
    ///
    /// EP1_R_EN
    #[inline(always)]
    pub const fn ep1_r_en(&self) -> bool {
        (self.bits & Uep41Mod::EP1_R_EN_MASK) != 0
    }

    /// Read **EP1_T_EN** `[6:6]`
    ///
    /// EP1_T_EN
    #[inline(always)]
    pub const fn ep1_t_en(&self) -> bool {
        (self.bits & Uep41Mod::EP1_T_EN_MASK) != 0
    }

    /// Read **EP1_BUF_MOD** `[4:4]`
    ///
    /// EP1_BUF_MOD
    #[inline(always)]
    pub const fn ep1_buf_mod(&self) -> bool {
        (self.bits & Uep41Mod::EP1_BUF_MOD_MASK) != 0
    }

    /// Read **EP4_R_EN** `[3:3]`
    ///
    /// EP4_R_EN
    #[inline(always)]
    pub const fn ep4_r_en(&self) -> bool {
        (self.bits & Uep41Mod::EP4_R_EN_MASK) != 0
    }

    /// Read **EP4_T_EN** `[2:2]`
    ///
    /// EP4_T_EN
    #[inline(always)]
    pub const fn ep4_t_en(&self) -> bool {
        (self.bits & Uep41Mod::EP4_T_EN_MASK) != 0
    }

    /// Set **EP1_R_EN** `[7:7]`
    ///
    /// EP1_R_EN
    #[inline(always)]
    pub const fn with_ep1_r_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep41Mod::EP1_R_EN_MASK
            } else {
                self.bits & !Uep41Mod::EP1_R_EN_MASK
            },
        }
    }

    /// Set **EP1_T_EN** `[6:6]`
    ///
    /// EP1_T_EN
    #[inline(always)]
    pub const fn with_ep1_t_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep41Mod::EP1_T_EN_MASK
            } else {
                self.bits & !Uep41Mod::EP1_T_EN_MASK
            },
        }
    }

    /// Set **EP1_BUF_MOD** `[4:4]`
    ///
    /// EP1_BUF_MOD
    #[inline(always)]
    pub const fn with_ep1_buf_mod(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep41Mod::EP1_BUF_MOD_MASK
            } else {
                self.bits & !Uep41Mod::EP1_BUF_MOD_MASK
            },
        }
    }

    /// Set **EP4_R_EN** `[3:3]`
    ///
    /// EP4_R_EN
    #[inline(always)]
    pub const fn with_ep4_r_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep41Mod::EP4_R_EN_MASK
            } else {
                self.bits & !Uep41Mod::EP4_R_EN_MASK
            },
        }
    }

    /// Set **EP4_T_EN** `[2:2]`
    ///
    /// EP4_T_EN
    #[inline(always)]
    pub const fn with_ep4_t_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep41Mod::EP4_T_EN_MASK
            } else {
                self.bits & !Uep41Mod::EP4_T_EN_MASK
            },
        }
    }
}

impl Default for Uep41ModVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uep41Mod {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uep41ModVal {
        Uep41ModVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Uep41ModVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Uep41ModVal) -> Uep41ModVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UEP2_3_MOD_HOST_EP_MOD Register
///
/// UEP2_3_MOD_HOST_EP_MOD
///
/// Address offset: 0x000d
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EP3_R_EN** `[7]` (RW)
///
///   EP1_R_EN
///
/// - **EP3_T_EN_RB_UH_EP_TX_EN** `[6]` (RW)
///
///   EP1_T_EN_RB_UH_EP_TX_EN
///
/// - **EP3_BUF_MOD_RB_UH_EP_TBUF_MOD** `[4]` (RW)
///
///   EP1_BUF_MOD_RB_UH_EP_TBUF_MOD
///
/// - **EP2_R_EN_RB_UH_EP_RX_EN** `[3]` (RW)
///
///   EP2_R_EN_RB_UH_EP_RX_EN
///
/// - **EP2_T_EN** `[2]` (RW)
///
///   EP2_T_EN
///
/// - **EP2_BUF_MOD_RB_UH_EP_RBUF_MOD** `[0]` (RW)
///
///   EP2_BUF_MOD_RB_UH_EP_RBUF_MOD
#[repr(transparent)]
pub struct Uep23ModHostEpMod {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uep23ModHostEpMod {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uep23ModHostEpMod {
    // ======== Field Constants ========

    /// **EP3_R_EN**
    ///
    /// EP1_R_EN
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const EP3_R_EN_OFFSET: u32 = 7;
    pub const EP3_R_EN_MASK: u32 = 0x00000080;
    pub const EP3_R_EN: u32 = 1 << 7;

    /// **EP3_T_EN_RB_UH_EP_TX_EN**
    ///
    /// EP1_T_EN_RB_UH_EP_TX_EN
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const EP3_T_EN_RB_UH_EP_TX_EN_OFFSET: u32 = 6;
    pub const EP3_T_EN_RB_UH_EP_TX_EN_MASK: u32 = 0x00000040;
    pub const EP3_T_EN_RB_UH_EP_TX_EN: u32 = 1 << 6;

    /// **EP3_BUF_MOD_RB_UH_EP_TBUF_MOD**
    ///
    /// EP1_BUF_MOD_RB_UH_EP_TBUF_MOD
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const EP3_BUF_MOD_RB_UH_EP_TBUF_MOD_OFFSET: u32 = 4;
    pub const EP3_BUF_MOD_RB_UH_EP_TBUF_MOD_MASK: u32 = 0x00000010;
    pub const EP3_BUF_MOD_RB_UH_EP_TBUF_MOD: u32 = 1 << 4;

    /// **EP2_R_EN_RB_UH_EP_RX_EN**
    ///
    /// EP2_R_EN_RB_UH_EP_RX_EN
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const EP2_R_EN_RB_UH_EP_RX_EN_OFFSET: u32 = 3;
    pub const EP2_R_EN_RB_UH_EP_RX_EN_MASK: u32 = 0x00000008;
    pub const EP2_R_EN_RB_UH_EP_RX_EN: u32 = 1 << 3;

    /// **EP2_T_EN**
    ///
    /// EP2_T_EN
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const EP2_T_EN_OFFSET: u32 = 2;
    pub const EP2_T_EN_MASK: u32 = 0x00000004;
    pub const EP2_T_EN: u32 = 1 << 2;

    /// **EP2_BUF_MOD_RB_UH_EP_RBUF_MOD**
    ///
    /// EP2_BUF_MOD_RB_UH_EP_RBUF_MOD
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EP2_BUF_MOD_RB_UH_EP_RBUF_MOD_OFFSET: u32 = 0;
    pub const EP2_BUF_MOD_RB_UH_EP_RBUF_MOD_MASK: u32 = 0x00000001;
    pub const EP2_BUF_MOD_RB_UH_EP_RBUF_MOD: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **EP3_R_EN** `[7]` (RW)
    ///
    /// EP1_R_EN
    #[inline(always)]
    pub fn ep3_r_en(&self) -> bool {
        (self.value.read_bits() & Self::EP3_R_EN_MASK) != 0
    }

    /// Set EP3_R_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep3_r_en(&self) {
        self.value.modify_bits(|r| r | Self::EP3_R_EN_MASK);
    }

    /// Clear EP3_R_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep3_r_en(&self) {
        self.value.modify_bits(|r| r & !Self::EP3_R_EN_MASK);
    }

    /// **EP3_T_EN_RB_UH_EP_TX_EN** `[6]` (RW)
    ///
    /// EP1_T_EN_RB_UH_EP_TX_EN
    #[inline(always)]
    pub fn ep3_t_en_rb_uh_ep_tx_en(&self) -> bool {
        (self.value.read_bits() & Self::EP3_T_EN_RB_UH_EP_TX_EN_MASK) != 0
    }

    /// Set EP3_T_EN_RB_UH_EP_TX_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep3_t_en_rb_uh_ep_tx_en(&self) {
        self.value
            .modify_bits(|r| r | Self::EP3_T_EN_RB_UH_EP_TX_EN_MASK);
    }

    /// Clear EP3_T_EN_RB_UH_EP_TX_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep3_t_en_rb_uh_ep_tx_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::EP3_T_EN_RB_UH_EP_TX_EN_MASK);
    }

    /// **EP3_BUF_MOD_RB_UH_EP_TBUF_MOD** `[4]` (RW)
    ///
    /// EP1_BUF_MOD_RB_UH_EP_TBUF_MOD
    #[inline(always)]
    pub fn ep3_buf_mod_rb_uh_ep_tbuf_mod(&self) -> bool {
        (self.value.read_bits() & Self::EP3_BUF_MOD_RB_UH_EP_TBUF_MOD_MASK) != 0
    }

    /// Set EP3_BUF_MOD_RB_UH_EP_TBUF_MOD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep3_buf_mod_rb_uh_ep_tbuf_mod(&self) {
        self.value
            .modify_bits(|r| r | Self::EP3_BUF_MOD_RB_UH_EP_TBUF_MOD_MASK);
    }

    /// Clear EP3_BUF_MOD_RB_UH_EP_TBUF_MOD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep3_buf_mod_rb_uh_ep_tbuf_mod(&self) {
        self.value
            .modify_bits(|r| r & !Self::EP3_BUF_MOD_RB_UH_EP_TBUF_MOD_MASK);
    }

    /// **EP2_R_EN_RB_UH_EP_RX_EN** `[3]` (RW)
    ///
    /// EP2_R_EN_RB_UH_EP_RX_EN
    #[inline(always)]
    pub fn ep2_r_en_rb_uh_ep_rx_en(&self) -> bool {
        (self.value.read_bits() & Self::EP2_R_EN_RB_UH_EP_RX_EN_MASK) != 0
    }

    /// Set EP2_R_EN_RB_UH_EP_RX_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep2_r_en_rb_uh_ep_rx_en(&self) {
        self.value
            .modify_bits(|r| r | Self::EP2_R_EN_RB_UH_EP_RX_EN_MASK);
    }

    /// Clear EP2_R_EN_RB_UH_EP_RX_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep2_r_en_rb_uh_ep_rx_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::EP2_R_EN_RB_UH_EP_RX_EN_MASK);
    }

    /// **EP2_T_EN** `[2]` (RW)
    ///
    /// EP2_T_EN
    #[inline(always)]
    pub fn ep2_t_en(&self) -> bool {
        (self.value.read_bits() & Self::EP2_T_EN_MASK) != 0
    }

    /// Set EP2_T_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep2_t_en(&self) {
        self.value.modify_bits(|r| r | Self::EP2_T_EN_MASK);
    }

    /// Clear EP2_T_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep2_t_en(&self) {
        self.value.modify_bits(|r| r & !Self::EP2_T_EN_MASK);
    }

    /// **EP2_BUF_MOD_RB_UH_EP_RBUF_MOD** `[0]` (RW)
    ///
    /// EP2_BUF_MOD_RB_UH_EP_RBUF_MOD
    #[inline(always)]
    pub fn ep2_buf_mod_rb_uh_ep_rbuf_mod(&self) -> bool {
        (self.value.read_bits() & Self::EP2_BUF_MOD_RB_UH_EP_RBUF_MOD_MASK) != 0
    }

    /// Set EP2_BUF_MOD_RB_UH_EP_RBUF_MOD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep2_buf_mod_rb_uh_ep_rbuf_mod(&self) {
        self.value
            .modify_bits(|r| r | Self::EP2_BUF_MOD_RB_UH_EP_RBUF_MOD_MASK);
    }

    /// Clear EP2_BUF_MOD_RB_UH_EP_RBUF_MOD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep2_buf_mod_rb_uh_ep_rbuf_mod(&self) {
        self.value
            .modify_bits(|r| r & !Self::EP2_BUF_MOD_RB_UH_EP_RBUF_MOD_MASK);
    }
}

/// UEP2_3_MOD_HOST_EP_MOD value type for batch read/modify/write operations
///
/// Use `uep23modhostepmod.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uep23modhostepmod.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uep23ModHostEpModVal {
    bits: u32,
}

impl Uep23ModHostEpModVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **EP3_R_EN** `[7:7]`
    ///
    /// EP1_R_EN
    #[inline(always)]
    pub const fn ep3_r_en(&self) -> bool {
        (self.bits & Uep23ModHostEpMod::EP3_R_EN_MASK) != 0
    }

    /// Read **EP3_T_EN_RB_UH_EP_TX_EN** `[6:6]`
    ///
    /// EP1_T_EN_RB_UH_EP_TX_EN
    #[inline(always)]
    pub const fn ep3_t_en_rb_uh_ep_tx_en(&self) -> bool {
        (self.bits & Uep23ModHostEpMod::EP3_T_EN_RB_UH_EP_TX_EN_MASK) != 0
    }

    /// Read **EP3_BUF_MOD_RB_UH_EP_TBUF_MOD** `[4:4]`
    ///
    /// EP1_BUF_MOD_RB_UH_EP_TBUF_MOD
    #[inline(always)]
    pub const fn ep3_buf_mod_rb_uh_ep_tbuf_mod(&self) -> bool {
        (self.bits & Uep23ModHostEpMod::EP3_BUF_MOD_RB_UH_EP_TBUF_MOD_MASK) != 0
    }

    /// Read **EP2_R_EN_RB_UH_EP_RX_EN** `[3:3]`
    ///
    /// EP2_R_EN_RB_UH_EP_RX_EN
    #[inline(always)]
    pub const fn ep2_r_en_rb_uh_ep_rx_en(&self) -> bool {
        (self.bits & Uep23ModHostEpMod::EP2_R_EN_RB_UH_EP_RX_EN_MASK) != 0
    }

    /// Read **EP2_T_EN** `[2:2]`
    ///
    /// EP2_T_EN
    #[inline(always)]
    pub const fn ep2_t_en(&self) -> bool {
        (self.bits & Uep23ModHostEpMod::EP2_T_EN_MASK) != 0
    }

    /// Read **EP2_BUF_MOD_RB_UH_EP_RBUF_MOD** `[0:0]`
    ///
    /// EP2_BUF_MOD_RB_UH_EP_RBUF_MOD
    #[inline(always)]
    pub const fn ep2_buf_mod_rb_uh_ep_rbuf_mod(&self) -> bool {
        (self.bits & Uep23ModHostEpMod::EP2_BUF_MOD_RB_UH_EP_RBUF_MOD_MASK) != 0
    }

    /// Set **EP3_R_EN** `[7:7]`
    ///
    /// EP1_R_EN
    #[inline(always)]
    pub const fn with_ep3_r_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep23ModHostEpMod::EP3_R_EN_MASK
            } else {
                self.bits & !Uep23ModHostEpMod::EP3_R_EN_MASK
            },
        }
    }

    /// Set **EP3_T_EN_RB_UH_EP_TX_EN** `[6:6]`
    ///
    /// EP1_T_EN_RB_UH_EP_TX_EN
    #[inline(always)]
    pub const fn with_ep3_t_en_rb_uh_ep_tx_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep23ModHostEpMod::EP3_T_EN_RB_UH_EP_TX_EN_MASK
            } else {
                self.bits & !Uep23ModHostEpMod::EP3_T_EN_RB_UH_EP_TX_EN_MASK
            },
        }
    }

    /// Set **EP3_BUF_MOD_RB_UH_EP_TBUF_MOD** `[4:4]`
    ///
    /// EP1_BUF_MOD_RB_UH_EP_TBUF_MOD
    #[inline(always)]
    pub const fn with_ep3_buf_mod_rb_uh_ep_tbuf_mod(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep23ModHostEpMod::EP3_BUF_MOD_RB_UH_EP_TBUF_MOD_MASK
            } else {
                self.bits & !Uep23ModHostEpMod::EP3_BUF_MOD_RB_UH_EP_TBUF_MOD_MASK
            },
        }
    }

    /// Set **EP2_R_EN_RB_UH_EP_RX_EN** `[3:3]`
    ///
    /// EP2_R_EN_RB_UH_EP_RX_EN
    #[inline(always)]
    pub const fn with_ep2_r_en_rb_uh_ep_rx_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep23ModHostEpMod::EP2_R_EN_RB_UH_EP_RX_EN_MASK
            } else {
                self.bits & !Uep23ModHostEpMod::EP2_R_EN_RB_UH_EP_RX_EN_MASK
            },
        }
    }

    /// Set **EP2_T_EN** `[2:2]`
    ///
    /// EP2_T_EN
    #[inline(always)]
    pub const fn with_ep2_t_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep23ModHostEpMod::EP2_T_EN_MASK
            } else {
                self.bits & !Uep23ModHostEpMod::EP2_T_EN_MASK
            },
        }
    }

    /// Set **EP2_BUF_MOD_RB_UH_EP_RBUF_MOD** `[0:0]`
    ///
    /// EP2_BUF_MOD_RB_UH_EP_RBUF_MOD
    #[inline(always)]
    pub const fn with_ep2_buf_mod_rb_uh_ep_rbuf_mod(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep23ModHostEpMod::EP2_BUF_MOD_RB_UH_EP_RBUF_MOD_MASK
            } else {
                self.bits & !Uep23ModHostEpMod::EP2_BUF_MOD_RB_UH_EP_RBUF_MOD_MASK
            },
        }
    }
}

impl Default for Uep23ModHostEpModVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uep23ModHostEpMod {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uep23ModHostEpModVal {
        Uep23ModHostEpModVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Uep23ModHostEpModVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Uep23ModHostEpModVal) -> Uep23ModHostEpModVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UEP567_MOD Register
///
/// UEP567_MOD
///
/// Address offset: 0x000e
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EP7_R_EN** `[5]` (RW)
///
///   EP7_R_EN
///
/// - **EP7_T_EN** `[4]` (RW)
///
///   EP7_T_EN
///
/// - **EP6_R_EN** `[3]` (RW)
///
///   EP6_R_EN
///
/// - **EP6_T_EN** `[2]` (RW)
///
///   EP6_T_EN
///
/// - **EP5_R_EN** `[1]` (RW)
///
///   EP5_R_EN
///
/// - **EP5_T_EN** `[0]` (RW)
///
///   EP5_T_EN
#[repr(transparent)]
pub struct Uep567Mod {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uep567Mod {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uep567Mod {
    // ======== Field Constants ========

    /// **EP7_R_EN**
    ///
    /// EP7_R_EN
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const EP7_R_EN_OFFSET: u32 = 5;
    pub const EP7_R_EN_MASK: u32 = 0x00000020;
    pub const EP7_R_EN: u32 = 1 << 5;

    /// **EP7_T_EN**
    ///
    /// EP7_T_EN
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const EP7_T_EN_OFFSET: u32 = 4;
    pub const EP7_T_EN_MASK: u32 = 0x00000010;
    pub const EP7_T_EN: u32 = 1 << 4;

    /// **EP6_R_EN**
    ///
    /// EP6_R_EN
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const EP6_R_EN_OFFSET: u32 = 3;
    pub const EP6_R_EN_MASK: u32 = 0x00000008;
    pub const EP6_R_EN: u32 = 1 << 3;

    /// **EP6_T_EN**
    ///
    /// EP6_T_EN
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const EP6_T_EN_OFFSET: u32 = 2;
    pub const EP6_T_EN_MASK: u32 = 0x00000004;
    pub const EP6_T_EN: u32 = 1 << 2;

    /// **EP5_R_EN**
    ///
    /// EP5_R_EN
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const EP5_R_EN_OFFSET: u32 = 1;
    pub const EP5_R_EN_MASK: u32 = 0x00000002;
    pub const EP5_R_EN: u32 = 1 << 1;

    /// **EP5_T_EN**
    ///
    /// EP5_T_EN
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EP5_T_EN_OFFSET: u32 = 0;
    pub const EP5_T_EN_MASK: u32 = 0x00000001;
    pub const EP5_T_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **EP7_R_EN** `[5]` (RW)
    ///
    /// EP7_R_EN
    #[inline(always)]
    pub fn ep7_r_en(&self) -> bool {
        (self.value.read_bits() & Self::EP7_R_EN_MASK) != 0
    }

    /// Set EP7_R_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep7_r_en(&self) {
        self.value.modify_bits(|r| r | Self::EP7_R_EN_MASK);
    }

    /// Clear EP7_R_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep7_r_en(&self) {
        self.value.modify_bits(|r| r & !Self::EP7_R_EN_MASK);
    }

    /// **EP7_T_EN** `[4]` (RW)
    ///
    /// EP7_T_EN
    #[inline(always)]
    pub fn ep7_t_en(&self) -> bool {
        (self.value.read_bits() & Self::EP7_T_EN_MASK) != 0
    }

    /// Set EP7_T_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep7_t_en(&self) {
        self.value.modify_bits(|r| r | Self::EP7_T_EN_MASK);
    }

    /// Clear EP7_T_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep7_t_en(&self) {
        self.value.modify_bits(|r| r & !Self::EP7_T_EN_MASK);
    }

    /// **EP6_R_EN** `[3]` (RW)
    ///
    /// EP6_R_EN
    #[inline(always)]
    pub fn ep6_r_en(&self) -> bool {
        (self.value.read_bits() & Self::EP6_R_EN_MASK) != 0
    }

    /// Set EP6_R_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep6_r_en(&self) {
        self.value.modify_bits(|r| r | Self::EP6_R_EN_MASK);
    }

    /// Clear EP6_R_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep6_r_en(&self) {
        self.value.modify_bits(|r| r & !Self::EP6_R_EN_MASK);
    }

    /// **EP6_T_EN** `[2]` (RW)
    ///
    /// EP6_T_EN
    #[inline(always)]
    pub fn ep6_t_en(&self) -> bool {
        (self.value.read_bits() & Self::EP6_T_EN_MASK) != 0
    }

    /// Set EP6_T_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep6_t_en(&self) {
        self.value.modify_bits(|r| r | Self::EP6_T_EN_MASK);
    }

    /// Clear EP6_T_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep6_t_en(&self) {
        self.value.modify_bits(|r| r & !Self::EP6_T_EN_MASK);
    }

    /// **EP5_R_EN** `[1]` (RW)
    ///
    /// EP5_R_EN
    #[inline(always)]
    pub fn ep5_r_en(&self) -> bool {
        (self.value.read_bits() & Self::EP5_R_EN_MASK) != 0
    }

    /// Set EP5_R_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep5_r_en(&self) {
        self.value.modify_bits(|r| r | Self::EP5_R_EN_MASK);
    }

    /// Clear EP5_R_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep5_r_en(&self) {
        self.value.modify_bits(|r| r & !Self::EP5_R_EN_MASK);
    }

    /// **EP5_T_EN** `[0]` (RW)
    ///
    /// EP5_T_EN
    #[inline(always)]
    pub fn ep5_t_en(&self) -> bool {
        (self.value.read_bits() & Self::EP5_T_EN_MASK) != 0
    }

    /// Set EP5_T_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep5_t_en(&self) {
        self.value.modify_bits(|r| r | Self::EP5_T_EN_MASK);
    }

    /// Clear EP5_T_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep5_t_en(&self) {
        self.value.modify_bits(|r| r & !Self::EP5_T_EN_MASK);
    }
}

/// UEP567_MOD value type for batch read/modify/write operations
///
/// Use `uep567mod.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uep567mod.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uep567ModVal {
    bits: u32,
}

impl Uep567ModVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **EP7_R_EN** `[5:5]`
    ///
    /// EP7_R_EN
    #[inline(always)]
    pub const fn ep7_r_en(&self) -> bool {
        (self.bits & Uep567Mod::EP7_R_EN_MASK) != 0
    }

    /// Read **EP7_T_EN** `[4:4]`
    ///
    /// EP7_T_EN
    #[inline(always)]
    pub const fn ep7_t_en(&self) -> bool {
        (self.bits & Uep567Mod::EP7_T_EN_MASK) != 0
    }

    /// Read **EP6_R_EN** `[3:3]`
    ///
    /// EP6_R_EN
    #[inline(always)]
    pub const fn ep6_r_en(&self) -> bool {
        (self.bits & Uep567Mod::EP6_R_EN_MASK) != 0
    }

    /// Read **EP6_T_EN** `[2:2]`
    ///
    /// EP6_T_EN
    #[inline(always)]
    pub const fn ep6_t_en(&self) -> bool {
        (self.bits & Uep567Mod::EP6_T_EN_MASK) != 0
    }

    /// Read **EP5_R_EN** `[1:1]`
    ///
    /// EP5_R_EN
    #[inline(always)]
    pub const fn ep5_r_en(&self) -> bool {
        (self.bits & Uep567Mod::EP5_R_EN_MASK) != 0
    }

    /// Read **EP5_T_EN** `[0:0]`
    ///
    /// EP5_T_EN
    #[inline(always)]
    pub const fn ep5_t_en(&self) -> bool {
        (self.bits & Uep567Mod::EP5_T_EN_MASK) != 0
    }

    /// Set **EP7_R_EN** `[5:5]`
    ///
    /// EP7_R_EN
    #[inline(always)]
    pub const fn with_ep7_r_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep567Mod::EP7_R_EN_MASK
            } else {
                self.bits & !Uep567Mod::EP7_R_EN_MASK
            },
        }
    }

    /// Set **EP7_T_EN** `[4:4]`
    ///
    /// EP7_T_EN
    #[inline(always)]
    pub const fn with_ep7_t_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep567Mod::EP7_T_EN_MASK
            } else {
                self.bits & !Uep567Mod::EP7_T_EN_MASK
            },
        }
    }

    /// Set **EP6_R_EN** `[3:3]`
    ///
    /// EP6_R_EN
    #[inline(always)]
    pub const fn with_ep6_r_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep567Mod::EP6_R_EN_MASK
            } else {
                self.bits & !Uep567Mod::EP6_R_EN_MASK
            },
        }
    }

    /// Set **EP6_T_EN** `[2:2]`
    ///
    /// EP6_T_EN
    #[inline(always)]
    pub const fn with_ep6_t_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep567Mod::EP6_T_EN_MASK
            } else {
                self.bits & !Uep567Mod::EP6_T_EN_MASK
            },
        }
    }

    /// Set **EP5_R_EN** `[1:1]`
    ///
    /// EP5_R_EN
    #[inline(always)]
    pub const fn with_ep5_r_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep567Mod::EP5_R_EN_MASK
            } else {
                self.bits & !Uep567Mod::EP5_R_EN_MASK
            },
        }
    }

    /// Set **EP5_T_EN** `[0:0]`
    ///
    /// EP5_T_EN
    #[inline(always)]
    pub const fn with_ep5_t_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep567Mod::EP5_T_EN_MASK
            } else {
                self.bits & !Uep567Mod::EP5_T_EN_MASK
            },
        }
    }
}

impl Default for Uep567ModVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uep567Mod {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uep567ModVal {
        Uep567ModVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Uep567ModVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Uep567ModVal) -> Uep567ModVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UEP0_DMA Register
///
/// UEP0_DMA
///
/// Address offset: 0x0010
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EP0_BUF_ADDR** `[14:0]` (RW)
///
///   EP0_BUF_ADDR
#[repr(transparent)]
pub struct Uep0Dma {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uep0Dma {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uep0Dma {
    // ======== Field Constants ========

    /// **EP0_BUF_ADDR**
    ///
    /// EP0_BUF_ADDR
    ///
    /// Bits `[14:0]`, mask `0x00007fff`
    pub const EP0_BUF_ADDR_OFFSET: u32 = 0;
    pub const EP0_BUF_ADDR_MASK: u32 = 0x00007fff;

    // ======== Field Accessors ========

    /// **EP0_BUF_ADDR** `[14:0]` (RW)
    ///
    /// EP0_BUF_ADDR
    #[inline(always)]
    pub fn ep0_buf_addr(&self) -> u32 {
        (self.value.read_bits() & Self::EP0_BUF_ADDR_MASK) >> Self::EP0_BUF_ADDR_OFFSET
    }

    /// Write **EP0_BUF_ADDR** `[14:0]` (RW)
    ///
    /// EP0_BUF_ADDR
    #[inline(always)]
    pub fn write_ep0_buf_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP0_BUF_ADDR_MASK)
                | ((value << Self::EP0_BUF_ADDR_OFFSET) & Self::EP0_BUF_ADDR_MASK)
        });
    }
}

/// UEP0_DMA value type for batch read/modify/write operations
///
/// Use `uep0dma.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uep0dma.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uep0DmaVal {
    bits: u32,
}

impl Uep0DmaVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **EP0_BUF_ADDR** `[14:0]`
    ///
    /// EP0_BUF_ADDR
    #[inline(always)]
    pub const fn ep0_buf_addr(&self) -> u32 {
        (self.bits & Uep0Dma::EP0_BUF_ADDR_MASK) >> Uep0Dma::EP0_BUF_ADDR_OFFSET
    }

    /// Set **EP0_BUF_ADDR** `[14:0]`
    ///
    /// EP0_BUF_ADDR
    #[inline(always)]
    pub const fn with_ep0_buf_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uep0Dma::EP0_BUF_ADDR_MASK)
                | ((value << Uep0Dma::EP0_BUF_ADDR_OFFSET) & Uep0Dma::EP0_BUF_ADDR_MASK),
        }
    }
}

impl Default for Uep0DmaVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uep0Dma {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uep0DmaVal {
        Uep0DmaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Uep0DmaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Uep0DmaVal) -> Uep0DmaVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UEP1_DMA Register
///
/// UEP1_DMA
///
/// Address offset: 0x0014
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EP1_BUF_ADDR** `[14:0]` (RW)
///
///   EP1_BUF_ADDR
#[repr(transparent)]
pub struct Uep1Dma {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uep1Dma {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uep1Dma {
    // ======== Field Constants ========

    /// **EP1_BUF_ADDR**
    ///
    /// EP1_BUF_ADDR
    ///
    /// Bits `[14:0]`, mask `0x00007fff`
    pub const EP1_BUF_ADDR_OFFSET: u32 = 0;
    pub const EP1_BUF_ADDR_MASK: u32 = 0x00007fff;

    // ======== Field Accessors ========

    /// **EP1_BUF_ADDR** `[14:0]` (RW)
    ///
    /// EP1_BUF_ADDR
    #[inline(always)]
    pub fn ep1_buf_addr(&self) -> u32 {
        (self.value.read_bits() & Self::EP1_BUF_ADDR_MASK) >> Self::EP1_BUF_ADDR_OFFSET
    }

    /// Write **EP1_BUF_ADDR** `[14:0]` (RW)
    ///
    /// EP1_BUF_ADDR
    #[inline(always)]
    pub fn write_ep1_buf_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP1_BUF_ADDR_MASK)
                | ((value << Self::EP1_BUF_ADDR_OFFSET) & Self::EP1_BUF_ADDR_MASK)
        });
    }
}

/// UEP1_DMA value type for batch read/modify/write operations
///
/// Use `uep1dma.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uep1dma.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uep1DmaVal {
    bits: u32,
}

impl Uep1DmaVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **EP1_BUF_ADDR** `[14:0]`
    ///
    /// EP1_BUF_ADDR
    #[inline(always)]
    pub const fn ep1_buf_addr(&self) -> u32 {
        (self.bits & Uep1Dma::EP1_BUF_ADDR_MASK) >> Uep1Dma::EP1_BUF_ADDR_OFFSET
    }

    /// Set **EP1_BUF_ADDR** `[14:0]`
    ///
    /// EP1_BUF_ADDR
    #[inline(always)]
    pub const fn with_ep1_buf_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uep1Dma::EP1_BUF_ADDR_MASK)
                | ((value << Uep1Dma::EP1_BUF_ADDR_OFFSET) & Uep1Dma::EP1_BUF_ADDR_MASK),
        }
    }
}

impl Default for Uep1DmaVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uep1Dma {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uep1DmaVal {
        Uep1DmaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Uep1DmaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Uep1DmaVal) -> Uep1DmaVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UEP2_DMA_HOST_RX_DMA Register
///
/// UEP2_DMA_HOST_RX_DMA
///
/// Address offset: 0x0018
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EP2_BUF_ADDR_UH_RX_DMA** `[14:0]` (RW)
///
///   EP2_BUF_ADDR_UH_RX_DMA
#[repr(transparent)]
pub struct Uep2DmaHostRxDma {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uep2DmaHostRxDma {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uep2DmaHostRxDma {
    // ======== Field Constants ========

    /// **EP2_BUF_ADDR_UH_RX_DMA**
    ///
    /// EP2_BUF_ADDR_UH_RX_DMA
    ///
    /// Bits `[14:0]`, mask `0x00007fff`
    pub const EP2_BUF_ADDR_UH_RX_DMA_OFFSET: u32 = 0;
    pub const EP2_BUF_ADDR_UH_RX_DMA_MASK: u32 = 0x00007fff;

    // ======== Field Accessors ========

    /// **EP2_BUF_ADDR_UH_RX_DMA** `[14:0]` (RW)
    ///
    /// EP2_BUF_ADDR_UH_RX_DMA
    #[inline(always)]
    pub fn ep2_buf_addr_uh_rx_dma(&self) -> u32 {
        (self.value.read_bits() & Self::EP2_BUF_ADDR_UH_RX_DMA_MASK)
            >> Self::EP2_BUF_ADDR_UH_RX_DMA_OFFSET
    }

    /// Write **EP2_BUF_ADDR_UH_RX_DMA** `[14:0]` (RW)
    ///
    /// EP2_BUF_ADDR_UH_RX_DMA
    #[inline(always)]
    pub fn write_ep2_buf_addr_uh_rx_dma(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP2_BUF_ADDR_UH_RX_DMA_MASK)
                | ((value << Self::EP2_BUF_ADDR_UH_RX_DMA_OFFSET)
                    & Self::EP2_BUF_ADDR_UH_RX_DMA_MASK)
        });
    }
}

/// UEP2_DMA_HOST_RX_DMA value type for batch read/modify/write operations
///
/// Use `uep2dmahostrxdma.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uep2dmahostrxdma.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uep2DmaHostRxDmaVal {
    bits: u32,
}

impl Uep2DmaHostRxDmaVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **EP2_BUF_ADDR_UH_RX_DMA** `[14:0]`
    ///
    /// EP2_BUF_ADDR_UH_RX_DMA
    #[inline(always)]
    pub const fn ep2_buf_addr_uh_rx_dma(&self) -> u32 {
        (self.bits & Uep2DmaHostRxDma::EP2_BUF_ADDR_UH_RX_DMA_MASK)
            >> Uep2DmaHostRxDma::EP2_BUF_ADDR_UH_RX_DMA_OFFSET
    }

    /// Set **EP2_BUF_ADDR_UH_RX_DMA** `[14:0]`
    ///
    /// EP2_BUF_ADDR_UH_RX_DMA
    #[inline(always)]
    pub const fn with_ep2_buf_addr_uh_rx_dma(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uep2DmaHostRxDma::EP2_BUF_ADDR_UH_RX_DMA_MASK)
                | ((value << Uep2DmaHostRxDma::EP2_BUF_ADDR_UH_RX_DMA_OFFSET)
                    & Uep2DmaHostRxDma::EP2_BUF_ADDR_UH_RX_DMA_MASK),
        }
    }
}

impl Default for Uep2DmaHostRxDmaVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uep2DmaHostRxDma {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uep2DmaHostRxDmaVal {
        Uep2DmaHostRxDmaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Uep2DmaHostRxDmaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Uep2DmaHostRxDmaVal) -> Uep2DmaHostRxDmaVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UEP3_DMA_HOST_TX_DMA Register
///
/// UEP3_DMA_HOST_TX_DMA
///
/// Address offset: 0x001c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EP3_BUF_ADDR_UH_TX_DMA** `[14:0]` (RW)
///
///   EP3_BUF_ADDR_UH_TX_DMA
#[repr(transparent)]
pub struct Uep3DmaHostTxDma {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uep3DmaHostTxDma {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uep3DmaHostTxDma {
    // ======== Field Constants ========

    /// **EP3_BUF_ADDR_UH_TX_DMA**
    ///
    /// EP3_BUF_ADDR_UH_TX_DMA
    ///
    /// Bits `[14:0]`, mask `0x00007fff`
    pub const EP3_BUF_ADDR_UH_TX_DMA_OFFSET: u32 = 0;
    pub const EP3_BUF_ADDR_UH_TX_DMA_MASK: u32 = 0x00007fff;

    // ======== Field Accessors ========

    /// **EP3_BUF_ADDR_UH_TX_DMA** `[14:0]` (RW)
    ///
    /// EP3_BUF_ADDR_UH_TX_DMA
    #[inline(always)]
    pub fn ep3_buf_addr_uh_tx_dma(&self) -> u32 {
        (self.value.read_bits() & Self::EP3_BUF_ADDR_UH_TX_DMA_MASK)
            >> Self::EP3_BUF_ADDR_UH_TX_DMA_OFFSET
    }

    /// Write **EP3_BUF_ADDR_UH_TX_DMA** `[14:0]` (RW)
    ///
    /// EP3_BUF_ADDR_UH_TX_DMA
    #[inline(always)]
    pub fn write_ep3_buf_addr_uh_tx_dma(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP3_BUF_ADDR_UH_TX_DMA_MASK)
                | ((value << Self::EP3_BUF_ADDR_UH_TX_DMA_OFFSET)
                    & Self::EP3_BUF_ADDR_UH_TX_DMA_MASK)
        });
    }
}

/// UEP3_DMA_HOST_TX_DMA value type for batch read/modify/write operations
///
/// Use `uep3dmahosttxdma.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uep3dmahosttxdma.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uep3DmaHostTxDmaVal {
    bits: u32,
}

impl Uep3DmaHostTxDmaVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **EP3_BUF_ADDR_UH_TX_DMA** `[14:0]`
    ///
    /// EP3_BUF_ADDR_UH_TX_DMA
    #[inline(always)]
    pub const fn ep3_buf_addr_uh_tx_dma(&self) -> u32 {
        (self.bits & Uep3DmaHostTxDma::EP3_BUF_ADDR_UH_TX_DMA_MASK)
            >> Uep3DmaHostTxDma::EP3_BUF_ADDR_UH_TX_DMA_OFFSET
    }

    /// Set **EP3_BUF_ADDR_UH_TX_DMA** `[14:0]`
    ///
    /// EP3_BUF_ADDR_UH_TX_DMA
    #[inline(always)]
    pub const fn with_ep3_buf_addr_uh_tx_dma(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uep3DmaHostTxDma::EP3_BUF_ADDR_UH_TX_DMA_MASK)
                | ((value << Uep3DmaHostTxDma::EP3_BUF_ADDR_UH_TX_DMA_OFFSET)
                    & Uep3DmaHostTxDma::EP3_BUF_ADDR_UH_TX_DMA_MASK),
        }
    }
}

impl Default for Uep3DmaHostTxDmaVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uep3DmaHostTxDma {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uep3DmaHostTxDmaVal {
        Uep3DmaHostTxDmaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Uep3DmaHostTxDmaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Uep3DmaHostTxDmaVal) -> Uep3DmaHostTxDmaVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UEP0_TX_LEN Register
///
/// UEP0_TX_LEN
///
/// Address offset: 0x0020
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EP0_T_LEN** `[6:0]` (RW)
///
///   EP0_T_LEN
#[repr(transparent)]
pub struct Uep0TxLen {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uep0TxLen {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uep0TxLen {
    // ======== Field Constants ========

    /// **EP0_T_LEN**
    ///
    /// EP0_T_LEN
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const EP0_T_LEN_OFFSET: u32 = 0;
    pub const EP0_T_LEN_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **EP0_T_LEN** `[6:0]` (RW)
    ///
    /// EP0_T_LEN
    #[inline(always)]
    pub fn ep0_t_len(&self) -> u32 {
        (self.value.read_bits() & Self::EP0_T_LEN_MASK) >> Self::EP0_T_LEN_OFFSET
    }

    /// Write **EP0_T_LEN** `[6:0]` (RW)
    ///
    /// EP0_T_LEN
    #[inline(always)]
    pub fn write_ep0_t_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP0_T_LEN_MASK) | ((value << Self::EP0_T_LEN_OFFSET) & Self::EP0_T_LEN_MASK)
        });
    }
}

/// UEP0_TX_LEN value type for batch read/modify/write operations
///
/// Use `uep0txlen.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uep0txlen.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uep0TxLenVal {
    bits: u32,
}

impl Uep0TxLenVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **EP0_T_LEN** `[6:0]`
    ///
    /// EP0_T_LEN
    #[inline(always)]
    pub const fn ep0_t_len(&self) -> u32 {
        (self.bits & Uep0TxLen::EP0_T_LEN_MASK) >> Uep0TxLen::EP0_T_LEN_OFFSET
    }

    /// Set **EP0_T_LEN** `[6:0]`
    ///
    /// EP0_T_LEN
    #[inline(always)]
    pub const fn with_ep0_t_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uep0TxLen::EP0_T_LEN_MASK)
                | ((value << Uep0TxLen::EP0_T_LEN_OFFSET) & Uep0TxLen::EP0_T_LEN_MASK),
        }
    }
}

impl Default for Uep0TxLenVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uep0TxLen {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uep0TxLenVal {
        Uep0TxLenVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Uep0TxLenVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Uep0TxLenVal) -> Uep0TxLenVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UEP0_CTRL_H Register
///
/// UEP0_CTRL_H
///
/// Address offset: 0x0022
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EP0_R_TOG** `[7]` (RW)
///
///   EP0_R_TOG
///
/// - **EP0_T_TOG** `[6]` (RW)
///
///   EP0_T_TOG
///
/// - **EP0_R_RES** `[3:2]` (RW)
///
///   EP0_R_RES
///
/// - **EP0_T_RES** `[1:0]` (RW)
///
///   EP0_T_RES
#[repr(transparent)]
pub struct Uep0CtrlH {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uep0CtrlH {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uep0CtrlH {
    // ======== Field Constants ========

    /// **EP0_R_TOG**
    ///
    /// EP0_R_TOG
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const EP0_R_TOG_OFFSET: u32 = 7;
    pub const EP0_R_TOG_MASK: u32 = 0x00000080;
    pub const EP0_R_TOG: u32 = 1 << 7;

    /// **EP0_T_TOG**
    ///
    /// EP0_T_TOG
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const EP0_T_TOG_OFFSET: u32 = 6;
    pub const EP0_T_TOG_MASK: u32 = 0x00000040;
    pub const EP0_T_TOG: u32 = 1 << 6;

    /// **EP0_R_RES**
    ///
    /// EP0_R_RES
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const EP0_R_RES_OFFSET: u32 = 2;
    pub const EP0_R_RES_MASK: u32 = 0x0000000c;

    /// **EP0_T_RES**
    ///
    /// EP0_T_RES
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const EP0_T_RES_OFFSET: u32 = 0;
    pub const EP0_T_RES_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **EP0_R_TOG** `[7]` (RW)
    ///
    /// EP0_R_TOG
    #[inline(always)]
    pub fn ep0_r_tog(&self) -> bool {
        (self.value.read_bits() & Self::EP0_R_TOG_MASK) != 0
    }

    /// Set EP0_R_TOG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep0_r_tog(&self) {
        self.value.modify_bits(|r| r | Self::EP0_R_TOG_MASK);
    }

    /// Clear EP0_R_TOG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep0_r_tog(&self) {
        self.value.modify_bits(|r| r & !Self::EP0_R_TOG_MASK);
    }

    /// **EP0_T_TOG** `[6]` (RW)
    ///
    /// EP0_T_TOG
    #[inline(always)]
    pub fn ep0_t_tog(&self) -> bool {
        (self.value.read_bits() & Self::EP0_T_TOG_MASK) != 0
    }

    /// Set EP0_T_TOG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep0_t_tog(&self) {
        self.value.modify_bits(|r| r | Self::EP0_T_TOG_MASK);
    }

    /// Clear EP0_T_TOG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep0_t_tog(&self) {
        self.value.modify_bits(|r| r & !Self::EP0_T_TOG_MASK);
    }

    /// **EP0_R_RES** `[3:2]` (RW)
    ///
    /// EP0_R_RES
    #[inline(always)]
    pub fn ep0_r_res(&self) -> u32 {
        (self.value.read_bits() & Self::EP0_R_RES_MASK) >> Self::EP0_R_RES_OFFSET
    }

    /// Write **EP0_R_RES** `[3:2]` (RW)
    ///
    /// EP0_R_RES
    #[inline(always)]
    pub fn write_ep0_r_res(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP0_R_RES_MASK) | ((value << Self::EP0_R_RES_OFFSET) & Self::EP0_R_RES_MASK)
        });
    }

    /// **EP0_T_RES** `[1:0]` (RW)
    ///
    /// EP0_T_RES
    #[inline(always)]
    pub fn ep0_t_res(&self) -> u32 {
        (self.value.read_bits() & Self::EP0_T_RES_MASK) >> Self::EP0_T_RES_OFFSET
    }

    /// Write **EP0_T_RES** `[1:0]` (RW)
    ///
    /// EP0_T_RES
    #[inline(always)]
    pub fn write_ep0_t_res(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP0_T_RES_MASK) | ((value << Self::EP0_T_RES_OFFSET) & Self::EP0_T_RES_MASK)
        });
    }
}

/// UEP0_CTRL_H value type for batch read/modify/write operations
///
/// Use `uep0ctrlh.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uep0ctrlh.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uep0CtrlHVal {
    bits: u32,
}

impl Uep0CtrlHVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **EP0_R_TOG** `[7:7]`
    ///
    /// EP0_R_TOG
    #[inline(always)]
    pub const fn ep0_r_tog(&self) -> bool {
        (self.bits & Uep0CtrlH::EP0_R_TOG_MASK) != 0
    }

    /// Read **EP0_T_TOG** `[6:6]`
    ///
    /// EP0_T_TOG
    #[inline(always)]
    pub const fn ep0_t_tog(&self) -> bool {
        (self.bits & Uep0CtrlH::EP0_T_TOG_MASK) != 0
    }

    /// Read **EP0_R_RES** `[3:2]`
    ///
    /// EP0_R_RES
    #[inline(always)]
    pub const fn ep0_r_res(&self) -> u32 {
        (self.bits & Uep0CtrlH::EP0_R_RES_MASK) >> Uep0CtrlH::EP0_R_RES_OFFSET
    }

    /// Read **EP0_T_RES** `[1:0]`
    ///
    /// EP0_T_RES
    #[inline(always)]
    pub const fn ep0_t_res(&self) -> u32 {
        (self.bits & Uep0CtrlH::EP0_T_RES_MASK) >> Uep0CtrlH::EP0_T_RES_OFFSET
    }

    /// Set **EP0_R_TOG** `[7:7]`
    ///
    /// EP0_R_TOG
    #[inline(always)]
    pub const fn with_ep0_r_tog(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep0CtrlH::EP0_R_TOG_MASK
            } else {
                self.bits & !Uep0CtrlH::EP0_R_TOG_MASK
            },
        }
    }

    /// Set **EP0_T_TOG** `[6:6]`
    ///
    /// EP0_T_TOG
    #[inline(always)]
    pub const fn with_ep0_t_tog(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep0CtrlH::EP0_T_TOG_MASK
            } else {
                self.bits & !Uep0CtrlH::EP0_T_TOG_MASK
            },
        }
    }

    /// Set **EP0_R_RES** `[3:2]`
    ///
    /// EP0_R_RES
    #[inline(always)]
    pub const fn with_ep0_r_res(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uep0CtrlH::EP0_R_RES_MASK)
                | ((value << Uep0CtrlH::EP0_R_RES_OFFSET) & Uep0CtrlH::EP0_R_RES_MASK),
        }
    }

    /// Set **EP0_T_RES** `[1:0]`
    ///
    /// EP0_T_RES
    #[inline(always)]
    pub const fn with_ep0_t_res(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uep0CtrlH::EP0_T_RES_MASK)
                | ((value << Uep0CtrlH::EP0_T_RES_OFFSET) & Uep0CtrlH::EP0_T_RES_MASK),
        }
    }
}

impl Default for Uep0CtrlHVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uep0CtrlH {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uep0CtrlHVal {
        Uep0CtrlHVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Uep0CtrlHVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Uep0CtrlHVal) -> Uep0CtrlHVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UEP1_TX_LEN Register
///
/// UEP1_TX_LEN
///
/// Address offset: 0x0024
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EP1_T_LEN** `[6:0]` (RW)
///
///   EP1_T_LEN
#[repr(transparent)]
pub struct Uep1TxLen {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uep1TxLen {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uep1TxLen {
    // ======== Field Constants ========

    /// **EP1_T_LEN**
    ///
    /// EP1_T_LEN
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const EP1_T_LEN_OFFSET: u32 = 0;
    pub const EP1_T_LEN_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **EP1_T_LEN** `[6:0]` (RW)
    ///
    /// EP1_T_LEN
    #[inline(always)]
    pub fn ep1_t_len(&self) -> u32 {
        (self.value.read_bits() & Self::EP1_T_LEN_MASK) >> Self::EP1_T_LEN_OFFSET
    }

    /// Write **EP1_T_LEN** `[6:0]` (RW)
    ///
    /// EP1_T_LEN
    #[inline(always)]
    pub fn write_ep1_t_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP1_T_LEN_MASK) | ((value << Self::EP1_T_LEN_OFFSET) & Self::EP1_T_LEN_MASK)
        });
    }
}

/// UEP1_TX_LEN value type for batch read/modify/write operations
///
/// Use `uep1txlen.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uep1txlen.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uep1TxLenVal {
    bits: u32,
}

impl Uep1TxLenVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **EP1_T_LEN** `[6:0]`
    ///
    /// EP1_T_LEN
    #[inline(always)]
    pub const fn ep1_t_len(&self) -> u32 {
        (self.bits & Uep1TxLen::EP1_T_LEN_MASK) >> Uep1TxLen::EP1_T_LEN_OFFSET
    }

    /// Set **EP1_T_LEN** `[6:0]`
    ///
    /// EP1_T_LEN
    #[inline(always)]
    pub const fn with_ep1_t_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uep1TxLen::EP1_T_LEN_MASK)
                | ((value << Uep1TxLen::EP1_T_LEN_OFFSET) & Uep1TxLen::EP1_T_LEN_MASK),
        }
    }
}

impl Default for Uep1TxLenVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uep1TxLen {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uep1TxLenVal {
        Uep1TxLenVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Uep1TxLenVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Uep1TxLenVal) -> Uep1TxLenVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UEP1_CTRL_H_HOST_SETUP Register
///
/// UEP1_CTRL_H_HOST_SETUP
///
/// Address offset: 0x0026
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EP1_R_TOG_RB_UH_PRE_PID_EN** `[7]` (RW)
///
///   EP1_R_TOG_RB_UH_PRE_PID_EN
///
/// - **EP1_T_TOG_RB_UH_SOF_EN** `[6]` (RW)
///
///   EP1_T_TOG_RB_UH_SOF_EN
///
/// - **EP1_TOG_AUTO** `[4]` (RW)
///
///   EP1_TOG_AUTO
///
/// - **EP1_R_RES** `[3:2]` (RW)
///
///   EP1_R_RES
///
/// - **EP1_T_RES** `[1:0]` (RW)
///
///   EP1_T_RES
#[repr(transparent)]
pub struct Uep1CtrlHHostSetup {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uep1CtrlHHostSetup {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uep1CtrlHHostSetup {
    // ======== Field Constants ========

    /// **EP1_R_TOG_RB_UH_PRE_PID_EN**
    ///
    /// EP1_R_TOG_RB_UH_PRE_PID_EN
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const EP1_R_TOG_RB_UH_PRE_PID_EN_OFFSET: u32 = 7;
    pub const EP1_R_TOG_RB_UH_PRE_PID_EN_MASK: u32 = 0x00000080;
    pub const EP1_R_TOG_RB_UH_PRE_PID_EN: u32 = 1 << 7;

    /// **EP1_T_TOG_RB_UH_SOF_EN**
    ///
    /// EP1_T_TOG_RB_UH_SOF_EN
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const EP1_T_TOG_RB_UH_SOF_EN_OFFSET: u32 = 6;
    pub const EP1_T_TOG_RB_UH_SOF_EN_MASK: u32 = 0x00000040;
    pub const EP1_T_TOG_RB_UH_SOF_EN: u32 = 1 << 6;

    /// **EP1_TOG_AUTO**
    ///
    /// EP1_TOG_AUTO
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const EP1_TOG_AUTO_OFFSET: u32 = 4;
    pub const EP1_TOG_AUTO_MASK: u32 = 0x00000010;
    pub const EP1_TOG_AUTO: u32 = 1 << 4;

    /// **EP1_R_RES**
    ///
    /// EP1_R_RES
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const EP1_R_RES_OFFSET: u32 = 2;
    pub const EP1_R_RES_MASK: u32 = 0x0000000c;

    /// **EP1_T_RES**
    ///
    /// EP1_T_RES
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const EP1_T_RES_OFFSET: u32 = 0;
    pub const EP1_T_RES_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **EP1_R_TOG_RB_UH_PRE_PID_EN** `[7]` (RW)
    ///
    /// EP1_R_TOG_RB_UH_PRE_PID_EN
    #[inline(always)]
    pub fn ep1_r_tog_rb_uh_pre_pid_en(&self) -> bool {
        (self.value.read_bits() & Self::EP1_R_TOG_RB_UH_PRE_PID_EN_MASK) != 0
    }

    /// Set EP1_R_TOG_RB_UH_PRE_PID_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep1_r_tog_rb_uh_pre_pid_en(&self) {
        self.value
            .modify_bits(|r| r | Self::EP1_R_TOG_RB_UH_PRE_PID_EN_MASK);
    }

    /// Clear EP1_R_TOG_RB_UH_PRE_PID_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep1_r_tog_rb_uh_pre_pid_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::EP1_R_TOG_RB_UH_PRE_PID_EN_MASK);
    }

    /// **EP1_T_TOG_RB_UH_SOF_EN** `[6]` (RW)
    ///
    /// EP1_T_TOG_RB_UH_SOF_EN
    #[inline(always)]
    pub fn ep1_t_tog_rb_uh_sof_en(&self) -> bool {
        (self.value.read_bits() & Self::EP1_T_TOG_RB_UH_SOF_EN_MASK) != 0
    }

    /// Set EP1_T_TOG_RB_UH_SOF_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep1_t_tog_rb_uh_sof_en(&self) {
        self.value
            .modify_bits(|r| r | Self::EP1_T_TOG_RB_UH_SOF_EN_MASK);
    }

    /// Clear EP1_T_TOG_RB_UH_SOF_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep1_t_tog_rb_uh_sof_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::EP1_T_TOG_RB_UH_SOF_EN_MASK);
    }

    /// **EP1_TOG_AUTO** `[4]` (RW)
    ///
    /// EP1_TOG_AUTO
    #[inline(always)]
    pub fn ep1_tog_auto(&self) -> bool {
        (self.value.read_bits() & Self::EP1_TOG_AUTO_MASK) != 0
    }

    /// Set EP1_TOG_AUTO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep1_tog_auto(&self) {
        self.value.modify_bits(|r| r | Self::EP1_TOG_AUTO_MASK);
    }

    /// Clear EP1_TOG_AUTO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep1_tog_auto(&self) {
        self.value.modify_bits(|r| r & !Self::EP1_TOG_AUTO_MASK);
    }

    /// **EP1_R_RES** `[3:2]` (RW)
    ///
    /// EP1_R_RES
    #[inline(always)]
    pub fn ep1_r_res(&self) -> u32 {
        (self.value.read_bits() & Self::EP1_R_RES_MASK) >> Self::EP1_R_RES_OFFSET
    }

    /// Write **EP1_R_RES** `[3:2]` (RW)
    ///
    /// EP1_R_RES
    #[inline(always)]
    pub fn write_ep1_r_res(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP1_R_RES_MASK) | ((value << Self::EP1_R_RES_OFFSET) & Self::EP1_R_RES_MASK)
        });
    }

    /// **EP1_T_RES** `[1:0]` (RW)
    ///
    /// EP1_T_RES
    #[inline(always)]
    pub fn ep1_t_res(&self) -> u32 {
        (self.value.read_bits() & Self::EP1_T_RES_MASK) >> Self::EP1_T_RES_OFFSET
    }

    /// Write **EP1_T_RES** `[1:0]` (RW)
    ///
    /// EP1_T_RES
    #[inline(always)]
    pub fn write_ep1_t_res(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP1_T_RES_MASK) | ((value << Self::EP1_T_RES_OFFSET) & Self::EP1_T_RES_MASK)
        });
    }
}

/// UEP1_CTRL_H_HOST_SETUP value type for batch read/modify/write operations
///
/// Use `uep1ctrlhhostsetup.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uep1ctrlhhostsetup.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uep1CtrlHHostSetupVal {
    bits: u32,
}

impl Uep1CtrlHHostSetupVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **EP1_R_TOG_RB_UH_PRE_PID_EN** `[7:7]`
    ///
    /// EP1_R_TOG_RB_UH_PRE_PID_EN
    #[inline(always)]
    pub const fn ep1_r_tog_rb_uh_pre_pid_en(&self) -> bool {
        (self.bits & Uep1CtrlHHostSetup::EP1_R_TOG_RB_UH_PRE_PID_EN_MASK) != 0
    }

    /// Read **EP1_T_TOG_RB_UH_SOF_EN** `[6:6]`
    ///
    /// EP1_T_TOG_RB_UH_SOF_EN
    #[inline(always)]
    pub const fn ep1_t_tog_rb_uh_sof_en(&self) -> bool {
        (self.bits & Uep1CtrlHHostSetup::EP1_T_TOG_RB_UH_SOF_EN_MASK) != 0
    }

    /// Read **EP1_TOG_AUTO** `[4:4]`
    ///
    /// EP1_TOG_AUTO
    #[inline(always)]
    pub const fn ep1_tog_auto(&self) -> bool {
        (self.bits & Uep1CtrlHHostSetup::EP1_TOG_AUTO_MASK) != 0
    }

    /// Read **EP1_R_RES** `[3:2]`
    ///
    /// EP1_R_RES
    #[inline(always)]
    pub const fn ep1_r_res(&self) -> u32 {
        (self.bits & Uep1CtrlHHostSetup::EP1_R_RES_MASK) >> Uep1CtrlHHostSetup::EP1_R_RES_OFFSET
    }

    /// Read **EP1_T_RES** `[1:0]`
    ///
    /// EP1_T_RES
    #[inline(always)]
    pub const fn ep1_t_res(&self) -> u32 {
        (self.bits & Uep1CtrlHHostSetup::EP1_T_RES_MASK) >> Uep1CtrlHHostSetup::EP1_T_RES_OFFSET
    }

    /// Set **EP1_R_TOG_RB_UH_PRE_PID_EN** `[7:7]`
    ///
    /// EP1_R_TOG_RB_UH_PRE_PID_EN
    #[inline(always)]
    pub const fn with_ep1_r_tog_rb_uh_pre_pid_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep1CtrlHHostSetup::EP1_R_TOG_RB_UH_PRE_PID_EN_MASK
            } else {
                self.bits & !Uep1CtrlHHostSetup::EP1_R_TOG_RB_UH_PRE_PID_EN_MASK
            },
        }
    }

    /// Set **EP1_T_TOG_RB_UH_SOF_EN** `[6:6]`
    ///
    /// EP1_T_TOG_RB_UH_SOF_EN
    #[inline(always)]
    pub const fn with_ep1_t_tog_rb_uh_sof_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep1CtrlHHostSetup::EP1_T_TOG_RB_UH_SOF_EN_MASK
            } else {
                self.bits & !Uep1CtrlHHostSetup::EP1_T_TOG_RB_UH_SOF_EN_MASK
            },
        }
    }

    /// Set **EP1_TOG_AUTO** `[4:4]`
    ///
    /// EP1_TOG_AUTO
    #[inline(always)]
    pub const fn with_ep1_tog_auto(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep1CtrlHHostSetup::EP1_TOG_AUTO_MASK
            } else {
                self.bits & !Uep1CtrlHHostSetup::EP1_TOG_AUTO_MASK
            },
        }
    }

    /// Set **EP1_R_RES** `[3:2]`
    ///
    /// EP1_R_RES
    #[inline(always)]
    pub const fn with_ep1_r_res(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uep1CtrlHHostSetup::EP1_R_RES_MASK)
                | ((value << Uep1CtrlHHostSetup::EP1_R_RES_OFFSET)
                    & Uep1CtrlHHostSetup::EP1_R_RES_MASK),
        }
    }

    /// Set **EP1_T_RES** `[1:0]`
    ///
    /// EP1_T_RES
    #[inline(always)]
    pub const fn with_ep1_t_res(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uep1CtrlHHostSetup::EP1_T_RES_MASK)
                | ((value << Uep1CtrlHHostSetup::EP1_T_RES_OFFSET)
                    & Uep1CtrlHHostSetup::EP1_T_RES_MASK),
        }
    }
}

impl Default for Uep1CtrlHHostSetupVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uep1CtrlHHostSetup {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uep1CtrlHHostSetupVal {
        Uep1CtrlHHostSetupVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Uep1CtrlHHostSetupVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Uep1CtrlHHostSetupVal) -> Uep1CtrlHHostSetupVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UEP2_TX_LEN Register
///
/// UEP2_TX_LEN
///
/// Address offset: 0x0028
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EP2_T_LEN** `[6:0]` (RW)
///
///   EP2_T_LEN
#[repr(transparent)]
pub struct Uep2TxLen {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uep2TxLen {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uep2TxLen {
    // ======== Field Constants ========

    /// **EP2_T_LEN**
    ///
    /// EP2_T_LEN
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const EP2_T_LEN_OFFSET: u32 = 0;
    pub const EP2_T_LEN_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **EP2_T_LEN** `[6:0]` (RW)
    ///
    /// EP2_T_LEN
    #[inline(always)]
    pub fn ep2_t_len(&self) -> u32 {
        (self.value.read_bits() & Self::EP2_T_LEN_MASK) >> Self::EP2_T_LEN_OFFSET
    }

    /// Write **EP2_T_LEN** `[6:0]` (RW)
    ///
    /// EP2_T_LEN
    #[inline(always)]
    pub fn write_ep2_t_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP2_T_LEN_MASK) | ((value << Self::EP2_T_LEN_OFFSET) & Self::EP2_T_LEN_MASK)
        });
    }
}

/// UEP2_TX_LEN value type for batch read/modify/write operations
///
/// Use `uep2txlen.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uep2txlen.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uep2TxLenVal {
    bits: u32,
}

impl Uep2TxLenVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **EP2_T_LEN** `[6:0]`
    ///
    /// EP2_T_LEN
    #[inline(always)]
    pub const fn ep2_t_len(&self) -> u32 {
        (self.bits & Uep2TxLen::EP2_T_LEN_MASK) >> Uep2TxLen::EP2_T_LEN_OFFSET
    }

    /// Set **EP2_T_LEN** `[6:0]`
    ///
    /// EP2_T_LEN
    #[inline(always)]
    pub const fn with_ep2_t_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uep2TxLen::EP2_T_LEN_MASK)
                | ((value << Uep2TxLen::EP2_T_LEN_OFFSET) & Uep2TxLen::EP2_T_LEN_MASK),
        }
    }
}

impl Default for Uep2TxLenVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uep2TxLen {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uep2TxLenVal {
        Uep2TxLenVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Uep2TxLenVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Uep2TxLenVal) -> Uep2TxLenVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UEP2_CTRL_H Register
///
/// UEP2_CTRL_H
///
/// Address offset: 0x002a
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EP2_R_TOG** `[7]` (RW)
///
///   EP2_R_TOG
///
/// - **EP2_T_TOG** `[6]` (RW)
///
///   EP2_T_TOG
///
/// - **EP2_TOG_AUTO** `[4]` (RW)
///
///   EP2_TOG_AUTO
///
/// - **EP2_R_RES** `[3:2]` (RW)
///
///   EP2_R_RES
///
/// - **EP2_T_RES** `[1:0]` (RW)
///
///   EP2_T_RES
#[repr(transparent)]
pub struct Uep2CtrlH {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uep2CtrlH {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uep2CtrlH {
    // ======== Field Constants ========

    /// **EP2_R_TOG**
    ///
    /// EP2_R_TOG
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const EP2_R_TOG_OFFSET: u32 = 7;
    pub const EP2_R_TOG_MASK: u32 = 0x00000080;
    pub const EP2_R_TOG: u32 = 1 << 7;

    /// **EP2_T_TOG**
    ///
    /// EP2_T_TOG
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const EP2_T_TOG_OFFSET: u32 = 6;
    pub const EP2_T_TOG_MASK: u32 = 0x00000040;
    pub const EP2_T_TOG: u32 = 1 << 6;

    /// **EP2_TOG_AUTO**
    ///
    /// EP2_TOG_AUTO
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const EP2_TOG_AUTO_OFFSET: u32 = 4;
    pub const EP2_TOG_AUTO_MASK: u32 = 0x00000010;
    pub const EP2_TOG_AUTO: u32 = 1 << 4;

    /// **EP2_R_RES**
    ///
    /// EP2_R_RES
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const EP2_R_RES_OFFSET: u32 = 2;
    pub const EP2_R_RES_MASK: u32 = 0x0000000c;

    /// **EP2_T_RES**
    ///
    /// EP2_T_RES
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const EP2_T_RES_OFFSET: u32 = 0;
    pub const EP2_T_RES_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **EP2_R_TOG** `[7]` (RW)
    ///
    /// EP2_R_TOG
    #[inline(always)]
    pub fn ep2_r_tog(&self) -> bool {
        (self.value.read_bits() & Self::EP2_R_TOG_MASK) != 0
    }

    /// Set EP2_R_TOG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep2_r_tog(&self) {
        self.value.modify_bits(|r| r | Self::EP2_R_TOG_MASK);
    }

    /// Clear EP2_R_TOG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep2_r_tog(&self) {
        self.value.modify_bits(|r| r & !Self::EP2_R_TOG_MASK);
    }

    /// **EP2_T_TOG** `[6]` (RW)
    ///
    /// EP2_T_TOG
    #[inline(always)]
    pub fn ep2_t_tog(&self) -> bool {
        (self.value.read_bits() & Self::EP2_T_TOG_MASK) != 0
    }

    /// Set EP2_T_TOG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep2_t_tog(&self) {
        self.value.modify_bits(|r| r | Self::EP2_T_TOG_MASK);
    }

    /// Clear EP2_T_TOG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep2_t_tog(&self) {
        self.value.modify_bits(|r| r & !Self::EP2_T_TOG_MASK);
    }

    /// **EP2_TOG_AUTO** `[4]` (RW)
    ///
    /// EP2_TOG_AUTO
    #[inline(always)]
    pub fn ep2_tog_auto(&self) -> bool {
        (self.value.read_bits() & Self::EP2_TOG_AUTO_MASK) != 0
    }

    /// Set EP2_TOG_AUTO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep2_tog_auto(&self) {
        self.value.modify_bits(|r| r | Self::EP2_TOG_AUTO_MASK);
    }

    /// Clear EP2_TOG_AUTO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep2_tog_auto(&self) {
        self.value.modify_bits(|r| r & !Self::EP2_TOG_AUTO_MASK);
    }

    /// **EP2_R_RES** `[3:2]` (RW)
    ///
    /// EP2_R_RES
    #[inline(always)]
    pub fn ep2_r_res(&self) -> u32 {
        (self.value.read_bits() & Self::EP2_R_RES_MASK) >> Self::EP2_R_RES_OFFSET
    }

    /// Write **EP2_R_RES** `[3:2]` (RW)
    ///
    /// EP2_R_RES
    #[inline(always)]
    pub fn write_ep2_r_res(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP2_R_RES_MASK) | ((value << Self::EP2_R_RES_OFFSET) & Self::EP2_R_RES_MASK)
        });
    }

    /// **EP2_T_RES** `[1:0]` (RW)
    ///
    /// EP2_T_RES
    #[inline(always)]
    pub fn ep2_t_res(&self) -> u32 {
        (self.value.read_bits() & Self::EP2_T_RES_MASK) >> Self::EP2_T_RES_OFFSET
    }

    /// Write **EP2_T_RES** `[1:0]` (RW)
    ///
    /// EP2_T_RES
    #[inline(always)]
    pub fn write_ep2_t_res(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP2_T_RES_MASK) | ((value << Self::EP2_T_RES_OFFSET) & Self::EP2_T_RES_MASK)
        });
    }
}

/// UEP2_CTRL_H value type for batch read/modify/write operations
///
/// Use `uep2ctrlh.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uep2ctrlh.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uep2CtrlHVal {
    bits: u32,
}

impl Uep2CtrlHVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **EP2_R_TOG** `[7:7]`
    ///
    /// EP2_R_TOG
    #[inline(always)]
    pub const fn ep2_r_tog(&self) -> bool {
        (self.bits & Uep2CtrlH::EP2_R_TOG_MASK) != 0
    }

    /// Read **EP2_T_TOG** `[6:6]`
    ///
    /// EP2_T_TOG
    #[inline(always)]
    pub const fn ep2_t_tog(&self) -> bool {
        (self.bits & Uep2CtrlH::EP2_T_TOG_MASK) != 0
    }

    /// Read **EP2_TOG_AUTO** `[4:4]`
    ///
    /// EP2_TOG_AUTO
    #[inline(always)]
    pub const fn ep2_tog_auto(&self) -> bool {
        (self.bits & Uep2CtrlH::EP2_TOG_AUTO_MASK) != 0
    }

    /// Read **EP2_R_RES** `[3:2]`
    ///
    /// EP2_R_RES
    #[inline(always)]
    pub const fn ep2_r_res(&self) -> u32 {
        (self.bits & Uep2CtrlH::EP2_R_RES_MASK) >> Uep2CtrlH::EP2_R_RES_OFFSET
    }

    /// Read **EP2_T_RES** `[1:0]`
    ///
    /// EP2_T_RES
    #[inline(always)]
    pub const fn ep2_t_res(&self) -> u32 {
        (self.bits & Uep2CtrlH::EP2_T_RES_MASK) >> Uep2CtrlH::EP2_T_RES_OFFSET
    }

    /// Set **EP2_R_TOG** `[7:7]`
    ///
    /// EP2_R_TOG
    #[inline(always)]
    pub const fn with_ep2_r_tog(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep2CtrlH::EP2_R_TOG_MASK
            } else {
                self.bits & !Uep2CtrlH::EP2_R_TOG_MASK
            },
        }
    }

    /// Set **EP2_T_TOG** `[6:6]`
    ///
    /// EP2_T_TOG
    #[inline(always)]
    pub const fn with_ep2_t_tog(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep2CtrlH::EP2_T_TOG_MASK
            } else {
                self.bits & !Uep2CtrlH::EP2_T_TOG_MASK
            },
        }
    }

    /// Set **EP2_TOG_AUTO** `[4:4]`
    ///
    /// EP2_TOG_AUTO
    #[inline(always)]
    pub const fn with_ep2_tog_auto(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep2CtrlH::EP2_TOG_AUTO_MASK
            } else {
                self.bits & !Uep2CtrlH::EP2_TOG_AUTO_MASK
            },
        }
    }

    /// Set **EP2_R_RES** `[3:2]`
    ///
    /// EP2_R_RES
    #[inline(always)]
    pub const fn with_ep2_r_res(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uep2CtrlH::EP2_R_RES_MASK)
                | ((value << Uep2CtrlH::EP2_R_RES_OFFSET) & Uep2CtrlH::EP2_R_RES_MASK),
        }
    }

    /// Set **EP2_T_RES** `[1:0]`
    ///
    /// EP2_T_RES
    #[inline(always)]
    pub const fn with_ep2_t_res(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uep2CtrlH::EP2_T_RES_MASK)
                | ((value << Uep2CtrlH::EP2_T_RES_OFFSET) & Uep2CtrlH::EP2_T_RES_MASK),
        }
    }
}

impl Default for Uep2CtrlHVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uep2CtrlH {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uep2CtrlHVal {
        Uep2CtrlHVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Uep2CtrlHVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Uep2CtrlHVal) -> Uep2CtrlHVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UEP3_TX_LEN Register
///
/// UEP3_TX_LEN
///
/// Address offset: 0x002c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EP3_T_LEN** `[6:0]` (RW)
///
///   EP3_T_LEN
#[repr(transparent)]
pub struct Uep3TxLen {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uep3TxLen {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uep3TxLen {
    // ======== Field Constants ========

    /// **EP3_T_LEN**
    ///
    /// EP3_T_LEN
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const EP3_T_LEN_OFFSET: u32 = 0;
    pub const EP3_T_LEN_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **EP3_T_LEN** `[6:0]` (RW)
    ///
    /// EP3_T_LEN
    #[inline(always)]
    pub fn ep3_t_len(&self) -> u32 {
        (self.value.read_bits() & Self::EP3_T_LEN_MASK) >> Self::EP3_T_LEN_OFFSET
    }

    /// Write **EP3_T_LEN** `[6:0]` (RW)
    ///
    /// EP3_T_LEN
    #[inline(always)]
    pub fn write_ep3_t_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP3_T_LEN_MASK) | ((value << Self::EP3_T_LEN_OFFSET) & Self::EP3_T_LEN_MASK)
        });
    }
}

/// UEP3_TX_LEN value type for batch read/modify/write operations
///
/// Use `uep3txlen.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uep3txlen.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uep3TxLenVal {
    bits: u32,
}

impl Uep3TxLenVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **EP3_T_LEN** `[6:0]`
    ///
    /// EP3_T_LEN
    #[inline(always)]
    pub const fn ep3_t_len(&self) -> u32 {
        (self.bits & Uep3TxLen::EP3_T_LEN_MASK) >> Uep3TxLen::EP3_T_LEN_OFFSET
    }

    /// Set **EP3_T_LEN** `[6:0]`
    ///
    /// EP3_T_LEN
    #[inline(always)]
    pub const fn with_ep3_t_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uep3TxLen::EP3_T_LEN_MASK)
                | ((value << Uep3TxLen::EP3_T_LEN_OFFSET) & Uep3TxLen::EP3_T_LEN_MASK),
        }
    }
}

impl Default for Uep3TxLenVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uep3TxLen {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uep3TxLenVal {
        Uep3TxLenVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Uep3TxLenVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Uep3TxLenVal) -> Uep3TxLenVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UEP3_CTRL_H Register
///
/// UEP3_CTRL_H
///
/// Address offset: 0x002e
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EP3_R_TOG** `[7]` (RW)
///
///   EP3_R_TOG
///
/// - **EP3_T_TOG** `[6]` (RW)
///
///   EP3_T_TOG
///
/// - **EP3_TOG_AUTO** `[4]` (RW)
///
///   EP3_TOG_AUTO
///
/// - **EP3_R_RES** `[3:2]` (RW)
///
///   EP3_R_RES
///
/// - **EP3_T_RES** `[1:0]` (RW)
///
///   EP3_T_RES
#[repr(transparent)]
pub struct Uep3CtrlH {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uep3CtrlH {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uep3CtrlH {
    // ======== Field Constants ========

    /// **EP3_R_TOG**
    ///
    /// EP3_R_TOG
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const EP3_R_TOG_OFFSET: u32 = 7;
    pub const EP3_R_TOG_MASK: u32 = 0x00000080;
    pub const EP3_R_TOG: u32 = 1 << 7;

    /// **EP3_T_TOG**
    ///
    /// EP3_T_TOG
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const EP3_T_TOG_OFFSET: u32 = 6;
    pub const EP3_T_TOG_MASK: u32 = 0x00000040;
    pub const EP3_T_TOG: u32 = 1 << 6;

    /// **EP3_TOG_AUTO**
    ///
    /// EP3_TOG_AUTO
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const EP3_TOG_AUTO_OFFSET: u32 = 4;
    pub const EP3_TOG_AUTO_MASK: u32 = 0x00000010;
    pub const EP3_TOG_AUTO: u32 = 1 << 4;

    /// **EP3_R_RES**
    ///
    /// EP3_R_RES
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const EP3_R_RES_OFFSET: u32 = 2;
    pub const EP3_R_RES_MASK: u32 = 0x0000000c;

    /// **EP3_T_RES**
    ///
    /// EP3_T_RES
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const EP3_T_RES_OFFSET: u32 = 0;
    pub const EP3_T_RES_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **EP3_R_TOG** `[7]` (RW)
    ///
    /// EP3_R_TOG
    #[inline(always)]
    pub fn ep3_r_tog(&self) -> bool {
        (self.value.read_bits() & Self::EP3_R_TOG_MASK) != 0
    }

    /// Set EP3_R_TOG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep3_r_tog(&self) {
        self.value.modify_bits(|r| r | Self::EP3_R_TOG_MASK);
    }

    /// Clear EP3_R_TOG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep3_r_tog(&self) {
        self.value.modify_bits(|r| r & !Self::EP3_R_TOG_MASK);
    }

    /// **EP3_T_TOG** `[6]` (RW)
    ///
    /// EP3_T_TOG
    #[inline(always)]
    pub fn ep3_t_tog(&self) -> bool {
        (self.value.read_bits() & Self::EP3_T_TOG_MASK) != 0
    }

    /// Set EP3_T_TOG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep3_t_tog(&self) {
        self.value.modify_bits(|r| r | Self::EP3_T_TOG_MASK);
    }

    /// Clear EP3_T_TOG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep3_t_tog(&self) {
        self.value.modify_bits(|r| r & !Self::EP3_T_TOG_MASK);
    }

    /// **EP3_TOG_AUTO** `[4]` (RW)
    ///
    /// EP3_TOG_AUTO
    #[inline(always)]
    pub fn ep3_tog_auto(&self) -> bool {
        (self.value.read_bits() & Self::EP3_TOG_AUTO_MASK) != 0
    }

    /// Set EP3_TOG_AUTO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep3_tog_auto(&self) {
        self.value.modify_bits(|r| r | Self::EP3_TOG_AUTO_MASK);
    }

    /// Clear EP3_TOG_AUTO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep3_tog_auto(&self) {
        self.value.modify_bits(|r| r & !Self::EP3_TOG_AUTO_MASK);
    }

    /// **EP3_R_RES** `[3:2]` (RW)
    ///
    /// EP3_R_RES
    #[inline(always)]
    pub fn ep3_r_res(&self) -> u32 {
        (self.value.read_bits() & Self::EP3_R_RES_MASK) >> Self::EP3_R_RES_OFFSET
    }

    /// Write **EP3_R_RES** `[3:2]` (RW)
    ///
    /// EP3_R_RES
    #[inline(always)]
    pub fn write_ep3_r_res(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP3_R_RES_MASK) | ((value << Self::EP3_R_RES_OFFSET) & Self::EP3_R_RES_MASK)
        });
    }

    /// **EP3_T_RES** `[1:0]` (RW)
    ///
    /// EP3_T_RES
    #[inline(always)]
    pub fn ep3_t_res(&self) -> u32 {
        (self.value.read_bits() & Self::EP3_T_RES_MASK) >> Self::EP3_T_RES_OFFSET
    }

    /// Write **EP3_T_RES** `[1:0]` (RW)
    ///
    /// EP3_T_RES
    #[inline(always)]
    pub fn write_ep3_t_res(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP3_T_RES_MASK) | ((value << Self::EP3_T_RES_OFFSET) & Self::EP3_T_RES_MASK)
        });
    }
}

/// UEP3_CTRL_H value type for batch read/modify/write operations
///
/// Use `uep3ctrlh.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uep3ctrlh.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uep3CtrlHVal {
    bits: u32,
}

impl Uep3CtrlHVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **EP3_R_TOG** `[7:7]`
    ///
    /// EP3_R_TOG
    #[inline(always)]
    pub const fn ep3_r_tog(&self) -> bool {
        (self.bits & Uep3CtrlH::EP3_R_TOG_MASK) != 0
    }

    /// Read **EP3_T_TOG** `[6:6]`
    ///
    /// EP3_T_TOG
    #[inline(always)]
    pub const fn ep3_t_tog(&self) -> bool {
        (self.bits & Uep3CtrlH::EP3_T_TOG_MASK) != 0
    }

    /// Read **EP3_TOG_AUTO** `[4:4]`
    ///
    /// EP3_TOG_AUTO
    #[inline(always)]
    pub const fn ep3_tog_auto(&self) -> bool {
        (self.bits & Uep3CtrlH::EP3_TOG_AUTO_MASK) != 0
    }

    /// Read **EP3_R_RES** `[3:2]`
    ///
    /// EP3_R_RES
    #[inline(always)]
    pub const fn ep3_r_res(&self) -> u32 {
        (self.bits & Uep3CtrlH::EP3_R_RES_MASK) >> Uep3CtrlH::EP3_R_RES_OFFSET
    }

    /// Read **EP3_T_RES** `[1:0]`
    ///
    /// EP3_T_RES
    #[inline(always)]
    pub const fn ep3_t_res(&self) -> u32 {
        (self.bits & Uep3CtrlH::EP3_T_RES_MASK) >> Uep3CtrlH::EP3_T_RES_OFFSET
    }

    /// Set **EP3_R_TOG** `[7:7]`
    ///
    /// EP3_R_TOG
    #[inline(always)]
    pub const fn with_ep3_r_tog(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep3CtrlH::EP3_R_TOG_MASK
            } else {
                self.bits & !Uep3CtrlH::EP3_R_TOG_MASK
            },
        }
    }

    /// Set **EP3_T_TOG** `[6:6]`
    ///
    /// EP3_T_TOG
    #[inline(always)]
    pub const fn with_ep3_t_tog(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep3CtrlH::EP3_T_TOG_MASK
            } else {
                self.bits & !Uep3CtrlH::EP3_T_TOG_MASK
            },
        }
    }

    /// Set **EP3_TOG_AUTO** `[4:4]`
    ///
    /// EP3_TOG_AUTO
    #[inline(always)]
    pub const fn with_ep3_tog_auto(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep3CtrlH::EP3_TOG_AUTO_MASK
            } else {
                self.bits & !Uep3CtrlH::EP3_TOG_AUTO_MASK
            },
        }
    }

    /// Set **EP3_R_RES** `[3:2]`
    ///
    /// EP3_R_RES
    #[inline(always)]
    pub const fn with_ep3_r_res(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uep3CtrlH::EP3_R_RES_MASK)
                | ((value << Uep3CtrlH::EP3_R_RES_OFFSET) & Uep3CtrlH::EP3_R_RES_MASK),
        }
    }

    /// Set **EP3_T_RES** `[1:0]`
    ///
    /// EP3_T_RES
    #[inline(always)]
    pub const fn with_ep3_t_res(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uep3CtrlH::EP3_T_RES_MASK)
                | ((value << Uep3CtrlH::EP3_T_RES_OFFSET) & Uep3CtrlH::EP3_T_RES_MASK),
        }
    }
}

impl Default for Uep3CtrlHVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uep3CtrlH {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uep3CtrlHVal {
        Uep3CtrlHVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Uep3CtrlHVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Uep3CtrlHVal) -> Uep3CtrlHVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UEP4_TX_LEN Register
///
/// UEP4_TX_LEN
///
/// Address offset: 0x0030
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EP4_T_LEN** `[6:0]` (RW)
///
///   EP4_T_LEN
#[repr(transparent)]
pub struct Uep4TxLen {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uep4TxLen {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uep4TxLen {
    // ======== Field Constants ========

    /// **EP4_T_LEN**
    ///
    /// EP4_T_LEN
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const EP4_T_LEN_OFFSET: u32 = 0;
    pub const EP4_T_LEN_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **EP4_T_LEN** `[6:0]` (RW)
    ///
    /// EP4_T_LEN
    #[inline(always)]
    pub fn ep4_t_len(&self) -> u32 {
        (self.value.read_bits() & Self::EP4_T_LEN_MASK) >> Self::EP4_T_LEN_OFFSET
    }

    /// Write **EP4_T_LEN** `[6:0]` (RW)
    ///
    /// EP4_T_LEN
    #[inline(always)]
    pub fn write_ep4_t_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP4_T_LEN_MASK) | ((value << Self::EP4_T_LEN_OFFSET) & Self::EP4_T_LEN_MASK)
        });
    }
}

/// UEP4_TX_LEN value type for batch read/modify/write operations
///
/// Use `uep4txlen.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uep4txlen.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uep4TxLenVal {
    bits: u32,
}

impl Uep4TxLenVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **EP4_T_LEN** `[6:0]`
    ///
    /// EP4_T_LEN
    #[inline(always)]
    pub const fn ep4_t_len(&self) -> u32 {
        (self.bits & Uep4TxLen::EP4_T_LEN_MASK) >> Uep4TxLen::EP4_T_LEN_OFFSET
    }

    /// Set **EP4_T_LEN** `[6:0]`
    ///
    /// EP4_T_LEN
    #[inline(always)]
    pub const fn with_ep4_t_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uep4TxLen::EP4_T_LEN_MASK)
                | ((value << Uep4TxLen::EP4_T_LEN_OFFSET) & Uep4TxLen::EP4_T_LEN_MASK),
        }
    }
}

impl Default for Uep4TxLenVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uep4TxLen {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uep4TxLenVal {
        Uep4TxLenVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Uep4TxLenVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Uep4TxLenVal) -> Uep4TxLenVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UEP4_CTRL_H Register
///
/// UEP4_CTRL_H
///
/// Address offset: 0x0032
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EP4_R_TOG** `[7]` (RW)
///
///   EP4_R_TOG
///
/// - **EP4_T_TOG** `[6]` (RW)
///
///   EP4_T_TOG
///
/// - **EP4_R_RES** `[3:2]` (RW)
///
///   EP4_R_RES
///
/// - **EP4_T_RES** `[1:0]` (RW)
///
///   EP4_T_RES
#[repr(transparent)]
pub struct Uep4CtrlH {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uep4CtrlH {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uep4CtrlH {
    // ======== Field Constants ========

    /// **EP4_R_TOG**
    ///
    /// EP4_R_TOG
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const EP4_R_TOG_OFFSET: u32 = 7;
    pub const EP4_R_TOG_MASK: u32 = 0x00000080;
    pub const EP4_R_TOG: u32 = 1 << 7;

    /// **EP4_T_TOG**
    ///
    /// EP4_T_TOG
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const EP4_T_TOG_OFFSET: u32 = 6;
    pub const EP4_T_TOG_MASK: u32 = 0x00000040;
    pub const EP4_T_TOG: u32 = 1 << 6;

    /// **EP4_R_RES**
    ///
    /// EP4_R_RES
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const EP4_R_RES_OFFSET: u32 = 2;
    pub const EP4_R_RES_MASK: u32 = 0x0000000c;

    /// **EP4_T_RES**
    ///
    /// EP4_T_RES
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const EP4_T_RES_OFFSET: u32 = 0;
    pub const EP4_T_RES_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **EP4_R_TOG** `[7]` (RW)
    ///
    /// EP4_R_TOG
    #[inline(always)]
    pub fn ep4_r_tog(&self) -> bool {
        (self.value.read_bits() & Self::EP4_R_TOG_MASK) != 0
    }

    /// Set EP4_R_TOG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep4_r_tog(&self) {
        self.value.modify_bits(|r| r | Self::EP4_R_TOG_MASK);
    }

    /// Clear EP4_R_TOG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep4_r_tog(&self) {
        self.value.modify_bits(|r| r & !Self::EP4_R_TOG_MASK);
    }

    /// **EP4_T_TOG** `[6]` (RW)
    ///
    /// EP4_T_TOG
    #[inline(always)]
    pub fn ep4_t_tog(&self) -> bool {
        (self.value.read_bits() & Self::EP4_T_TOG_MASK) != 0
    }

    /// Set EP4_T_TOG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep4_t_tog(&self) {
        self.value.modify_bits(|r| r | Self::EP4_T_TOG_MASK);
    }

    /// Clear EP4_T_TOG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep4_t_tog(&self) {
        self.value.modify_bits(|r| r & !Self::EP4_T_TOG_MASK);
    }

    /// **EP4_R_RES** `[3:2]` (RW)
    ///
    /// EP4_R_RES
    #[inline(always)]
    pub fn ep4_r_res(&self) -> u32 {
        (self.value.read_bits() & Self::EP4_R_RES_MASK) >> Self::EP4_R_RES_OFFSET
    }

    /// Write **EP4_R_RES** `[3:2]` (RW)
    ///
    /// EP4_R_RES
    #[inline(always)]
    pub fn write_ep4_r_res(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP4_R_RES_MASK) | ((value << Self::EP4_R_RES_OFFSET) & Self::EP4_R_RES_MASK)
        });
    }

    /// **EP4_T_RES** `[1:0]` (RW)
    ///
    /// EP4_T_RES
    #[inline(always)]
    pub fn ep4_t_res(&self) -> u32 {
        (self.value.read_bits() & Self::EP4_T_RES_MASK) >> Self::EP4_T_RES_OFFSET
    }

    /// Write **EP4_T_RES** `[1:0]` (RW)
    ///
    /// EP4_T_RES
    #[inline(always)]
    pub fn write_ep4_t_res(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP4_T_RES_MASK) | ((value << Self::EP4_T_RES_OFFSET) & Self::EP4_T_RES_MASK)
        });
    }
}

/// UEP4_CTRL_H value type for batch read/modify/write operations
///
/// Use `uep4ctrlh.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uep4ctrlh.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uep4CtrlHVal {
    bits: u32,
}

impl Uep4CtrlHVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **EP4_R_TOG** `[7:7]`
    ///
    /// EP4_R_TOG
    #[inline(always)]
    pub const fn ep4_r_tog(&self) -> bool {
        (self.bits & Uep4CtrlH::EP4_R_TOG_MASK) != 0
    }

    /// Read **EP4_T_TOG** `[6:6]`
    ///
    /// EP4_T_TOG
    #[inline(always)]
    pub const fn ep4_t_tog(&self) -> bool {
        (self.bits & Uep4CtrlH::EP4_T_TOG_MASK) != 0
    }

    /// Read **EP4_R_RES** `[3:2]`
    ///
    /// EP4_R_RES
    #[inline(always)]
    pub const fn ep4_r_res(&self) -> u32 {
        (self.bits & Uep4CtrlH::EP4_R_RES_MASK) >> Uep4CtrlH::EP4_R_RES_OFFSET
    }

    /// Read **EP4_T_RES** `[1:0]`
    ///
    /// EP4_T_RES
    #[inline(always)]
    pub const fn ep4_t_res(&self) -> u32 {
        (self.bits & Uep4CtrlH::EP4_T_RES_MASK) >> Uep4CtrlH::EP4_T_RES_OFFSET
    }

    /// Set **EP4_R_TOG** `[7:7]`
    ///
    /// EP4_R_TOG
    #[inline(always)]
    pub const fn with_ep4_r_tog(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep4CtrlH::EP4_R_TOG_MASK
            } else {
                self.bits & !Uep4CtrlH::EP4_R_TOG_MASK
            },
        }
    }

    /// Set **EP4_T_TOG** `[6:6]`
    ///
    /// EP4_T_TOG
    #[inline(always)]
    pub const fn with_ep4_t_tog(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep4CtrlH::EP4_T_TOG_MASK
            } else {
                self.bits & !Uep4CtrlH::EP4_T_TOG_MASK
            },
        }
    }

    /// Set **EP4_R_RES** `[3:2]`
    ///
    /// EP4_R_RES
    #[inline(always)]
    pub const fn with_ep4_r_res(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uep4CtrlH::EP4_R_RES_MASK)
                | ((value << Uep4CtrlH::EP4_R_RES_OFFSET) & Uep4CtrlH::EP4_R_RES_MASK),
        }
    }

    /// Set **EP4_T_RES** `[1:0]`
    ///
    /// EP4_T_RES
    #[inline(always)]
    pub const fn with_ep4_t_res(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uep4CtrlH::EP4_T_RES_MASK)
                | ((value << Uep4CtrlH::EP4_T_RES_OFFSET) & Uep4CtrlH::EP4_T_RES_MASK),
        }
    }
}

impl Default for Uep4CtrlHVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uep4CtrlH {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uep4CtrlHVal {
        Uep4CtrlHVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Uep4CtrlHVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Uep4CtrlHVal) -> Uep4CtrlHVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UEP5_DMA Register
///
/// UEP5_DMA
///
/// Address offset: 0x0054
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EP5_BUF_ADDR** `[14:0]` (RW)
///
///   EP5_BUF_ADDR
#[repr(transparent)]
pub struct Uep5Dma {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uep5Dma {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uep5Dma {
    // ======== Field Constants ========

    /// **EP5_BUF_ADDR**
    ///
    /// EP5_BUF_ADDR
    ///
    /// Bits `[14:0]`, mask `0x00007fff`
    pub const EP5_BUF_ADDR_OFFSET: u32 = 0;
    pub const EP5_BUF_ADDR_MASK: u32 = 0x00007fff;

    // ======== Field Accessors ========

    /// **EP5_BUF_ADDR** `[14:0]` (RW)
    ///
    /// EP5_BUF_ADDR
    #[inline(always)]
    pub fn ep5_buf_addr(&self) -> u32 {
        (self.value.read_bits() & Self::EP5_BUF_ADDR_MASK) >> Self::EP5_BUF_ADDR_OFFSET
    }

    /// Write **EP5_BUF_ADDR** `[14:0]` (RW)
    ///
    /// EP5_BUF_ADDR
    #[inline(always)]
    pub fn write_ep5_buf_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP5_BUF_ADDR_MASK)
                | ((value << Self::EP5_BUF_ADDR_OFFSET) & Self::EP5_BUF_ADDR_MASK)
        });
    }
}

/// UEP5_DMA value type for batch read/modify/write operations
///
/// Use `uep5dma.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uep5dma.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uep5DmaVal {
    bits: u32,
}

impl Uep5DmaVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **EP5_BUF_ADDR** `[14:0]`
    ///
    /// EP5_BUF_ADDR
    #[inline(always)]
    pub const fn ep5_buf_addr(&self) -> u32 {
        (self.bits & Uep5Dma::EP5_BUF_ADDR_MASK) >> Uep5Dma::EP5_BUF_ADDR_OFFSET
    }

    /// Set **EP5_BUF_ADDR** `[14:0]`
    ///
    /// EP5_BUF_ADDR
    #[inline(always)]
    pub const fn with_ep5_buf_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uep5Dma::EP5_BUF_ADDR_MASK)
                | ((value << Uep5Dma::EP5_BUF_ADDR_OFFSET) & Uep5Dma::EP5_BUF_ADDR_MASK),
        }
    }
}

impl Default for Uep5DmaVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uep5Dma {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uep5DmaVal {
        Uep5DmaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Uep5DmaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Uep5DmaVal) -> Uep5DmaVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UEP6_DMA Register
///
/// UEP6_DMA
///
/// Address offset: 0x0058
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EP6_BUF_ADDR** `[14:0]` (RW)
///
///   EP6_BUF_ADDR
#[repr(transparent)]
pub struct Uep6Dma {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uep6Dma {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uep6Dma {
    // ======== Field Constants ========

    /// **EP6_BUF_ADDR**
    ///
    /// EP6_BUF_ADDR
    ///
    /// Bits `[14:0]`, mask `0x00007fff`
    pub const EP6_BUF_ADDR_OFFSET: u32 = 0;
    pub const EP6_BUF_ADDR_MASK: u32 = 0x00007fff;

    // ======== Field Accessors ========

    /// **EP6_BUF_ADDR** `[14:0]` (RW)
    ///
    /// EP6_BUF_ADDR
    #[inline(always)]
    pub fn ep6_buf_addr(&self) -> u32 {
        (self.value.read_bits() & Self::EP6_BUF_ADDR_MASK) >> Self::EP6_BUF_ADDR_OFFSET
    }

    /// Write **EP6_BUF_ADDR** `[14:0]` (RW)
    ///
    /// EP6_BUF_ADDR
    #[inline(always)]
    pub fn write_ep6_buf_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP6_BUF_ADDR_MASK)
                | ((value << Self::EP6_BUF_ADDR_OFFSET) & Self::EP6_BUF_ADDR_MASK)
        });
    }
}

/// UEP6_DMA value type for batch read/modify/write operations
///
/// Use `uep6dma.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uep6dma.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uep6DmaVal {
    bits: u32,
}

impl Uep6DmaVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **EP6_BUF_ADDR** `[14:0]`
    ///
    /// EP6_BUF_ADDR
    #[inline(always)]
    pub const fn ep6_buf_addr(&self) -> u32 {
        (self.bits & Uep6Dma::EP6_BUF_ADDR_MASK) >> Uep6Dma::EP6_BUF_ADDR_OFFSET
    }

    /// Set **EP6_BUF_ADDR** `[14:0]`
    ///
    /// EP6_BUF_ADDR
    #[inline(always)]
    pub const fn with_ep6_buf_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uep6Dma::EP6_BUF_ADDR_MASK)
                | ((value << Uep6Dma::EP6_BUF_ADDR_OFFSET) & Uep6Dma::EP6_BUF_ADDR_MASK),
        }
    }
}

impl Default for Uep6DmaVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uep6Dma {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uep6DmaVal {
        Uep6DmaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Uep6DmaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Uep6DmaVal) -> Uep6DmaVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UEP7_DMA Register
///
/// UEP7_DMA
///
/// Address offset: 0x005c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EP7_BUF_ADDR** `[14:0]` (RW)
///
///   EP7_BUF_ADDR
#[repr(transparent)]
pub struct Uep7Dma {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uep7Dma {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uep7Dma {
    // ======== Field Constants ========

    /// **EP7_BUF_ADDR**
    ///
    /// EP7_BUF_ADDR
    ///
    /// Bits `[14:0]`, mask `0x00007fff`
    pub const EP7_BUF_ADDR_OFFSET: u32 = 0;
    pub const EP7_BUF_ADDR_MASK: u32 = 0x00007fff;

    // ======== Field Accessors ========

    /// **EP7_BUF_ADDR** `[14:0]` (RW)
    ///
    /// EP7_BUF_ADDR
    #[inline(always)]
    pub fn ep7_buf_addr(&self) -> u32 {
        (self.value.read_bits() & Self::EP7_BUF_ADDR_MASK) >> Self::EP7_BUF_ADDR_OFFSET
    }

    /// Write **EP7_BUF_ADDR** `[14:0]` (RW)
    ///
    /// EP7_BUF_ADDR
    #[inline(always)]
    pub fn write_ep7_buf_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP7_BUF_ADDR_MASK)
                | ((value << Self::EP7_BUF_ADDR_OFFSET) & Self::EP7_BUF_ADDR_MASK)
        });
    }
}

/// UEP7_DMA value type for batch read/modify/write operations
///
/// Use `uep7dma.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uep7dma.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uep7DmaVal {
    bits: u32,
}

impl Uep7DmaVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **EP7_BUF_ADDR** `[14:0]`
    ///
    /// EP7_BUF_ADDR
    #[inline(always)]
    pub const fn ep7_buf_addr(&self) -> u32 {
        (self.bits & Uep7Dma::EP7_BUF_ADDR_MASK) >> Uep7Dma::EP7_BUF_ADDR_OFFSET
    }

    /// Set **EP7_BUF_ADDR** `[14:0]`
    ///
    /// EP7_BUF_ADDR
    #[inline(always)]
    pub const fn with_ep7_buf_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uep7Dma::EP7_BUF_ADDR_MASK)
                | ((value << Uep7Dma::EP7_BUF_ADDR_OFFSET) & Uep7Dma::EP7_BUF_ADDR_MASK),
        }
    }
}

impl Default for Uep7DmaVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uep7Dma {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uep7DmaVal {
        Uep7DmaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Uep7DmaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Uep7DmaVal) -> Uep7DmaVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UEP5_TX_LEN Register
///
/// UEP5_TX_LEN
///
/// Address offset: 0x0064
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EP5_T_LEN** `[6:0]` (RW)
///
///   EP5_T_LEN
#[repr(transparent)]
pub struct Uep5TxLen {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uep5TxLen {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uep5TxLen {
    // ======== Field Constants ========

    /// **EP5_T_LEN**
    ///
    /// EP5_T_LEN
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const EP5_T_LEN_OFFSET: u32 = 0;
    pub const EP5_T_LEN_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **EP5_T_LEN** `[6:0]` (RW)
    ///
    /// EP5_T_LEN
    #[inline(always)]
    pub fn ep5_t_len(&self) -> u32 {
        (self.value.read_bits() & Self::EP5_T_LEN_MASK) >> Self::EP5_T_LEN_OFFSET
    }

    /// Write **EP5_T_LEN** `[6:0]` (RW)
    ///
    /// EP5_T_LEN
    #[inline(always)]
    pub fn write_ep5_t_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP5_T_LEN_MASK) | ((value << Self::EP5_T_LEN_OFFSET) & Self::EP5_T_LEN_MASK)
        });
    }
}

/// UEP5_TX_LEN value type for batch read/modify/write operations
///
/// Use `uep5txlen.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uep5txlen.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uep5TxLenVal {
    bits: u32,
}

impl Uep5TxLenVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **EP5_T_LEN** `[6:0]`
    ///
    /// EP5_T_LEN
    #[inline(always)]
    pub const fn ep5_t_len(&self) -> u32 {
        (self.bits & Uep5TxLen::EP5_T_LEN_MASK) >> Uep5TxLen::EP5_T_LEN_OFFSET
    }

    /// Set **EP5_T_LEN** `[6:0]`
    ///
    /// EP5_T_LEN
    #[inline(always)]
    pub const fn with_ep5_t_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uep5TxLen::EP5_T_LEN_MASK)
                | ((value << Uep5TxLen::EP5_T_LEN_OFFSET) & Uep5TxLen::EP5_T_LEN_MASK),
        }
    }
}

impl Default for Uep5TxLenVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uep5TxLen {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uep5TxLenVal {
        Uep5TxLenVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Uep5TxLenVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Uep5TxLenVal) -> Uep5TxLenVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UEP5_CTRL_H Register
///
/// UEP5_CTRL_H
///
/// Address offset: 0x0066
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EP5_R_TOG** `[7]` (RW)
///
///   EP5_R_TOG
///
/// - **EP5_T_TOG** `[6]` (RW)
///
///   EP5_T_TOG
///
/// - **EP5_TOG_AUTO** `[4]` (RW)
///
///   EP5_TOG_AUTO
///
/// - **EP5_R_RES** `[3:2]` (RW)
///
///   EP5_R_RES
///
/// - **EP5_T_RES** `[1:0]` (RW)
///
///   EP5_T_RES
#[repr(transparent)]
pub struct Uep5CtrlH {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uep5CtrlH {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uep5CtrlH {
    // ======== Field Constants ========

    /// **EP5_R_TOG**
    ///
    /// EP5_R_TOG
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const EP5_R_TOG_OFFSET: u32 = 7;
    pub const EP5_R_TOG_MASK: u32 = 0x00000080;
    pub const EP5_R_TOG: u32 = 1 << 7;

    /// **EP5_T_TOG**
    ///
    /// EP5_T_TOG
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const EP5_T_TOG_OFFSET: u32 = 6;
    pub const EP5_T_TOG_MASK: u32 = 0x00000040;
    pub const EP5_T_TOG: u32 = 1 << 6;

    /// **EP5_TOG_AUTO**
    ///
    /// EP5_TOG_AUTO
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const EP5_TOG_AUTO_OFFSET: u32 = 4;
    pub const EP5_TOG_AUTO_MASK: u32 = 0x00000010;
    pub const EP5_TOG_AUTO: u32 = 1 << 4;

    /// **EP5_R_RES**
    ///
    /// EP5_R_RES
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const EP5_R_RES_OFFSET: u32 = 2;
    pub const EP5_R_RES_MASK: u32 = 0x0000000c;

    /// **EP5_T_RES**
    ///
    /// EP5_T_RES
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const EP5_T_RES_OFFSET: u32 = 0;
    pub const EP5_T_RES_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **EP5_R_TOG** `[7]` (RW)
    ///
    /// EP5_R_TOG
    #[inline(always)]
    pub fn ep5_r_tog(&self) -> bool {
        (self.value.read_bits() & Self::EP5_R_TOG_MASK) != 0
    }

    /// Set EP5_R_TOG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep5_r_tog(&self) {
        self.value.modify_bits(|r| r | Self::EP5_R_TOG_MASK);
    }

    /// Clear EP5_R_TOG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep5_r_tog(&self) {
        self.value.modify_bits(|r| r & !Self::EP5_R_TOG_MASK);
    }

    /// **EP5_T_TOG** `[6]` (RW)
    ///
    /// EP5_T_TOG
    #[inline(always)]
    pub fn ep5_t_tog(&self) -> bool {
        (self.value.read_bits() & Self::EP5_T_TOG_MASK) != 0
    }

    /// Set EP5_T_TOG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep5_t_tog(&self) {
        self.value.modify_bits(|r| r | Self::EP5_T_TOG_MASK);
    }

    /// Clear EP5_T_TOG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep5_t_tog(&self) {
        self.value.modify_bits(|r| r & !Self::EP5_T_TOG_MASK);
    }

    /// **EP5_TOG_AUTO** `[4]` (RW)
    ///
    /// EP5_TOG_AUTO
    #[inline(always)]
    pub fn ep5_tog_auto(&self) -> bool {
        (self.value.read_bits() & Self::EP5_TOG_AUTO_MASK) != 0
    }

    /// Set EP5_TOG_AUTO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep5_tog_auto(&self) {
        self.value.modify_bits(|r| r | Self::EP5_TOG_AUTO_MASK);
    }

    /// Clear EP5_TOG_AUTO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep5_tog_auto(&self) {
        self.value.modify_bits(|r| r & !Self::EP5_TOG_AUTO_MASK);
    }

    /// **EP5_R_RES** `[3:2]` (RW)
    ///
    /// EP5_R_RES
    #[inline(always)]
    pub fn ep5_r_res(&self) -> u32 {
        (self.value.read_bits() & Self::EP5_R_RES_MASK) >> Self::EP5_R_RES_OFFSET
    }

    /// Write **EP5_R_RES** `[3:2]` (RW)
    ///
    /// EP5_R_RES
    #[inline(always)]
    pub fn write_ep5_r_res(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP5_R_RES_MASK) | ((value << Self::EP5_R_RES_OFFSET) & Self::EP5_R_RES_MASK)
        });
    }

    /// **EP5_T_RES** `[1:0]` (RW)
    ///
    /// EP5_T_RES
    #[inline(always)]
    pub fn ep5_t_res(&self) -> u32 {
        (self.value.read_bits() & Self::EP5_T_RES_MASK) >> Self::EP5_T_RES_OFFSET
    }

    /// Write **EP5_T_RES** `[1:0]` (RW)
    ///
    /// EP5_T_RES
    #[inline(always)]
    pub fn write_ep5_t_res(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP5_T_RES_MASK) | ((value << Self::EP5_T_RES_OFFSET) & Self::EP5_T_RES_MASK)
        });
    }
}

/// UEP5_CTRL_H value type for batch read/modify/write operations
///
/// Use `uep5ctrlh.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uep5ctrlh.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uep5CtrlHVal {
    bits: u32,
}

impl Uep5CtrlHVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **EP5_R_TOG** `[7:7]`
    ///
    /// EP5_R_TOG
    #[inline(always)]
    pub const fn ep5_r_tog(&self) -> bool {
        (self.bits & Uep5CtrlH::EP5_R_TOG_MASK) != 0
    }

    /// Read **EP5_T_TOG** `[6:6]`
    ///
    /// EP5_T_TOG
    #[inline(always)]
    pub const fn ep5_t_tog(&self) -> bool {
        (self.bits & Uep5CtrlH::EP5_T_TOG_MASK) != 0
    }

    /// Read **EP5_TOG_AUTO** `[4:4]`
    ///
    /// EP5_TOG_AUTO
    #[inline(always)]
    pub const fn ep5_tog_auto(&self) -> bool {
        (self.bits & Uep5CtrlH::EP5_TOG_AUTO_MASK) != 0
    }

    /// Read **EP5_R_RES** `[3:2]`
    ///
    /// EP5_R_RES
    #[inline(always)]
    pub const fn ep5_r_res(&self) -> u32 {
        (self.bits & Uep5CtrlH::EP5_R_RES_MASK) >> Uep5CtrlH::EP5_R_RES_OFFSET
    }

    /// Read **EP5_T_RES** `[1:0]`
    ///
    /// EP5_T_RES
    #[inline(always)]
    pub const fn ep5_t_res(&self) -> u32 {
        (self.bits & Uep5CtrlH::EP5_T_RES_MASK) >> Uep5CtrlH::EP5_T_RES_OFFSET
    }

    /// Set **EP5_R_TOG** `[7:7]`
    ///
    /// EP5_R_TOG
    #[inline(always)]
    pub const fn with_ep5_r_tog(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep5CtrlH::EP5_R_TOG_MASK
            } else {
                self.bits & !Uep5CtrlH::EP5_R_TOG_MASK
            },
        }
    }

    /// Set **EP5_T_TOG** `[6:6]`
    ///
    /// EP5_T_TOG
    #[inline(always)]
    pub const fn with_ep5_t_tog(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep5CtrlH::EP5_T_TOG_MASK
            } else {
                self.bits & !Uep5CtrlH::EP5_T_TOG_MASK
            },
        }
    }

    /// Set **EP5_TOG_AUTO** `[4:4]`
    ///
    /// EP5_TOG_AUTO
    #[inline(always)]
    pub const fn with_ep5_tog_auto(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep5CtrlH::EP5_TOG_AUTO_MASK
            } else {
                self.bits & !Uep5CtrlH::EP5_TOG_AUTO_MASK
            },
        }
    }

    /// Set **EP5_R_RES** `[3:2]`
    ///
    /// EP5_R_RES
    #[inline(always)]
    pub const fn with_ep5_r_res(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uep5CtrlH::EP5_R_RES_MASK)
                | ((value << Uep5CtrlH::EP5_R_RES_OFFSET) & Uep5CtrlH::EP5_R_RES_MASK),
        }
    }

    /// Set **EP5_T_RES** `[1:0]`
    ///
    /// EP5_T_RES
    #[inline(always)]
    pub const fn with_ep5_t_res(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uep5CtrlH::EP5_T_RES_MASK)
                | ((value << Uep5CtrlH::EP5_T_RES_OFFSET) & Uep5CtrlH::EP5_T_RES_MASK),
        }
    }
}

impl Default for Uep5CtrlHVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uep5CtrlH {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uep5CtrlHVal {
        Uep5CtrlHVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Uep5CtrlHVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Uep5CtrlHVal) -> Uep5CtrlHVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UEP6_TX_LEN Register
///
/// UEP6_TX_LEN
///
/// Address offset: 0x0068
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EP6_T_LEN** `[6:0]` (RW)
///
///   EP6_T_LEN
#[repr(transparent)]
pub struct Uep6TxLen {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uep6TxLen {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uep6TxLen {
    // ======== Field Constants ========

    /// **EP6_T_LEN**
    ///
    /// EP6_T_LEN
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const EP6_T_LEN_OFFSET: u32 = 0;
    pub const EP6_T_LEN_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **EP6_T_LEN** `[6:0]` (RW)
    ///
    /// EP6_T_LEN
    #[inline(always)]
    pub fn ep6_t_len(&self) -> u32 {
        (self.value.read_bits() & Self::EP6_T_LEN_MASK) >> Self::EP6_T_LEN_OFFSET
    }

    /// Write **EP6_T_LEN** `[6:0]` (RW)
    ///
    /// EP6_T_LEN
    #[inline(always)]
    pub fn write_ep6_t_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP6_T_LEN_MASK) | ((value << Self::EP6_T_LEN_OFFSET) & Self::EP6_T_LEN_MASK)
        });
    }
}

/// UEP6_TX_LEN value type for batch read/modify/write operations
///
/// Use `uep6txlen.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uep6txlen.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uep6TxLenVal {
    bits: u32,
}

impl Uep6TxLenVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **EP6_T_LEN** `[6:0]`
    ///
    /// EP6_T_LEN
    #[inline(always)]
    pub const fn ep6_t_len(&self) -> u32 {
        (self.bits & Uep6TxLen::EP6_T_LEN_MASK) >> Uep6TxLen::EP6_T_LEN_OFFSET
    }

    /// Set **EP6_T_LEN** `[6:0]`
    ///
    /// EP6_T_LEN
    #[inline(always)]
    pub const fn with_ep6_t_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uep6TxLen::EP6_T_LEN_MASK)
                | ((value << Uep6TxLen::EP6_T_LEN_OFFSET) & Uep6TxLen::EP6_T_LEN_MASK),
        }
    }
}

impl Default for Uep6TxLenVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uep6TxLen {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uep6TxLenVal {
        Uep6TxLenVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Uep6TxLenVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Uep6TxLenVal) -> Uep6TxLenVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UEP6_CTRL_H Register
///
/// UEP6_CTRL_H
///
/// Address offset: 0x006a
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EP6_R_TOG** `[7]` (RW)
///
///   EP6_R_TOG
///
/// - **EP6_T_TOG** `[6]` (RW)
///
///   EP6_T_TOG
///
/// - **EP6_TOG_AUTO** `[4]` (RW)
///
///   EP6_TOG_AUTO
///
/// - **EP6_R_RES** `[3:2]` (RW)
///
///   EP6_R_RES
///
/// - **EP6_T_RES** `[1:0]` (RW)
///
///   EP6_T_RES
#[repr(transparent)]
pub struct Uep6CtrlH {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uep6CtrlH {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uep6CtrlH {
    // ======== Field Constants ========

    /// **EP6_R_TOG**
    ///
    /// EP6_R_TOG
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const EP6_R_TOG_OFFSET: u32 = 7;
    pub const EP6_R_TOG_MASK: u32 = 0x00000080;
    pub const EP6_R_TOG: u32 = 1 << 7;

    /// **EP6_T_TOG**
    ///
    /// EP6_T_TOG
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const EP6_T_TOG_OFFSET: u32 = 6;
    pub const EP6_T_TOG_MASK: u32 = 0x00000040;
    pub const EP6_T_TOG: u32 = 1 << 6;

    /// **EP6_TOG_AUTO**
    ///
    /// EP6_TOG_AUTO
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const EP6_TOG_AUTO_OFFSET: u32 = 4;
    pub const EP6_TOG_AUTO_MASK: u32 = 0x00000010;
    pub const EP6_TOG_AUTO: u32 = 1 << 4;

    /// **EP6_R_RES**
    ///
    /// EP6_R_RES
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const EP6_R_RES_OFFSET: u32 = 2;
    pub const EP6_R_RES_MASK: u32 = 0x0000000c;

    /// **EP6_T_RES**
    ///
    /// EP6_T_RES
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const EP6_T_RES_OFFSET: u32 = 0;
    pub const EP6_T_RES_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **EP6_R_TOG** `[7]` (RW)
    ///
    /// EP6_R_TOG
    #[inline(always)]
    pub fn ep6_r_tog(&self) -> bool {
        (self.value.read_bits() & Self::EP6_R_TOG_MASK) != 0
    }

    /// Set EP6_R_TOG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep6_r_tog(&self) {
        self.value.modify_bits(|r| r | Self::EP6_R_TOG_MASK);
    }

    /// Clear EP6_R_TOG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep6_r_tog(&self) {
        self.value.modify_bits(|r| r & !Self::EP6_R_TOG_MASK);
    }

    /// **EP6_T_TOG** `[6]` (RW)
    ///
    /// EP6_T_TOG
    #[inline(always)]
    pub fn ep6_t_tog(&self) -> bool {
        (self.value.read_bits() & Self::EP6_T_TOG_MASK) != 0
    }

    /// Set EP6_T_TOG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep6_t_tog(&self) {
        self.value.modify_bits(|r| r | Self::EP6_T_TOG_MASK);
    }

    /// Clear EP6_T_TOG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep6_t_tog(&self) {
        self.value.modify_bits(|r| r & !Self::EP6_T_TOG_MASK);
    }

    /// **EP6_TOG_AUTO** `[4]` (RW)
    ///
    /// EP6_TOG_AUTO
    #[inline(always)]
    pub fn ep6_tog_auto(&self) -> bool {
        (self.value.read_bits() & Self::EP6_TOG_AUTO_MASK) != 0
    }

    /// Set EP6_TOG_AUTO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep6_tog_auto(&self) {
        self.value.modify_bits(|r| r | Self::EP6_TOG_AUTO_MASK);
    }

    /// Clear EP6_TOG_AUTO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep6_tog_auto(&self) {
        self.value.modify_bits(|r| r & !Self::EP6_TOG_AUTO_MASK);
    }

    /// **EP6_R_RES** `[3:2]` (RW)
    ///
    /// EP6_R_RES
    #[inline(always)]
    pub fn ep6_r_res(&self) -> u32 {
        (self.value.read_bits() & Self::EP6_R_RES_MASK) >> Self::EP6_R_RES_OFFSET
    }

    /// Write **EP6_R_RES** `[3:2]` (RW)
    ///
    /// EP6_R_RES
    #[inline(always)]
    pub fn write_ep6_r_res(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP6_R_RES_MASK) | ((value << Self::EP6_R_RES_OFFSET) & Self::EP6_R_RES_MASK)
        });
    }

    /// **EP6_T_RES** `[1:0]` (RW)
    ///
    /// EP6_T_RES
    #[inline(always)]
    pub fn ep6_t_res(&self) -> u32 {
        (self.value.read_bits() & Self::EP6_T_RES_MASK) >> Self::EP6_T_RES_OFFSET
    }

    /// Write **EP6_T_RES** `[1:0]` (RW)
    ///
    /// EP6_T_RES
    #[inline(always)]
    pub fn write_ep6_t_res(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP6_T_RES_MASK) | ((value << Self::EP6_T_RES_OFFSET) & Self::EP6_T_RES_MASK)
        });
    }
}

/// UEP6_CTRL_H value type for batch read/modify/write operations
///
/// Use `uep6ctrlh.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uep6ctrlh.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uep6CtrlHVal {
    bits: u32,
}

impl Uep6CtrlHVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **EP6_R_TOG** `[7:7]`
    ///
    /// EP6_R_TOG
    #[inline(always)]
    pub const fn ep6_r_tog(&self) -> bool {
        (self.bits & Uep6CtrlH::EP6_R_TOG_MASK) != 0
    }

    /// Read **EP6_T_TOG** `[6:6]`
    ///
    /// EP6_T_TOG
    #[inline(always)]
    pub const fn ep6_t_tog(&self) -> bool {
        (self.bits & Uep6CtrlH::EP6_T_TOG_MASK) != 0
    }

    /// Read **EP6_TOG_AUTO** `[4:4]`
    ///
    /// EP6_TOG_AUTO
    #[inline(always)]
    pub const fn ep6_tog_auto(&self) -> bool {
        (self.bits & Uep6CtrlH::EP6_TOG_AUTO_MASK) != 0
    }

    /// Read **EP6_R_RES** `[3:2]`
    ///
    /// EP6_R_RES
    #[inline(always)]
    pub const fn ep6_r_res(&self) -> u32 {
        (self.bits & Uep6CtrlH::EP6_R_RES_MASK) >> Uep6CtrlH::EP6_R_RES_OFFSET
    }

    /// Read **EP6_T_RES** `[1:0]`
    ///
    /// EP6_T_RES
    #[inline(always)]
    pub const fn ep6_t_res(&self) -> u32 {
        (self.bits & Uep6CtrlH::EP6_T_RES_MASK) >> Uep6CtrlH::EP6_T_RES_OFFSET
    }

    /// Set **EP6_R_TOG** `[7:7]`
    ///
    /// EP6_R_TOG
    #[inline(always)]
    pub const fn with_ep6_r_tog(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep6CtrlH::EP6_R_TOG_MASK
            } else {
                self.bits & !Uep6CtrlH::EP6_R_TOG_MASK
            },
        }
    }

    /// Set **EP6_T_TOG** `[6:6]`
    ///
    /// EP6_T_TOG
    #[inline(always)]
    pub const fn with_ep6_t_tog(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep6CtrlH::EP6_T_TOG_MASK
            } else {
                self.bits & !Uep6CtrlH::EP6_T_TOG_MASK
            },
        }
    }

    /// Set **EP6_TOG_AUTO** `[4:4]`
    ///
    /// EP6_TOG_AUTO
    #[inline(always)]
    pub const fn with_ep6_tog_auto(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep6CtrlH::EP6_TOG_AUTO_MASK
            } else {
                self.bits & !Uep6CtrlH::EP6_TOG_AUTO_MASK
            },
        }
    }

    /// Set **EP6_R_RES** `[3:2]`
    ///
    /// EP6_R_RES
    #[inline(always)]
    pub const fn with_ep6_r_res(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uep6CtrlH::EP6_R_RES_MASK)
                | ((value << Uep6CtrlH::EP6_R_RES_OFFSET) & Uep6CtrlH::EP6_R_RES_MASK),
        }
    }

    /// Set **EP6_T_RES** `[1:0]`
    ///
    /// EP6_T_RES
    #[inline(always)]
    pub const fn with_ep6_t_res(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uep6CtrlH::EP6_T_RES_MASK)
                | ((value << Uep6CtrlH::EP6_T_RES_OFFSET) & Uep6CtrlH::EP6_T_RES_MASK),
        }
    }
}

impl Default for Uep6CtrlHVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uep6CtrlH {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uep6CtrlHVal {
        Uep6CtrlHVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Uep6CtrlHVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Uep6CtrlHVal) -> Uep6CtrlHVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UEP7_TX_LEN Register
///
/// UEP7_TX_LEN
///
/// Address offset: 0x006c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EP7_T_LEN** `[6:0]` (RW)
///
///   EP7_T_LEN
#[repr(transparent)]
pub struct Uep7TxLen {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uep7TxLen {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uep7TxLen {
    // ======== Field Constants ========

    /// **EP7_T_LEN**
    ///
    /// EP7_T_LEN
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const EP7_T_LEN_OFFSET: u32 = 0;
    pub const EP7_T_LEN_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **EP7_T_LEN** `[6:0]` (RW)
    ///
    /// EP7_T_LEN
    #[inline(always)]
    pub fn ep7_t_len(&self) -> u32 {
        (self.value.read_bits() & Self::EP7_T_LEN_MASK) >> Self::EP7_T_LEN_OFFSET
    }

    /// Write **EP7_T_LEN** `[6:0]` (RW)
    ///
    /// EP7_T_LEN
    #[inline(always)]
    pub fn write_ep7_t_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP7_T_LEN_MASK) | ((value << Self::EP7_T_LEN_OFFSET) & Self::EP7_T_LEN_MASK)
        });
    }
}

/// UEP7_TX_LEN value type for batch read/modify/write operations
///
/// Use `uep7txlen.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uep7txlen.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uep7TxLenVal {
    bits: u32,
}

impl Uep7TxLenVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **EP7_T_LEN** `[6:0]`
    ///
    /// EP7_T_LEN
    #[inline(always)]
    pub const fn ep7_t_len(&self) -> u32 {
        (self.bits & Uep7TxLen::EP7_T_LEN_MASK) >> Uep7TxLen::EP7_T_LEN_OFFSET
    }

    /// Set **EP7_T_LEN** `[6:0]`
    ///
    /// EP7_T_LEN
    #[inline(always)]
    pub const fn with_ep7_t_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uep7TxLen::EP7_T_LEN_MASK)
                | ((value << Uep7TxLen::EP7_T_LEN_OFFSET) & Uep7TxLen::EP7_T_LEN_MASK),
        }
    }
}

impl Default for Uep7TxLenVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uep7TxLen {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uep7TxLenVal {
        Uep7TxLenVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Uep7TxLenVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Uep7TxLenVal) -> Uep7TxLenVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UEP7_CTRL_H Register
///
/// UEP7_CTRL_H
///
/// Address offset: 0x006e
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EP7_R_TOG** `[7]` (RW)
///
///   EP7_R_TOG
///
/// - **EP7_T_TOG** `[6]` (RW)
///
///   EP7_T_TOG
///
/// - **EP7_TOG_AUTO** `[4]` (RW)
///
///   EP7_TOG_AUTO
///
/// - **EP7_R_RES** `[3:2]` (RW)
///
///   EP7_R_RES
///
/// - **EP7_T_RES** `[1:0]` (RW)
///
///   EP7_T_RES
#[repr(transparent)]
pub struct Uep7CtrlH {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uep7CtrlH {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uep7CtrlH {
    // ======== Field Constants ========

    /// **EP7_R_TOG**
    ///
    /// EP7_R_TOG
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const EP7_R_TOG_OFFSET: u32 = 7;
    pub const EP7_R_TOG_MASK: u32 = 0x00000080;
    pub const EP7_R_TOG: u32 = 1 << 7;

    /// **EP7_T_TOG**
    ///
    /// EP7_T_TOG
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const EP7_T_TOG_OFFSET: u32 = 6;
    pub const EP7_T_TOG_MASK: u32 = 0x00000040;
    pub const EP7_T_TOG: u32 = 1 << 6;

    /// **EP7_TOG_AUTO**
    ///
    /// EP7_TOG_AUTO
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const EP7_TOG_AUTO_OFFSET: u32 = 4;
    pub const EP7_TOG_AUTO_MASK: u32 = 0x00000010;
    pub const EP7_TOG_AUTO: u32 = 1 << 4;

    /// **EP7_R_RES**
    ///
    /// EP7_R_RES
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const EP7_R_RES_OFFSET: u32 = 2;
    pub const EP7_R_RES_MASK: u32 = 0x0000000c;

    /// **EP7_T_RES**
    ///
    /// EP7_T_RES
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const EP7_T_RES_OFFSET: u32 = 0;
    pub const EP7_T_RES_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **EP7_R_TOG** `[7]` (RW)
    ///
    /// EP7_R_TOG
    #[inline(always)]
    pub fn ep7_r_tog(&self) -> bool {
        (self.value.read_bits() & Self::EP7_R_TOG_MASK) != 0
    }

    /// Set EP7_R_TOG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep7_r_tog(&self) {
        self.value.modify_bits(|r| r | Self::EP7_R_TOG_MASK);
    }

    /// Clear EP7_R_TOG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep7_r_tog(&self) {
        self.value.modify_bits(|r| r & !Self::EP7_R_TOG_MASK);
    }

    /// **EP7_T_TOG** `[6]` (RW)
    ///
    /// EP7_T_TOG
    #[inline(always)]
    pub fn ep7_t_tog(&self) -> bool {
        (self.value.read_bits() & Self::EP7_T_TOG_MASK) != 0
    }

    /// Set EP7_T_TOG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep7_t_tog(&self) {
        self.value.modify_bits(|r| r | Self::EP7_T_TOG_MASK);
    }

    /// Clear EP7_T_TOG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep7_t_tog(&self) {
        self.value.modify_bits(|r| r & !Self::EP7_T_TOG_MASK);
    }

    /// **EP7_TOG_AUTO** `[4]` (RW)
    ///
    /// EP7_TOG_AUTO
    #[inline(always)]
    pub fn ep7_tog_auto(&self) -> bool {
        (self.value.read_bits() & Self::EP7_TOG_AUTO_MASK) != 0
    }

    /// Set EP7_TOG_AUTO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ep7_tog_auto(&self) {
        self.value.modify_bits(|r| r | Self::EP7_TOG_AUTO_MASK);
    }

    /// Clear EP7_TOG_AUTO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ep7_tog_auto(&self) {
        self.value.modify_bits(|r| r & !Self::EP7_TOG_AUTO_MASK);
    }

    /// **EP7_R_RES** `[3:2]` (RW)
    ///
    /// EP7_R_RES
    #[inline(always)]
    pub fn ep7_r_res(&self) -> u32 {
        (self.value.read_bits() & Self::EP7_R_RES_MASK) >> Self::EP7_R_RES_OFFSET
    }

    /// Write **EP7_R_RES** `[3:2]` (RW)
    ///
    /// EP7_R_RES
    #[inline(always)]
    pub fn write_ep7_r_res(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP7_R_RES_MASK) | ((value << Self::EP7_R_RES_OFFSET) & Self::EP7_R_RES_MASK)
        });
    }

    /// **EP7_T_RES** `[1:0]` (RW)
    ///
    /// EP7_T_RES
    #[inline(always)]
    pub fn ep7_t_res(&self) -> u32 {
        (self.value.read_bits() & Self::EP7_T_RES_MASK) >> Self::EP7_T_RES_OFFSET
    }

    /// Write **EP7_T_RES** `[1:0]` (RW)
    ///
    /// EP7_T_RES
    #[inline(always)]
    pub fn write_ep7_t_res(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP7_T_RES_MASK) | ((value << Self::EP7_T_RES_OFFSET) & Self::EP7_T_RES_MASK)
        });
    }
}

/// UEP7_CTRL_H value type for batch read/modify/write operations
///
/// Use `uep7ctrlh.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uep7ctrlh.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uep7CtrlHVal {
    bits: u32,
}

impl Uep7CtrlHVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **EP7_R_TOG** `[7:7]`
    ///
    /// EP7_R_TOG
    #[inline(always)]
    pub const fn ep7_r_tog(&self) -> bool {
        (self.bits & Uep7CtrlH::EP7_R_TOG_MASK) != 0
    }

    /// Read **EP7_T_TOG** `[6:6]`
    ///
    /// EP7_T_TOG
    #[inline(always)]
    pub const fn ep7_t_tog(&self) -> bool {
        (self.bits & Uep7CtrlH::EP7_T_TOG_MASK) != 0
    }

    /// Read **EP7_TOG_AUTO** `[4:4]`
    ///
    /// EP7_TOG_AUTO
    #[inline(always)]
    pub const fn ep7_tog_auto(&self) -> bool {
        (self.bits & Uep7CtrlH::EP7_TOG_AUTO_MASK) != 0
    }

    /// Read **EP7_R_RES** `[3:2]`
    ///
    /// EP7_R_RES
    #[inline(always)]
    pub const fn ep7_r_res(&self) -> u32 {
        (self.bits & Uep7CtrlH::EP7_R_RES_MASK) >> Uep7CtrlH::EP7_R_RES_OFFSET
    }

    /// Read **EP7_T_RES** `[1:0]`
    ///
    /// EP7_T_RES
    #[inline(always)]
    pub const fn ep7_t_res(&self) -> u32 {
        (self.bits & Uep7CtrlH::EP7_T_RES_MASK) >> Uep7CtrlH::EP7_T_RES_OFFSET
    }

    /// Set **EP7_R_TOG** `[7:7]`
    ///
    /// EP7_R_TOG
    #[inline(always)]
    pub const fn with_ep7_r_tog(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep7CtrlH::EP7_R_TOG_MASK
            } else {
                self.bits & !Uep7CtrlH::EP7_R_TOG_MASK
            },
        }
    }

    /// Set **EP7_T_TOG** `[6:6]`
    ///
    /// EP7_T_TOG
    #[inline(always)]
    pub const fn with_ep7_t_tog(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep7CtrlH::EP7_T_TOG_MASK
            } else {
                self.bits & !Uep7CtrlH::EP7_T_TOG_MASK
            },
        }
    }

    /// Set **EP7_TOG_AUTO** `[4:4]`
    ///
    /// EP7_TOG_AUTO
    #[inline(always)]
    pub const fn with_ep7_tog_auto(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uep7CtrlH::EP7_TOG_AUTO_MASK
            } else {
                self.bits & !Uep7CtrlH::EP7_TOG_AUTO_MASK
            },
        }
    }

    /// Set **EP7_R_RES** `[3:2]`
    ///
    /// EP7_R_RES
    #[inline(always)]
    pub const fn with_ep7_r_res(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uep7CtrlH::EP7_R_RES_MASK)
                | ((value << Uep7CtrlH::EP7_R_RES_OFFSET) & Uep7CtrlH::EP7_R_RES_MASK),
        }
    }

    /// Set **EP7_T_RES** `[1:0]`
    ///
    /// EP7_T_RES
    #[inline(always)]
    pub const fn with_ep7_t_res(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uep7CtrlH::EP7_T_RES_MASK)
                | ((value << Uep7CtrlH::EP7_T_RES_OFFSET) & Uep7CtrlH::EP7_T_RES_MASK),
        }
    }
}

impl Default for Uep7CtrlHVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uep7CtrlH {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uep7CtrlHVal {
        Uep7CtrlHVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Uep7CtrlHVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Uep7CtrlHVal) -> Uep7CtrlHVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// USB_EPX_CTRL Register
///
/// USB_EPX_CTRL
///
/// Address offset: 0x0070
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EP_T_AF** `[23:17]` (RW)
///
///   EP_T_AF
///
/// - **EP_R_LEN** `[15:8]` (RW)
///
///   EP5_R_LEN
///
/// - **EP_T_LEN** `[7:0]` (RW)
///
///   EP5_T_LEN
#[repr(transparent)]
pub struct UsbEpxCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for UsbEpxCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl UsbEpxCtrl {
    // ======== Field Constants ========

    /// **EP_T_AF**
    ///
    /// EP_T_AF
    ///
    /// Bits `[23:17]`, mask `0x00fe0000`
    pub const EP_T_AF_OFFSET: u32 = 17;
    pub const EP_T_AF_MASK: u32 = 0x00fe0000;

    /// **EP_R_LEN**
    ///
    /// EP5_R_LEN
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const EP_R_LEN_OFFSET: u32 = 8;
    pub const EP_R_LEN_MASK: u32 = 0x0000ff00;

    /// **EP_T_LEN**
    ///
    /// EP5_T_LEN
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const EP_T_LEN_OFFSET: u32 = 0;
    pub const EP_T_LEN_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **EP_T_AF** `[23:17]` (RW)
    ///
    /// EP_T_AF
    #[inline(always)]
    pub fn ep_t_af(&self) -> u32 {
        (self.value.read_bits() & Self::EP_T_AF_MASK) >> Self::EP_T_AF_OFFSET
    }

    /// Write **EP_T_AF** `[23:17]` (RW)
    ///
    /// EP_T_AF
    #[inline(always)]
    pub fn write_ep_t_af(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP_T_AF_MASK) | ((value << Self::EP_T_AF_OFFSET) & Self::EP_T_AF_MASK)
        });
    }

    /// **EP_R_LEN** `[15:8]` (RW)
    ///
    /// EP5_R_LEN
    #[inline(always)]
    pub fn ep_r_len(&self) -> u32 {
        (self.value.read_bits() & Self::EP_R_LEN_MASK) >> Self::EP_R_LEN_OFFSET
    }

    /// Write **EP_R_LEN** `[15:8]` (RW)
    ///
    /// EP5_R_LEN
    #[inline(always)]
    pub fn write_ep_r_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP_R_LEN_MASK) | ((value << Self::EP_R_LEN_OFFSET) & Self::EP_R_LEN_MASK)
        });
    }

    /// **EP_T_LEN** `[7:0]` (RW)
    ///
    /// EP5_T_LEN
    #[inline(always)]
    pub fn ep_t_len(&self) -> u32 {
        (self.value.read_bits() & Self::EP_T_LEN_MASK) >> Self::EP_T_LEN_OFFSET
    }

    /// Write **EP_T_LEN** `[7:0]` (RW)
    ///
    /// EP5_T_LEN
    #[inline(always)]
    pub fn write_ep_t_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP_T_LEN_MASK) | ((value << Self::EP_T_LEN_OFFSET) & Self::EP_T_LEN_MASK)
        });
    }
}

/// USB_EPX_CTRL value type for batch read/modify/write operations
///
/// Use `usbepxctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `usbepxctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UsbEpxCtrlVal {
    bits: u32,
}

impl UsbEpxCtrlVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **EP_T_AF** `[23:17]`
    ///
    /// EP_T_AF
    #[inline(always)]
    pub const fn ep_t_af(&self) -> u32 {
        (self.bits & UsbEpxCtrl::EP_T_AF_MASK) >> UsbEpxCtrl::EP_T_AF_OFFSET
    }

    /// Read **EP_R_LEN** `[15:8]`
    ///
    /// EP5_R_LEN
    #[inline(always)]
    pub const fn ep_r_len(&self) -> u32 {
        (self.bits & UsbEpxCtrl::EP_R_LEN_MASK) >> UsbEpxCtrl::EP_R_LEN_OFFSET
    }

    /// Read **EP_T_LEN** `[7:0]`
    ///
    /// EP5_T_LEN
    #[inline(always)]
    pub const fn ep_t_len(&self) -> u32 {
        (self.bits & UsbEpxCtrl::EP_T_LEN_MASK) >> UsbEpxCtrl::EP_T_LEN_OFFSET
    }

    /// Set **EP_T_AF** `[23:17]`
    ///
    /// EP_T_AF
    #[inline(always)]
    pub const fn with_ep_t_af(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !UsbEpxCtrl::EP_T_AF_MASK)
                | ((value << UsbEpxCtrl::EP_T_AF_OFFSET) & UsbEpxCtrl::EP_T_AF_MASK),
        }
    }

    /// Set **EP_R_LEN** `[15:8]`
    ///
    /// EP5_R_LEN
    #[inline(always)]
    pub const fn with_ep_r_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !UsbEpxCtrl::EP_R_LEN_MASK)
                | ((value << UsbEpxCtrl::EP_R_LEN_OFFSET) & UsbEpxCtrl::EP_R_LEN_MASK),
        }
    }

    /// Set **EP_T_LEN** `[7:0]`
    ///
    /// EP5_T_LEN
    #[inline(always)]
    pub const fn with_ep_t_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !UsbEpxCtrl::EP_T_LEN_MASK)
                | ((value << UsbEpxCtrl::EP_T_LEN_OFFSET) & UsbEpxCtrl::EP_T_LEN_MASK),
        }
    }
}

impl Default for UsbEpxCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl UsbEpxCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> UsbEpxCtrlVal {
        UsbEpxCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: UsbEpxCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(UsbEpxCtrlVal) -> UsbEpxCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// USB base control
    pub base_ctrl: BaseCtrl,
    /// 0x0001
    /// USB HOST control_UDEV_CTRL
    pub host_ctrl_udev_ctrl: HostCtrlUdevCtrl,
    /// 0x0002
    /// USB interrupt enable
    pub int_en: IntEn,
    /// 0x0003
    /// USB device address
    pub dev_addr: DevAddr,
    /// 0x0005
    /// USB_MIS_ST
    pub mis_st: MisSt,
    /// 0x0006
    /// USB_INT_FG
    pub int_fg: IntFg,
    /// 0x0007
    /// USB_INT_ST
    pub int_st: IntSt,
    /// 0x0008
    /// USB_RX_LEN
    pub rx_len: RxLen,
    /// 0x000c
    /// UEP4_1_MOD
    pub uep4_1_mod: Uep41Mod,
    /// 0x000d
    /// UEP2_3_MOD_HOST_EP_MOD
    pub uep2_3_mod_host_ep_mod: Uep23ModHostEpMod,
    /// 0x000e
    /// UEP567_MOD
    pub uep567_mod: Uep567Mod,
    /// 0x0010
    /// UEP0_DMA
    pub uep0_dma: Uep0Dma,
    /// 0x0014
    /// UEP1_DMA
    pub uep1_dma: Uep1Dma,
    /// 0x0018
    /// UEP2_DMA_HOST_RX_DMA
    pub uep2_dma_host_rx_dma: Uep2DmaHostRxDma,
    /// 0x001c
    /// UEP3_DMA_HOST_TX_DMA
    pub uep3_dma_host_tx_dma: Uep3DmaHostTxDma,
    /// 0x0020
    /// UEP0_TX_LEN
    pub uep0_tx_len: Uep0TxLen,
    /// 0x0022
    /// UEP0_CTRL_H
    pub uep0_ctrl_h: Uep0CtrlH,
    /// 0x0024
    /// UEP1_TX_LEN
    pub uep1_tx_len: Uep1TxLen,
    /// 0x0026
    /// UEP1_CTRL_H_HOST_SETUP
    pub uep1_ctrl_h_host_setup: Uep1CtrlHHostSetup,
    /// 0x0028
    /// UEP2_TX_LEN
    pub uep2_tx_len: Uep2TxLen,
    /// 0x002a
    /// UEP2_CTRL_H
    pub uep2_ctrl_h: Uep2CtrlH,
    /// 0x002c
    /// UEP3_TX_LEN
    pub uep3_tx_len: Uep3TxLen,
    /// 0x002e
    /// UEP3_CTRL_H
    pub uep3_ctrl_h: Uep3CtrlH,
    /// 0x0030
    /// UEP4_TX_LEN
    pub uep4_tx_len: Uep4TxLen,
    /// 0x0032
    /// UEP4_CTRL_H
    pub uep4_ctrl_h: Uep4CtrlH,
    _reserved0: [u8; 30],
    /// 0x0054
    /// UEP5_DMA
    pub uep5_dma: Uep5Dma,
    /// 0x0058
    /// UEP6_DMA
    pub uep6_dma: Uep6Dma,
    /// 0x005c
    /// UEP7_DMA
    pub uep7_dma: Uep7Dma,
    _reserved1: [u8; 4],
    /// 0x0064
    /// UEP5_TX_LEN
    pub uep5_tx_len: Uep5TxLen,
    /// 0x0066
    /// UEP5_CTRL_H
    pub uep5_ctrl_h: Uep5CtrlH,
    /// 0x0068
    /// UEP6_TX_LEN
    pub uep6_tx_len: Uep6TxLen,
    /// 0x006a
    /// UEP6_CTRL_H
    pub uep6_ctrl_h: Uep6CtrlH,
    /// 0x006c
    /// UEP7_TX_LEN
    pub uep7_tx_len: Uep7TxLen,
    /// 0x006e
    /// UEP7_CTRL_H
    pub uep7_ctrl_h: Uep7CtrlH,
    /// 0x0070
    /// USB_EPX_CTRL
    pub usb_epx_ctrl: UsbEpxCtrl,
}
