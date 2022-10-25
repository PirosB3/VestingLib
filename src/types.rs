use crate::errors::{VestingError};


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
    pub amount: u64,
}

#[derive(Debug, Clone)]
pub struct Vesting {
    pub terms: VestingTerms,
    pub state: VestingState,
}
pub struct GetReleasableAmountParams {
    pub current_time: u64,
}

impl Vesting {
    pub fn get_releasable_amount(&self, params: &GetReleasableAmountParams) -> Result<u64, VestingError> {
        let GetReleasableAmountParams{current_time} = *params;
        let UnixVestingTimeline{start_unix, cliff_unix, end_unix} = self.terms.timeline.get_unix_timeline();

        // Grant was revoked. When grant is revoked, it is also fully paid out if there was
        // any vesting
        if self.state.revoked {
            return Err(VestingError::Revoked);
        }

        // Too early, cliff not reached yet
        if current_time < cliff_unix {
            println!("Early exit: Not reached cliff");
            return Ok(0);
        }

        // Reached to the end of the grant
        if current_time >= end_unix {
            println!("Early exit: ended");
            let remaining_amount = self.terms.amount - self.state.amount_already_issued;
            return Ok(remaining_amount);
        }

        // Cliff was reached and grant end date is not reached yet.
        let elapsed_seconds = current_time - start_unix;
        let vested_seconds = elapsed_seconds - (elapsed_seconds % self.terms.timeline.seconds_per_slice);
        let vested_amount = {
            // NOTE: There is of course some precision loss here
            let percentage_vested = vested_seconds as f64 / self.terms.timeline.duration_seconds as f64;
            (percentage_vested * self.terms.amount as f64) as u64
        };
        let remaining_amount = vested_amount - self.state.amount_already_issued;
        Ok(remaining_amount)
    }
}

#[derive(Debug, Clone)]
pub struct VestingState {
    pub revoked: bool,
    pub amount_already_issued: u64,
}