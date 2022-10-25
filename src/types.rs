use crate::errors::{VestingError};

const THIRTY_DAYS: u64 = 2592000;

#[derive(Debug, Clone)]
pub struct VestingTimeline {
    pub cliff_seconds: u64,
    pub start_unix: u64,
    pub duration_seconds: u64,
    pub seconds_per_slice: u64,
}

pub struct VestingTimelineInitParams {
    pub start_unix: u64,
    pub cliff_seconds: u64,
    pub duration_seconds: u64,
    pub seconds_per_slice: u64,
}

impl VestingTimeline {
    pub fn new(params: VestingTimelineInitParams) -> Result<Self, VestingError> {
        let VestingTimelineInitParams{cliff_seconds, duration_seconds, seconds_per_slice, start_unix} = params;
        if duration_seconds < cliff_seconds {
            return Err(VestingError::ConfigurationError("Grant duration is less than the cliff"));
        }
        if seconds_per_slice <= 0 || seconds_per_slice > THIRTY_DAYS {
            return Err(VestingError::ConfigurationError("Slice must be > 0 and <= 30 days"));
        }
        if seconds_per_slice > duration_seconds {
            return Err(VestingError::ConfigurationError("Slice must < than grant length"));
        }
        Ok(Self { cliff_seconds, start_unix, duration_seconds, seconds_per_slice })
    }
}

pub struct UnixVestingTimeline {
    pub start_unix: u64,
    pub cliff_unix: u64,
    pub end_unix: u64,
}

impl VestingTimeline {
    pub fn get_unix_timeline(&self) -> UnixVestingTimeline {
        let cliff_unix = self.start_unix + self.cliff_seconds;
        let end_unix = self.start_unix + self.duration_seconds;
        UnixVestingTimeline { start_unix: self.start_unix, cliff_unix, end_unix }
    }
}

#[derive(Debug, Clone)]
pub struct VestingTerms {
    pub timeline: VestingTimeline,
    pub amount: u64,
}

#[derive(Debug, Clone)]
pub struct Vesting {
    pub terms: VestingTerms,
    pub state: VestingState,
}

#[derive(Debug, Clone)]
pub struct VestingState {
    pub revoked: bool,
    pub amount_already_issued: u64,
}