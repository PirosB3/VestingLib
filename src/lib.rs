mod errors;
mod traits;
mod types;
pub use errors::VestingError;
pub use types::{UnixVestingTimeline, Vesting, VestingTimeline};

#[cfg(test)]
mod tests {
    use crate::traits::CanInitialize;
    use crate::{
        traits::VestingInitParams, types::GetReleasableAmountParams, Vesting, VestingError,
        VestingTimeline,
    };

    #[test]
    fn fails_when_params_are_inalid() {
        let invalid_params: Vec<(VestingInitParams, &'static str)> = vec![
            (
                VestingInitParams {
                    cliff_seconds: 500,
                    duration_seconds: 300,
                    seconds_per_slice: 30,
                    start_unix: 0,
                    already_issued_token_amount: 0,
                    grant_token_amount: 100,
                    revoked: false,
                },
                "Grant duration is less than the cliff",
            ),
            (
                VestingInitParams {
                    cliff_seconds: 500,
                    duration_seconds: 1000,
                    seconds_per_slice: 4320000,
                    start_unix: 0,
                    already_issued_token_amount: 0,
                    grant_token_amount: 100,
                    revoked: false,
                },
                "Slice must be > 0 and <= 30 days",
            ),
            (
                VestingInitParams {
                    cliff_seconds: 500,
                    duration_seconds: 1000,
                    seconds_per_slice: 0,
                    start_unix: 0,
                    already_issued_token_amount: 0,
                    grant_token_amount: 100,
                    revoked: false,
                },
                "Slice must be > 0 and <= 30 days",
            ),
            (
                VestingInitParams {
                    cliff_seconds: 500,
                    duration_seconds: 1000,
                    seconds_per_slice: 1200,
                    start_unix: 0,
                    already_issued_token_amount: 0,
                    grant_token_amount: 100,
                    revoked: false,
                },
                "Slice must < than grant length",
            ),
        ];
        for (param, error) in invalid_params {
            let result = VestingTimeline::from_init_params(&param);
            match result {
                Err(VestingError::ConfigurationError(msg)) => assert_eq!(msg, error),
                _ => assert_eq!(3, 2),
            }
        }
    }

    #[test]
    fn fails_when_grant_is_revoked() {
        let start_unix = 1666716060;
        let cliff_seconds = 2592000;
        let params = VestingInitParams {
            already_issued_token_amount: 30,
            grant_token_amount: 100_000,
            start_unix,
            cliff_seconds: cliff_seconds,
            duration_seconds: cliff_seconds * 4,
            revoked: true,
            seconds_per_slice: 300,
        };
        let vesting = Vesting::from_init_params(&params).unwrap();
        let result = vesting.get_releasable_amount(&GetReleasableAmountParams {
            current_time: start_unix + (2 * cliff_seconds),
        });
        assert_eq!(result.err().unwrap(), VestingError::Revoked);
    }

    #[test]
    fn returns_zero_before_cliff() {
        let start_unix = 1666716060;
        let params = VestingInitParams {
            already_issued_token_amount: 30,
            grant_token_amount: 100_000,
            start_unix,
            cliff_seconds: 2592000,
            duration_seconds: 2592000 * 4,
            revoked: false,
            seconds_per_slice: 300,
        };
        let vesting = Vesting::from_init_params(&params).unwrap();

        let mut current_time = start_unix - 100;
        loop {
            let result = vesting.get_releasable_amount(&GetReleasableAmountParams { current_time });
            assert!(result.is_ok());
            if let Ok(vested_amount) = result {
                if vested_amount > 0 {
                    assert!(current_time >= params.start_unix + params.cliff_seconds);
                    assert_eq!(
                        vested_amount,
                        (params.grant_token_amount / 4) - params.already_issued_token_amount
                    );
                    break;
                } else {
                    current_time += 100;
                }
            }
        }
    }

    #[test]
    fn it_fails_if_number_of_issued_tokens_bigger_than_grant() {
        let vesting = Vesting::from_init_params(&VestingInitParams {
            start_unix: 100,
            cliff_seconds: 10,
            duration_seconds: 2000,
            seconds_per_slice: 300,
            grant_token_amount: 100_000,
            already_issued_token_amount: 500_000,
            revoked: false,
        });
        assert_eq!(vesting.err().unwrap(), VestingError::ConfigurationError("Tokens issued are greater than the total grant"));
    }

    #[test]
    fn it_returns_total_amount() {
        let start_unix = 1666716060;
        let cliff_seconds = 2592000;
        let duration_seconds = cliff_seconds * 4;
        let amount = 100_000;
        let vesting = Vesting::from_init_params(&VestingInitParams {
            start_unix,
            cliff_seconds,
            duration_seconds,
            seconds_per_slice: 300,
            grant_token_amount: amount,
            already_issued_token_amount: 0,
            revoked: false,
        })
        .unwrap();
        let result = vesting
            .get_releasable_amount(&GetReleasableAmountParams {
                current_time: start_unix + duration_seconds,
            })
            .unwrap();
        assert_eq!(result, amount);
    }

    #[test]
    fn it_returns_gradual_amount() {
        let start_unix = 1666716060;
        let cliff_seconds = 2592000;
        let duration_seconds = cliff_seconds * 4;
        let amount = 100_000;
        let amount_already_issued: u64 = 0;
        let vesting = Vesting::from_init_params(&VestingInitParams {
            start_unix,
            cliff_seconds,
            duration_seconds,
            seconds_per_slice: 300,
            grant_token_amount: amount,
            already_issued_token_amount: amount_already_issued,
            revoked: false,
        })
        .unwrap();
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
