use crate::types::{VestingState, VestingTerms};
use crate::VestingError;
use crate::{UnixVestingTimeline, Vesting, VestingTimeline};

pub struct VestingInitParams {
    pub start_unix: u64,
    pub cliff_seconds: u64,
    pub duration_seconds: u64,
    pub seconds_per_slice: u64,
    pub grant_token_amount: u64,
    pub already_issued_token_amount: u64,
    pub revoked: bool,
}

const THIRTY_DAYS: u64 = 2592000;

pub trait CanInitialize {
    fn from_init_params(params: &VestingInitParams) -> Result<Self, VestingError>
    where
        Self: Sized;
}

impl CanInitialize for VestingTimeline {
    fn from_init_params(params: &VestingInitParams) -> Result<Self, VestingError>
    where
        Self: Sized,
    {
        let VestingInitParams {
            cliff_seconds,
            duration_seconds,
            seconds_per_slice,
            start_unix,
            ..
        } = *params;
        if duration_seconds < cliff_seconds {
            return Err(VestingError::ConfigurationError(
                "Grant duration is less than the cliff",
            ));
        }
        if seconds_per_slice <= 0 || seconds_per_slice > THIRTY_DAYS {
            return Err(VestingError::ConfigurationError(
                "Slice must be > 0 and <= 30 days",
            ));
        }
        if seconds_per_slice > duration_seconds {
            return Err(VestingError::ConfigurationError(
                "Slice must < than grant length",
            ));
        }
        Ok(Self {
            cliff_seconds,
            start_unix,
            duration_seconds,
            seconds_per_slice,
        })
    }
}

impl CanInitialize for VestingTerms {
    fn from_init_params(params: &VestingInitParams) -> Result<Self, VestingError>
    where
        Self: Sized,
    {
        let timeline = VestingTimeline::from_init_params(params)?;
        Ok(Self {
            timeline,
            amount: params.grant_token_amount,
        })
    }
}

impl CanInitialize for VestingState {
    fn from_init_params(params: &VestingInitParams) -> Result<Self, VestingError>
    where
        Self: Sized,
    {
        let VestingInitParams {
            revoked,
            already_issued_token_amount,
            ..
        } = *params;
        Ok(VestingState {
            revoked,
            amount_already_issued: already_issued_token_amount,
        })
    }
}

impl CanInitialize for Vesting {
    fn from_init_params(params: &VestingInitParams) -> Result<Self, VestingError>
    where
        Self: Sized,
    {
        let state = VestingState::from_init_params(params)?;
        let terms = VestingTerms::from_init_params(params)?;
        Ok(Self { state, terms })
    }
}
