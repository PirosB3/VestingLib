mod types;
mod traits;
mod errors;
pub use types::{Vesting, VestingTimeline, UnixVestingTimeline};
pub use traits::{IsVestingSchedule, GetReleasableAmountParams};
pub use errors::{VestingError};


#[cfg(test)]
mod tests {
    use crate::{Vesting, types::{VestingState, VestingTerms, VestingTimelineInitParams}, VestingTimeline, IsVestingSchedule, GetReleasableAmountParams, VestingError};

    #[test]
    fn fails_when_params_are_inalid() {
        let invalid_params: Vec<(VestingTimelineInitParams, &'static str)> = vec![
            (VestingTimelineInitParams{cliff_seconds: 500, duration_seconds: 300, seconds_per_slice: 30, start_unix: 0}, "Grant duration is less than the cliff"),
            (VestingTimelineInitParams{cliff_seconds: 500, duration_seconds: 1000, seconds_per_slice: 4320000, start_unix: 0}, "Slice must be > 0 and <= 30 days"),
            (VestingTimelineInitParams{cliff_seconds: 500, duration_seconds: 1000, seconds_per_slice: 0, start_unix: 0}, "Slice must be > 0 and <= 30 days"),
            (VestingTimelineInitParams{cliff_seconds: 500, duration_seconds: 1000, seconds_per_slice: 1200, start_unix: 0}, "Slice must < than grant length"),
        ];
        for (param, error) in invalid_params {
            let result = VestingTimeline::new(param);
            match result {
                Err(VestingError::ConfigurationError(msg)) => assert_eq!(msg, error),
                _ => assert_eq!(3, 2)
            }
        }
    }

    #[test]
    fn fails_when_grant_is_revoked() {
        let start_unix = 1666716060;
        let cliff_seconds = 2592000;
        let duration_seconds = cliff_seconds * 4;
        let vesting = Vesting{
            state: VestingState{
                amount_already_issued: 0,
                revoked: true,
            },
            terms: VestingTerms{
                amount: u64::MAX,
                timeline: VestingTimeline::new(VestingTimelineInitParams{
                    start_unix: 1666716060,
                    cliff_seconds,
                    duration_seconds,
                    seconds_per_slice: 30,
                }).unwrap()
            }
        };
        let result = vesting.get_releasable_amount(&GetReleasableAmountParams { current_time: start_unix + (2 * cliff_seconds) });
        assert_eq!(result.err().unwrap(), VestingError::Revoked);
    }

    #[test]
    fn returns_zero_before_cliff() {
        let start_unix = 1666716060;
        let cliff_seconds = 2592000;
        let duration_seconds = cliff_seconds * 4;
        let amount = 100_000;
        let amount_already_issued: u64 = 30;
        let vesting = Vesting{
            state: VestingState{
                amount_already_issued,
                revoked: false,
            },
            terms: VestingTerms{
                amount: amount,
                timeline: VestingTimeline::new(VestingTimelineInitParams{
                    start_unix: 1666716060,
                    cliff_seconds,
                    duration_seconds,
                    seconds_per_slice: 300,
                }).unwrap()
            }
        };
        let mut current_time = start_unix - 100;
        loop {
            let result = vesting.get_releasable_amount(&GetReleasableAmountParams { current_time });
            assert!(result.is_ok());
            if let Ok(vested_amount) = result {
                if vested_amount > 0 {
                    assert!(current_time >= start_unix + cliff_seconds);
                    assert_eq!(vested_amount, (amount / 4) - amount_already_issued);
                    break;
                } else {
                    current_time += 100;
                }
            }
        }
    }

    #[test]
    fn it_returns_total_amount() {
        let start_unix = 1666716060;
        let cliff_seconds = 2592000;
        let duration_seconds = cliff_seconds * 4;
        let amount = 100_000;
        let vesting = Vesting{
            state: VestingState{
                amount_already_issued: 0,
                revoked: false,
            },
            terms: VestingTerms{
                amount: amount,
                timeline: VestingTimeline::new(VestingTimelineInitParams{
                    start_unix,
                    cliff_seconds,
                    duration_seconds,
                    seconds_per_slice: 300,
                }).unwrap()
            }
        };
        let result = vesting.get_releasable_amount(&GetReleasableAmountParams { current_time: start_unix + duration_seconds }).unwrap();
        assert_eq!(result, amount);
    }

    #[test]
    fn it_returns_gradual_amount() {
        let start_unix = 1666716060;
        let cliff_seconds = 2592000;
        let duration_seconds = cliff_seconds * 4;
        let amount = 100_000;
        let amount_already_issued: u64 = 0;
        let vesting = Vesting{
            state: VestingState{
                amount_already_issued,
                revoked: false,
            },
            terms: VestingTerms{
                amount: amount,
                timeline: VestingTimeline::new(VestingTimelineInitParams{
                    start_unix: 1666716060,
                    cliff_seconds,
                    duration_seconds,
                    seconds_per_slice: 300,
                }).unwrap()
            }
        };
        let mut last_price = 0;
        let mut current_time = start_unix + cliff_seconds;
        loop {
            let result = vesting.get_releasable_amount(&GetReleasableAmountParams { current_time });
            assert!(result.is_ok());
            if let Ok(vested_amount) = result {
                println!("{}", vested_amount);
                assert!(vested_amount > last_price);
                if vested_amount == amount {
                    break;
                }
                current_time += 300;
                last_price = vested_amount;
            }
        }
    }
}
