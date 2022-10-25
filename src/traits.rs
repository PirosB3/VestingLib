use crate::types::{VestingState, VestingTerms};
use crate::VestingError;
use crate::{Vesting, VestingTimeline};

/// The parameters needed to initialize the vesting
/// Library is inspired by https://github.com/abdelhamidbakhta/token-vesting-contracts/blob/main/contracts/TokenVesting.sol
pub struct VestingInitParams {
    /// The vesting UNIX start time in seconds
    pub start_unix: u64,
    /// The duration of the cliff in seconds
    pub cliff_seconds: u64,
    /// The duration of overall grant (must be >= cliff)
    pub duration_seconds: u64,
    /// The duration of a slice period for the vesting in seconds
    pub seconds_per_slice: u64,
    /// The total number of tokens issued for the duration of the grant
    pub grant_token_amount: u64,
    /// The number of tokens already issued to the user (<= grant_token_amount)
    pub already_issued_token_amount: u64,
    /// Marks if the grant was revoked by an admin
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
            grant_token_amount,
            ..
        } = *params;
        if already_issued_token_amount > grant_token_amount {
            return Err(VestingError::ConfigurationError(
                "Tokens issued are greater than the total grant",
            ));
        }
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
