// Constitutional fault taxonomy. Every possible failure is classified here.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum ConstitutionalFault {
    // Capacity & Resource (0x1XXX)
    CapacityExceeded = 0x1001,
    TableFull = 0x1002,
    MemoryBudgetExhausted = 0x1003,
    BufferTooSmall = 0x1004,
    // State Transition (0x2XXX)
    InvalidStateTransition = 0x2001,
    UninitializedAccess = 0x2002,
    DoubleInitialization = 0x2003,
    // Temporal & Ordering (0x3XXX)
    TemporalViolation = 0x3001,
    ReplayViolation = 0x3002,
    SequenceMismatch = 0x3003,
    // Byzantine & Security (0x4XXX)
    EquivocationDetected = 0x4001,
    InvalidQuorum = 0x4002,
    SignatureInvalid = 0x4003,
    MerkleProofInvalid = 0x4004,
    // Durability (0x5XXX)
    DurabilityViolation = 0x5001,
    JournalHashMismatch = 0x5002,
    // Contract & Invariant (0x6XXX)
    UnsafeContractViolation = 0x6001,
    ConstitutionalViolation = 0x6002,
    // Encoding (0x7XXX)
    TrailingBytesDetected = 0x7001,
    MalformedEncoding = 0x7002,
    EncodingVersionMismatch = 0x7003,
    // Arithmetic (0x8XXX)
    ArithmeticOverflow = 0x8001,
    ArithmeticUnderflow = 0x8002,
    // Generic Input (0xAXXX)
    InvalidInput = 0xA001,
    // Budget (0x9XXX)
    DecodeBudgetExceeded = 0x9001,
    CryptoBudgetExceeded = 0x9002,
}

impl ConstitutionalFault {
    pub const fn severity(self) -> FaultSeverity {
        match self {
            Self::CapacityExceeded
            | Self::TableFull
            | Self::BufferTooSmall
            | Self::MemoryBudgetExhausted => FaultSeverity::Degraded,
            Self::InvalidStateTransition
            | Self::UninitializedAccess
            | Self::DoubleInitialization
            | Self::TemporalViolation
            | Self::ReplayViolation
            | Self::SequenceMismatch => FaultSeverity::Rejected,
            _ => FaultSeverity::Critical,
        }
    }

    pub const fn should_halt(self) -> bool {
        matches!(self.severity(), FaultSeverity::Critical)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum FaultSeverity {
    Degraded = 0,
    Rejected = 1,
    Critical = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Subsystem {
    Kernel = 0x00,
    Codec = 0x01,
    State = 0x02,
    Consensus = 0x03,
    Network = 0x04,
    Storage = 0x05,
    Crypto = 0x06,
    Constitution = 0x07,
    Evidence = 0x08,
}

pub mod module_ids {
    pub const AMUN_UNSAFE: u16 = 0x0001;
    pub const AMUN_FAILURE: u16 = 0x0002;
    pub const AMUN_KERNEL_TYPES: u16 = 0x0003;
    pub const AMUN_CODEC: u16 = 0x0004;
    pub const AMUN_STATE_TYPES: u16 = 0x0005;
    pub const AMUN_CONSTITUTION: u16 = 0x0006;
    pub const AMUN_EVIDENCE: u16 = 0x0007;
    pub const AMUN_EXECUTION: u16 = 0x0008;
}

pub mod operation_ids {
    pub const RAW_SLOT_WRITE: u16 = 0x0101;
    pub const RAW_SLOT_TAKE: u16 = 0x0102;
    pub const RAW_SLOT_REPLACE: u16 = 0x0103;
    pub const RAW_SLOT_GET: u16 = 0x0104;
    pub const ENCODE: u16 = 0x0201;
    pub const DECODE: u16 = 0x0202;
    pub const DECODE_EXACT: u16 = 0x0203;
    pub const BUFFER_WRITE: u16 = 0x0204;
    pub const HASHER_WRITE: u16 = 0x0205;
    pub const ENCODE_SEQUENCE: u16 = 0x0206;
    pub const ENCODE_SET: u16 = 0x0207;
    pub const ENCODE_MAP: u16 = 0x0208;
    pub const DECODE_HEADER: u16 = 0x0209;
    pub const KERNEL_CHECK_HEALTH: u16 = 0x0301;
    pub const CCF_MISMATCH: u16 = 0x0302;
    pub const CAPACITY_MISMATCH: u16 = 0x0303;
    pub const QUORUM_TRANSITION: u16 = 0x0304;
    pub const QUORUM_CONTINUITY: u16 = 0x0305;
    pub const EVIDENCE_VERIFY: u16 = 0x0401;
    pub const CANONICAL_SORT: u16 = 0x0501;
    pub const BUDGET_BYTES: u16 = 0x0601;
    pub const BUDGET_OBJECT: u16 = 0x0602;
    pub const BUDGET_DEPTH: u16 = 0x0603;
    pub const BUDGET_HASH: u16 = 0x0604;
    pub const BUDGET_SIGNATURE: u16 = 0x0605;
    pub const SYNCHRONY_VERIFY: u16 = 0x0701;
    pub const TIMEOUT_VERIFY: u16 = 0x0702;
    pub const CONSTITUTION_MISMATCH: u16 = 0x0801;
    pub const TRANSACTION_BEGIN: u16 = 0x0901;
    pub const TRANSACTION_COMMIT: u16 = 0x0902;
    pub const SHADOW_WRITE: u16 = 0x0903;
    pub const WEIGHTED_QUORUM_TRANSITION: u16 = 0x0A01;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FailureContext {
    pub fault: ConstitutionalFault,
    pub module_id: u16,
    pub operation_id: u16,
}

impl FailureContext {
    pub const fn new(fault: ConstitutionalFault, module_id: u16, operation_id: u16) -> Self {
        Self {
            fault,
            module_id,
            operation_id,
        }
    }

    pub fn severity(&self) -> FaultSeverity {
        self.fault.severity()
    }

    pub fn should_halt(&self) -> bool {
        self.fault.should_halt()
    }
}

pub type AmunResult<T> = Result<T, FailureContext>;
