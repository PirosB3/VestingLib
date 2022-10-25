#[derive(Debug, Clone)]
pub struct VestingTimeline {
    pub cliff_seconds: u64,
    pub start_unix: u64,
    pub duration_seconds: u64,
    pub seconds_per_slice: u64,
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
    pub revokable: bool,
    pub amount: u64,
    pub user: String,
}

#[derive(Debug, Clone)]
pub struct Vesting {
    pub terms: VestingTerms,
    pub state: VestingState,
}

#[derive(Debug, Clone)]
pub struct VestingState {
    pub initialized: bool,
    pub revoked: bool,
    pub amount_already_issued: u64,
}