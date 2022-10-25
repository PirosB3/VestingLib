mod types;
mod traits;
mod errors;
pub use types::{Vesting, VestingTimeline, UnixVestingTimeline};
pub use traits::{IsVestingSchedule, GetReleasableAmountParams};
pub use errors::{VestingError};


#[cfg(test)]
mod tests {
    use crate::{Vesting, types::{VestingState, VestingTerms}, VestingTimeline, IsVestingSchedule, GetReleasableAmountParams};

    #[test]
    fn it_works() {
        let vesting = Vesting{
            state: VestingState{
                amount_already_issued: 0,
                initialized: true,
                revoked: false,
            },
            terms: VestingTerms{
                // amount: 100_000,
                amount: u64::MAX,
                revokable: true,
                user: format!("H8"),
                timeline: VestingTimeline {
                    cliff_seconds: 31540000,
                    start_unix: 1644062400,
                    duration_seconds: (31540000 * 4),
                    seconds_per_slice: 2628000,
                }
            }
        };
        let mut current_time = 1666614761;
        let mut last_value = 0;
        let mut passed_cliff = false;
        loop {
            let results = vesting.get_releasable_amount(&GetReleasableAmountParams{
                current_time,
            }).unwrap();
            passed_cliff = passed_cliff || results != 0;
            if passed_cliff {
                assert!(last_value <= results);
            }
            last_value = results;
            current_time += 1296000;
            if results == u64::MAX {break;}
        }
    }
}
