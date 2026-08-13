//! USB PD protocol layer: pure data types and encode/decode helpers.
//!
//! Everything here is hardware-independent: message headers, data objects
//! ([`Pdo`] / [`Rdo`]) and the byte-level encode/decode of PD frames. The
//! transceiver (BMC, SOP, CRC) lives in [`super::phy`], and the blocking
//! transmit/receive logic (GoodCRC, Message-ID) in [`super::port`].
//!
//! Note: the CH32X035 header layout matches the USB PD spec (`NDO` at
//! bits `[14:12]`, `MsgID` at `[11:9]`, `DRole` at bit 5, `SpecRev` at
//! `[7:6]`). Data objects (PDO/RDO) follow the standard layout as well.

/// Maximum number of data objects carried by a single (non-extended) message.
pub const MAX_DATA_OBJECTS: usize = 7;

/// Maximum data bytes carried in a single extended-message chunk (a chunk
/// frame is the 2-byte message header, the 2-byte extended header and 26
/// data bytes packed into 7 data objects).
pub const MAX_CHUNK_DATA: usize = 26;
/// Maximum total data bytes of a (chunked) extended message.
pub const MAX_EXTENDED_DATA: usize = 260;
/// Maximum number of chunks needed for the longest extended message.
pub const MAX_CHUNKS: usize = MAX_EXTENDED_DATA / MAX_CHUNK_DATA;

/// USB PD 16-bit message header.
///
/// ```text
/// bit[4:0]  Message Type
/// bit5      Port Data Role (0 = UFP, 1 = DFP)
/// bit[7:6]  Spec Revision (00 = 1.0, 01 = 2.0, 10 = 3.x)
/// bit8      Port Power Role (0 = Sink, 1 = Source)
/// bit[11:9] Message ID
/// bit[14:12] Number of Data Objects
/// bit15     Extended
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Header(u16);

impl Header {
    /// An empty header (all fields zero).
    pub const fn new() -> Self {
        Self(0)
    }

    /// Build a header from raw 16-bit value.
    pub const fn from_bits(bits: u16) -> Self {
        Self(bits)
    }

    /// The raw 16-bit header value.
    pub const fn bits(self) -> u16 {
        self.0
    }

    /// Message type (`bit[4:0]`).
    pub const fn message_type(self) -> u8 {
        (self.0 & 0x1F) as u8
    }

    /// Port data role (`bit5`).
    pub const fn port_data_role(self) -> DataRole {
        if self.0 & 0x20 != 0 {
            DataRole::Dfp
        } else {
            DataRole::Ufp
        }
    }

    /// Specification revision (`bit[7:6]`).
    pub const fn spec_revision(self) -> SpecRev {
        match (self.0 >> 6) & 0x3 {
            0 => SpecRev::Rev1_0,
            1 => SpecRev::Rev2_0,
            _ => SpecRev::Rev3_0,
        }
    }

    /// Port power role (`bit8`).
    pub const fn port_power_role(self) -> PowerRole {
        if self.0 & 0x100 != 0 {
            PowerRole::Source
        } else {
            PowerRole::Sink
        }
    }

    /// Message ID (`bit[11:9]`, 0..=7).
    pub const fn message_id(self) -> u8 {
        ((self.0 >> 9) & 0x7) as u8
    }

    /// Number of data objects (`bit[14:12]`, 0..=7).
    pub const fn num_data_objects(self) -> u8 {
        ((self.0 >> 12) & 0x7) as u8
    }

    /// Extended bit (`bit15`).
    pub const fn extended(self) -> bool {
        self.0 & 0x8000 != 0
    }

    /// Set the message type (`bit[4:0]`).
    pub const fn with_message_type(self, v: u8) -> Self {
        Self((self.0 & !0x1F) | (v as u16 & 0x1F))
    }

    /// Set the port data role (`bit5`).
    pub const fn with_port_data_role(self, v: DataRole) -> Self {
        Self((self.0 & !0x20) | ((v as u16) << 5))
    }

    /// Set the specification revision (`bit[7:6]`).
    pub const fn with_spec_revision(self, v: SpecRev) -> Self {
        Self((self.0 & !0xC0) | ((v as u16) << 6))
    }

    /// Set the port power role (`bit8`).
    pub const fn with_port_power_role(self, v: PowerRole) -> Self {
        Self((self.0 & !0x100) | ((v as u16) << 8))
    }

    /// Set the message ID (`bit[11:9]`, masked to 0..=7).
    pub const fn with_message_id(self, v: u8) -> Self {
        Self((self.0 & !0x0E00) | (((v as u16) & 0x7) << 9))
    }

    /// Set the number of data objects (`bit[14:12]`, masked to 0..=7).
    pub const fn with_num_data_objects(self, v: u8) -> Self {
        Self((self.0 & !0x7000) | (((v as u16) & 0x7) << 12))
    }

    /// Set the extended bit (`bit15`).
    pub const fn with_extended(self, v: bool) -> Self {
        if v {
            Self(self.0 | 0x8000)
        } else {
            Self(self.0 & !0x8000)
        }
    }
}

impl Default for Header {
    fn default() -> Self {
        Self::new()
    }
}

/// Port power role (`header` bit8): who is supplying power.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PowerRole {
    /// Sink (receives power)
    Sink = 0,
    /// Source (supplies power)
    Source = 1,
}

/// Port data role (`header` bit5): who is the data host.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DataRole {
    /// UFP (upstream-facing port, device)
    Ufp = 0,
    /// DFP (downstream-facing port, host)
    Dfp = 1,
}

/// Specification revision (`header` bit[7:6]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SpecRev {
    /// USB PD 1.0
    Rev1_0 = 0b00,
    /// USB PD 2.0
    Rev2_0 = 0b01,
    /// USB PD 3.0
    Rev3_0 = 0b10,
}

/// USB PD control message types (no data objects).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ControlMessage {
    /// Reserved
    Reserved = 0x00,
    /// GoodCRC
    GoodCrc = 0x01,
    /// GotoMin
    GotoMin = 0x02,
    /// Accept
    Accept = 0x03,
    /// Reject
    Reject = 0x04,
    /// Ping
    Ping = 0x05,
    /// PS_RDY
    PsRdy = 0x06,
    /// Get_Source_Cap
    GetSourceCap = 0x07,
    /// Get_Sink_Cap
    GetSinkCap = 0x08,
    /// DR_Swap
    DrSwap = 0x09,
    /// PR_Swap
    PrSwap = 0x0A,
    /// VCONN_Swap
    VconnSwap = 0x0B,
    /// Wait
    Wait = 0x0C,
    /// Soft_Reset
    SoftReset = 0x0D,
    /// Data_Reset
    DataReset = 0x0E,
    /// Data_Reset_Complete
    DataResetComplete = 0x0F,
    /// Not_Supported
    NotSupported = 0x10,
    /// Get_Source_Cap_Ext
    GetSourceCapExt = 0x11,
    /// Get_Status
    GetStatus = 0x12,
    /// FR_Swap
    FrSwap = 0x13,
    /// Get_PPS_Status
    GetPpsStatus = 0x14,
    /// Get_Country_Codes
    GetCountryCodes = 0x15,
    /// Get_Sink_Cap_Ext
    GetSinkCapExt = 0x16,
    /// Get_Source_Info
    GetSourceInfo = 0x17,
    /// Get_Revision
    GetRevision = 0x18,
}

/// USB PD data message types (with data objects).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DataMessage {
    /// Source_Capabilities
    SourceCapabilities = 0x01,
    /// Request
    Request = 0x02,
    /// BIST
    Bist = 0x03,
    /// Sink_Capabilities
    SinkCapabilities = 0x04,
    /// Battery_Status
    BatteryStatus = 0x05,
    /// Alert
    Alert = 0x06,
    /// Get_Country_Info
    GetCountryInfo = 0x07,
    /// Enter_USB
    EnterUsb = 0x08,
    /// EPR_Request
    EprRequest = 0x09,
    /// EPR_Mode
    EprMode = 0x0A,
    /// Source_Info
    SourceInfo = 0x0B,
    /// Revision
    Revision = 0x0C,
    /// Vendor_Defined
    VendorDefined = 0x0F,
}

/// USB PD extended message types (used with the `Extended` header bit).
///
/// An extended message carries a 16-bit [`ExtendedHeader`] followed by up
/// to [`MAX_EXTENDED_DATA`] payload bytes; long payloads are transmitted as
/// [`MAX_CHUNK_DATA`]-byte chunks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ExtendedMsgType {
    /// Source_Capabilities_Extended
    SourceCapabilitiesExtended = 0x03,
    /// Security_Response
    SecurityResponse = 0x04,
    /// Get_Source_Capabilities_Extended
    GetSourceCapabilitiesExtended = 0x05,
    /// Get_Sink_Capabilities_Extended
    GetSinkCapabilitiesExtended = 0x06,
    /// Manufacturer_Info
    ManufacturerInfo = 0x07,
    /// EPR_Source_Capabilities (a list of EPR APDOs)
    EprSourceCapabilities = 0x08,
    /// EPR_Get_Info
    EprGetInfo = 0x09,
    /// EPR_Info
    EprInfo = 0x0A,
    /// Sink_Capabilities_Extended
    SinkCapabilitiesExtended = 0x10,
    /// Country_Codes
    CountryCodes = 0x11,
    /// Country_Info
    CountryInfo = 0x12,
    /// Source_Info_Extended
    SourceInfoExtended = 0x13,
    /// Vendor_Defined_Extended
    VendorDefinedExtended = 0x1F,
}

/// USB PD extended-message header (16 bits, right after the message header).
///
/// ```text
/// bit[8:0]   Data size (total payload bytes across all chunks)
/// bit9       Reserved
/// bit10      Request chunk flag
/// bit[14:11] Chunk number (0 for the first chunk)
/// bit15      Chunked (1 = more chunks follow)
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExtendedHeader(u16);

impl ExtendedHeader {
    /// Build an extended header for `data_size` payload bytes.
    pub const fn new(data_size: u16) -> Self {
        Self(data_size & 0x1FF)
    }

    /// Build an extended header from a raw 16-bit value.
    pub const fn from_bits(bits: u16) -> Self {
        Self(bits)
    }

    /// The raw 16-bit extended-header value.
    pub const fn bits(self) -> u16 {
        self.0
    }

    /// Total payload size in bytes (bits `[8:0]`).
    pub const fn data_size(self) -> u16 {
        self.0 & 0x1FF
    }

    /// Request-chunk flag (bit10).
    pub const fn request_chunk(self) -> bool {
        self.0 & (1 << 10) != 0
    }

    /// Chunk number (bits `[14:11]`, 0..=15).
    pub const fn chunk_number(self) -> u8 {
        ((self.0 >> 11) & 0xF) as u8
    }

    /// Chunked flag (bit15): 1 = more chunks follow.
    pub const fn chunked(self) -> bool {
        self.0 & 0x8000 != 0
    }

    /// Set the payload size (bits `[8:0]`).
    pub const fn with_data_size(self, v: u16) -> Self {
        Self((self.0 & !0x1FF) | (v & 0x1FF))
    }

    /// Set the request-chunk flag (bit10).
    pub const fn with_request_chunk(self, v: bool) -> Self {
        if v {
            Self(self.0 | (1 << 10))
        } else {
            Self(self.0 & !(1 << 10))
        }
    }

    /// Set the chunk number (bits `[14:11]`).
    pub const fn with_chunk_number(self, v: u8) -> Self {
        Self((self.0 & !(0xF << 11)) | (((v as u16) & 0xF) << 11))
    }

    /// Set the chunked flag (bit15).
    pub const fn with_chunked(self, v: bool) -> Self {
        if v {
            Self(self.0 | 0x8000)
        } else {
            Self(self.0 & !0x8000)
        }
    }
}

impl Default for ExtendedHeader {
    fn default() -> Self {
        Self::new(0)
    }
}

/// PD protocol error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdError {
    /// Message too long (more than 7 data objects / 30 bytes).
    TooManyObjects,
    /// Destination buffer too small.
    BufferTooSmall,
    /// No GoodCRC received before the timeout (after retries).
    NoAck,
    /// Extended payload longer than [`MAX_EXTENDED_DATA`] bytes.
    TooLong,
    /// A single extended chunk longer than [`MAX_CHUNK_DATA`] bytes.
    ChunkTooLong,
    /// Out-of-order chunk in a chunked extended message.
    ChunkOutOfOrder,
}

/// An outgoing PD message: a [`Header`] plus up to [`MAX_DATA_OBJECTS`]
/// data objects.
///
/// Build one with [`Message::control`] or [`Message::data`], then send it
/// with [`super::port::Port::send_message`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Message {
    header: Header,
    objects: [u32; MAX_DATA_OBJECTS],
    num_objects: u8,
}

impl Message {
    /// A control message (no data objects).
    pub const fn control(
        msg_type: ControlMessage,
        power: PowerRole,
        data: DataRole,
        spec: SpecRev,
    ) -> Self {
        Self {
            header: Header::new()
                .with_message_type(msg_type as u8)
                .with_port_power_role(power)
                .with_port_data_role(data)
                .with_spec_revision(spec),
            objects: [0; MAX_DATA_OBJECTS],
            num_objects: 0,
        }
    }

    /// A data message carrying the given data objects.
    ///
    /// # Errors
    ///
    /// [`PdError::TooManyObjects`] if `objects.len() > MAX_DATA_OBJECTS`.
    pub fn data(
        msg_type: DataMessage,
        power: PowerRole,
        data: DataRole,
        spec: SpecRev,
        objects: &[u32],
    ) -> Result<Self, PdError> {
        if objects.len() > MAX_DATA_OBJECTS {
            return Err(PdError::TooManyObjects);
        }
        let mut arr = [0u32; MAX_DATA_OBJECTS];
        arr[..objects.len()].copy_from_slice(objects);
        Ok(Self {
            header: Header::new()
                .with_message_type(msg_type as u8)
                .with_port_power_role(power)
                .with_port_data_role(data)
                .with_spec_revision(spec),
            objects: arr,
            num_objects: objects.len() as u8,
        })
    }

    /// The message header (its number-of-objects field is filled in by
    /// [`encode`](Self::encode)).
    pub const fn header(&self) -> Header {
        self.header
    }

    /// The data objects carried by the message.
    pub fn objects(&self) -> &[u32] {
        &self.objects[..self.num_objects as usize]
    }

    /// Encode the message into `buf` (header + data objects, little-endian).
    ///
    /// Returns the encoded length in bytes (`2 + 4 * num_objects`). `buf`
    /// must be 4-byte aligned (the DMA engine reads it directly); the
    /// header's NDO field is set here, its MsgID field is left for the link
    /// layer to inject.
    ///
    /// # Errors
    ///
    /// [`PdError::BufferTooSmall`] if `buf` is too small.
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, PdError> {
        let len = 2 + self.num_objects as usize * 4;
        if buf.len() < len {
            return Err(PdError::BufferTooSmall);
        }
        let header = self.header.with_num_data_objects(self.num_objects);
        buf[0] = header.bits() as u8;
        buf[1] = (header.bits() >> 8) as u8;
        for (i, o) in self.objects[..self.num_objects as usize].iter().enumerate() {
            let off = 2 + i * 4;
            buf[off..off + 4].copy_from_slice(&o.to_le_bytes());
        }
        Ok(len)
    }
}

/// A PD message received from the bus.
///
/// Data objects are copied out of the DMA buffer, so this type owns its data
/// and carries no lifetime.
#[derive(Clone, Copy, Debug)]
pub struct RxMessage {
    header: Header,
    objects: [u32; MAX_DATA_OBJECTS],
    num_objects: u8,
}

impl RxMessage {
    /// The parsed message header.
    pub const fn header(&self) -> Header {
        self.header
    }

    /// Number of data objects in the message.
    pub const fn num_data_objects(&self) -> u8 {
        self.num_objects
    }

    /// The data objects of the message.
    pub fn objects(&self) -> &[u32] {
        &self.objects[..self.num_objects as usize]
    }

    /// Read the `i`-th data object.
    ///
    /// Panics if `i >= num_data_objects()`.
    pub fn object(&self, i: usize) -> u32 {
        self.objects[i]
    }

    /// Decode a received DMA buffer into an [`RxMessage`].
    ///
    /// The header's NDO field determines how many data-object bytes are
    /// copied. Returns `None` if the buffer is too short.
    pub fn decode(rx: &[u8]) -> Option<Self> {
        if rx.len() < 2 {
            return None;
        }
        let header = Header::from_bits(u16::from_le_bytes([rx[0], rx[1]]));
        let n = header.num_data_objects() as usize;
        if n > MAX_DATA_OBJECTS || rx.len() < 2 + n * 4 {
            return None;
        }
        let mut objects = [0u32; MAX_DATA_OBJECTS];
        for (i, o) in objects[..n].iter_mut().enumerate() {
            let off = 2 + i * 4;
            *o = u32::from_le_bytes([rx[off], rx[off + 1], rx[off + 2], rx[off + 3]]);
        }
        Some(Self {
            header,
            objects,
            num_objects: n as u8,
        })
    }
}

/// Build one extended-message chunk into `tx`.
///
/// Layout: 2-byte message header (Extended set, NDO = 7) + 2-byte
/// [`ExtendedHeader`] + up to [`MAX_CHUNK_DATA`] data bytes. The header's
/// MsgID is injected by the link layer; the extended bit and NDO are set
/// here.
///
/// Returns the chunk length in bytes (`4 + chunk_data.len()`).
///
/// # Errors
///
/// [`PdError::ChunkTooLong`] if `chunk_data.len() > MAX_CHUNK_DATA`,
/// [`PdError::BufferTooSmall`] if `tx` is too small.
pub fn encode_extended_chunk(
    tx: &mut [u8],
    header: Header,
    ext_header: ExtendedHeader,
    chunk_data: &[u8],
) -> Result<usize, PdError> {
    if chunk_data.len() > MAX_CHUNK_DATA {
        return Err(PdError::ChunkTooLong);
    }
    let len = 4 + chunk_data.len();
    if tx.len() < len {
        return Err(PdError::BufferTooSmall);
    }
    let header = header.with_extended(true).with_num_data_objects(7);
    tx[0] = header.bits() as u8;
    tx[1] = (header.bits() >> 8) as u8;
    tx[2] = ext_header.bits() as u8;
    tx[3] = (ext_header.bits() >> 8) as u8;
    tx[4..len].copy_from_slice(chunk_data);
    Ok(len)
}

/// A received extended-message chunk.
///
/// The chunk payload is copied out of the DMA buffer, so this type owns its
/// data and carries no lifetime.
#[derive(Clone, Copy, Debug)]
pub struct RxExtendedChunk {
    header: Header,
    ext_header: ExtendedHeader,
    data: [u8; MAX_CHUNK_DATA],
    data_len: u8,
}

impl RxExtendedChunk {
    /// The parsed message header.
    pub const fn header(&self) -> Header {
        self.header
    }

    /// The parsed extended header (chunk number / chunked / data size).
    pub const fn extended_header(&self) -> ExtendedHeader {
        self.ext_header
    }

    /// The chunk's data bytes (everything after the two headers).
    pub fn data(&self) -> &[u8] {
        &self.data[..self.data_len as usize]
    }
}

/// Parse a received DMA buffer as an extended-message chunk.
///
/// Returns `None` if the buffer is too short, the message header is not an
/// extended message, or the chunk payload exceeds [`MAX_CHUNK_DATA`]. The
/// chunk's data length is whatever remains after the two headers (bound it
/// with the PHY `received_len()`).
pub fn parse_extended_chunk(rx: &[u8]) -> Option<RxExtendedChunk> {
    if rx.len() < 4 {
        return None;
    }
    let header = Header::from_bits(u16::from_le_bytes([rx[0], rx[1]]));
    if !header.extended() {
        return None;
    }
    let ext_header = ExtendedHeader::from_bits(u16::from_le_bytes([rx[2], rx[3]]));
    let payload = &rx[4..];
    if payload.len() > MAX_CHUNK_DATA {
        return None;
    }
    let mut data = [0u8; MAX_CHUNK_DATA];
    data[..payload.len()].copy_from_slice(payload);
    Some(RxExtendedChunk {
        header,
        ext_header,
        data,
        data_len: payload.len() as u8,
    })
}

/// Reassembles a chunked extended message into a contiguous buffer.
///
/// Feed each received chunk with [`push`](Self::push); the data objects are
/// accumulated in place until the total size announced in the extended
/// header is reached, then [`data`](Self::data) holds the whole payload.
///
/// # Example
///
/// ```ignore
/// let mut asm = ChunkedAssembler::new();
/// loop {
///     if let Some(chunk) = parse_extended_chunk(rx) {
///         if asm.push(chunk.header(), chunk.extended_header(), chunk.data()).unwrap() {
///             break; // asm.data() now holds the whole payload
///         }
///     }
/// }
/// ```
pub struct ChunkedAssembler {
    buf: [u8; MAX_EXTENDED_DATA],
    len: usize,
    expected: u16,
    next_chunk: u8,
    msg_type: Option<u8>,
    in_progress: bool,
}

impl ChunkedAssembler {
    /// An idle assembler (nothing in progress).
    pub const fn new() -> Self {
        Self {
            buf: [0; MAX_EXTENDED_DATA],
            len: 0,
            expected: 0,
            next_chunk: 0,
            msg_type: None,
            in_progress: false,
        }
    }

    /// Reset to idle, discarding any partial message.
    pub fn reset(&mut self) {
        self.len = 0;
        self.expected = 0;
        self.next_chunk = 0;
        self.msg_type = None;
        self.in_progress = false;
    }

    /// Whether a message is being assembled.
    pub const fn is_in_progress(&self) -> bool {
        self.in_progress
    }

    /// The extended message type being assembled (after the first chunk).
    pub const fn message_type(&self) -> Option<u8> {
        self.msg_type
    }

    /// Feed one received chunk.
    ///
    /// Returns `Ok(true)` when the message is complete and available via
    /// [`data`](Self::data). On an out-of-order or oversized chunk the
    /// assembler resets itself and returns an error.
    pub fn push(
        &mut self,
        header: Header,
        ext_header: ExtendedHeader,
        data: &[u8],
    ) -> Result<bool, PdError> {
        if !self.in_progress {
            // Start a new message.
            self.expected = ext_header.data_size();
            if self.expected as usize > MAX_EXTENDED_DATA {
                self.reset();
                return Err(PdError::TooLong);
            }
            self.msg_type = Some(header.message_type());
            self.next_chunk = 0;
            self.len = 0;
            self.in_progress = true;
        }
        if ext_header.chunk_number() != self.next_chunk {
            self.reset();
            return Err(PdError::ChunkOutOfOrder);
        }
        if self.len + data.len() > self.expected as usize {
            self.reset();
            return Err(PdError::TooLong);
        }
        self.buf[self.len..self.len + data.len()].copy_from_slice(data);
        self.len += data.len();
        self.next_chunk += 1;
        if self.len as u16 == self.expected {
            self.in_progress = false;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// The assembled payload (valid after [`push`](Self::push) returns
    /// `Ok(true)`).
    pub fn data(&self) -> &[u8] {
        &self.buf[..self.len]
    }
}

impl Default for ChunkedAssembler {
    fn default() -> Self {
        Self::new()
    }
}

/// Power Data Object type (PDO bits `[31:30]`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdoType {
    /// Fixed supply
    FixedSupply = 0b00,
    /// Variable supply
    VariableSupply = 0b01,
    /// Battery
    Battery = 0b10,
    /// Adjustable PDO (APDO, PD 3.0 PPS)
    Apdo = 0b11,
}

/// A Power Data Object — one entry of a Source_Capabilities message.
///
/// Data objects follow the standard USB PD layout.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pdo(u32);

impl Pdo {
    /// Build a PDO from its raw 32-bit value.
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// The raw 32-bit PDO value.
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Build a fixed-supply PDO.
    ///
    /// # Panics
    ///
    /// Debug builds panic if `voltage_mv` (50 mV units) or `max_current_ma`
    /// (10 mA units) overflow their fields.
    pub const fn fixed(voltage_mv: u16, max_current_ma: u16) -> Self {
        let v = (voltage_mv / 50) as u32 & 0x3FF;
        let i = (max_current_ma / 10) as u32 & 0x3FF;
        debug_assert!(voltage_mv / 50 <= 0x3FF, "voltage out of range");
        debug_assert!(max_current_ma / 10 <= 0x3FF, "current out of range");
        Self((v << 10) | i)
    }

    /// Build a SPR AVS (PD 3.1) APDO.
    ///
    /// The voltage range is fixed at 9–20 V; the two fields hold the maximum
    /// current in the 9–15 V band (bits `[19:10]`, 10 mA units) and the
    /// 15–20 V band (bits `[9:0]`, 10 mA units).
    pub const fn spr_avs(current_9_15v_ma: u16, current_15_20v_ma: u16) -> Self {
        let c1 = (current_9_15v_ma / 10) as u32 & 0x3FF;
        let c2 = (current_15_20v_ma / 10) as u32 & 0x3FF;
        Self((0b11u32 << 30) | (0b10u32 << 28) | (c1 << 10) | c2)
    }

    /// Build an EPR AVS (PD 3.1) APDO.
    ///
    /// `min_mv`/`max_mv` are the adjustable range (bits `[15:8]` / `[25:17]`,
    /// 100 mV units) and `pd_power_w` the rated power in watts (bits `[7:0]`).
    pub const fn epr_avs(min_mv: u16, max_mv: u16, pd_power_w: u16) -> Self {
        let minv = (min_mv / 100) as u32 & 0xFF;
        let maxv = (max_mv / 100) as u32 & 0x1FF;
        let pdp = (pd_power_w as u32) & 0xFF;
        Self((0b11u32 << 30) | (0b01u32 << 28) | (maxv << 17) | (minv << 8) | pdp)
    }

    /// The PDO type (bits `[31:30]`).
    pub const fn pdo_type(self) -> PdoType {
        match self.0 >> 30 {
            0 => PdoType::FixedSupply,
            1 => PdoType::VariableSupply,
            2 => PdoType::Battery,
            _ => PdoType::Apdo,
        }
    }

    /// Fixed-supply voltage in millivolts (bits `[19:10]`, 50 mV units).
    ///
    /// Only meaningful for [`PdoType::FixedSupply`].
    pub const fn voltage_mv(self) -> u16 {
        (((self.0 >> 10) & 0x3FF) * 50) as u16
    }

    /// APDO (PPS) minimum voltage in millivolts (bits `[15:8]`, 100 mV
    /// units).
    ///
    /// Only meaningful for [`PdoType::Apdo`].
    pub const fn min_voltage_mv(self) -> u16 {
        (((self.0 >> 8) & 0xFF) * 100) as u16
    }

    /// APDO (PPS) maximum voltage in millivolts (bits `[24:17]`, 100 mV
    /// units).
    ///
    /// Only meaningful for [`PdoType::Apdo`].
    pub const fn max_voltage_mv(self) -> u16 {
        (((self.0 >> 17) & 0xFF) * 100) as u16
    }

    /// Maximum current in milliamps.
    ///
    /// Fixed supply: bits `[9:0]` in 10 mA units. APDO (PPS): bits
    /// `[6:0]` in 50 mA units. 0 for other types.
    pub const fn max_current_ma(self) -> u16 {
        match self.pdo_type() {
            PdoType::FixedSupply => ((self.0 & 0x3FF) * 10) as u16,
            PdoType::Apdo => ((self.0 & 0x7F) * 50) as u16,
            _ => 0,
        }
    }

    /// The APDO supply type (bits `[29:28]`).
    ///
    /// Only meaningful for [`PdoType::Apdo`].
    pub const fn apdo_type(self) -> ApdoType {
        match (self.0 >> 28) & 0x3 {
            0 => ApdoType::Pps,
            1 => ApdoType::EprAvs,
            _ => ApdoType::SprAvs,
        }
    }

    /// EPR AVS minimum voltage in millivolts (bits `[15:8]`, 100 mV units).
    ///
    /// Only meaningful for an [`ApdoType::EprAvs`] PDO.
    pub const fn epr_min_voltage_mv(self) -> u16 {
        (((self.0 >> 8) & 0xFF) * 100) as u16
    }

    /// EPR AVS maximum voltage in millivolts (bits `[25:17]`, 100 mV units).
    ///
    /// Only meaningful for an [`ApdoType::EprAvs`] PDO.
    pub const fn epr_max_voltage_mv(self) -> u16 {
        (((self.0 >> 17) & 0x1FF) * 100) as u16
    }

    /// EPR AVS rated power in watts (bits `[7:0]`).
    ///
    /// Only meaningful for an [`ApdoType::EprAvs`] PDO.
    pub const fn epr_pd_power_w(self) -> u16 {
        (self.0 & 0xFF) as u16
    }

    /// SPR AVS maximum current in the 9–15 V band, in mA (bits `[19:10]`,
    /// 10 mA units).
    ///
    /// Only meaningful for an [`ApdoType::SprAvs`] PDO.
    pub const fn spr_avs_current_9_15v_ma(self) -> u16 {
        (((self.0 >> 10) & 0x3FF) * 10) as u16
    }

    /// SPR AVS maximum current in the 15–20 V band, in mA (bits `[9:0]`,
    /// 10 mA units).
    ///
    /// Only meaningful for an [`ApdoType::SprAvs`] PDO.
    pub const fn spr_avs_current_15_20v_ma(self) -> u16 {
        ((self.0 & 0x3FF) * 10) as u16
    }
}

/// Augmented PDO supply type (APDO bits `[29:28]`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApdoType {
    /// Programmable power supply (PPS)
    Pps = 0b00,
    /// EPR adjustable voltage supply
    EprAvs = 0b01,
    /// SPR adjustable voltage supply
    SprAvs = 0b10,
}

/// A Request Data Object — the sink's answer to a Source_Capabilities PDO.
///
/// Build one with [`Rdo::fixed`], [`Rdo::pps`] or [`Rdo::avs`], depending on
/// the kind of PDO being requested.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rdo(u32);

impl Rdo {
    /// Build a RDO from its raw 32-bit value.
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// The raw 32-bit RDO value.
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// The 1-based index of the requested PDO (bits `[31:28]`).
    pub const fn position(self) -> u8 {
        ((self.0 >> 28) & 0xF) as u8
    }

    /// A fixed-supply request.
    ///
    /// `operating_ma`/`max_ma` are the operating and maximum currents
    /// (10 mA units, bits `[19:10]` / `[9:0]`).
    pub const fn fixed(position: u8, operating_ma: u16, max_ma: u16) -> Self {
        let pos = ((position as u32) & 0xF) << 28;
        let op = ((operating_ma / 10) as u32) & 0x3FF;
        let max = ((max_ma / 10) as u32) & 0x3FF;
        Self(pos | (op << 10) | max)
    }

    /// A PPS (programmable power supply) request.
    ///
    /// `output_mv` is the requested output voltage (20 mV units, bits
    /// `[20:9]`) and `current_ma` the operating current (50 mA units, bits
    /// `[6:0]`).
    pub const fn pps(position: u8, output_mv: u16, current_ma: u16) -> Self {
        let pos = ((position as u32) & 0xF) << 28;
        let v = ((output_mv / 20) as u32) & 0xFFF;
        let c = ((current_ma / 50) as u32) & 0x7F;
        Self(pos | (v << 9) | c)
    }

    /// An AVS (adjustable voltage supply) request.
    ///
    /// `output_mv` is the requested output voltage (25 mV units, bits
    /// `[20:9]`) and `current_ma` the operating current (50 mA units, bits
    /// `[6:0]`).
    pub const fn avs(position: u8, output_mv: u16, current_ma: u16) -> Self {
        let pos = ((position as u32) & 0xF) << 28;
        let v = ((output_mv / 25) as u32) & 0xFFF;
        let c = ((current_ma / 50) as u32) & 0x7F;
        Self(pos | (v << 9) | c)
    }

    /// Fixed-supply operating current in mA (bits `[19:10]`, 10 mA units).
    pub const fn operating_current_ma(self) -> u16 {
        (((self.0 >> 10) & 0x3FF) * 10) as u16
    }

    /// Fixed-supply maximum current in mA (bits `[9:0]`, 10 mA units).
    pub const fn max_current_ma(self) -> u16 {
        ((self.0 & 0x3FF) * 10) as u16
    }

    /// PPS output voltage in mV (bits `[20:9]`, 20 mV units).
    pub const fn pps_output_voltage_mv(self) -> u16 {
        (((self.0 >> 9) & 0xFFF) * 20) as u16
    }

    /// AVS output voltage in mV (bits `[20:9]`, 25 mV units).
    pub const fn avs_output_voltage_mv(self) -> u16 {
        (((self.0 >> 9) & 0xFFF) * 25) as u16
    }

    /// PPS/AVS operating current in mA (bits `[6:0]`, 50 mA units).
    pub const fn output_current_ma(self) -> u16 {
        ((self.0 & 0x7F) * 50) as u16
    }
}

/// EPR_Mode action.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EprModeAction {
    /// Enter EPR mode
    Enter = 0x01,
    /// Enter EPR mode acknowledged
    EnterAcknowledged = 0x02,
    /// Enter EPR mode succeeded
    EnterSucceeded = 0x03,
    /// Enter EPR mode failed
    EnterFailed = 0x04,
    /// Exit EPR mode
    Exit = 0x05,
}

/// Build an EPR_Mode data object: `action` in bits `[31:24]` with an 8-bit
/// payload in bits `[23:16]`.
pub const fn make_epr_mode(action: EprModeAction, data: u8) -> u32 {
    ((action as u32) << 24) | ((data as u32) << 16)
}

/// Build an EPR_Request payload: the Request Data Object followed by a copy
/// of the requested PDO (for source verification).
///
/// Returns the two data objects to pass to [`Message::data`] with message
/// type [`DataMessage::EprRequest`].
pub const fn make_epr_request(rdo: u32, pdo: u32) -> [u32; 2] {
    [rdo, pdo]
}
