use crate::{Vesting, UnixVestingTimeline};
use crate::VestingError;


pub trait IsVestingSchedule<P> {
    type Error;
    fn get_releasable_amount(&self, params: &P) -> Result<u64, Self::Error>;
}

pub struct GetReleasableAmountParams {
    pub current_time: u64,
}

impl IsVestingSchedule<GetReleasableAmountParams> for Vesting {
    type Error = VestingError;

    fn get_releasable_amount(&self, params: &GetReleasableAmountParams) -> Result<u64, Self::Error> {
        let GetReleasableAmountParams{current_time} = *params;
        let UnixVestingTimeline{start_unix, cliff_unix, end_unix} = self.terms.timeline.get_unix_timeline();

        // Grant was revoked. When grant is revoked, it is also fully paid out if there was
        // any vesting
        if self.state.revoked {
            return Err(Self::Error::Revoked);
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