# VestingLib


## What is this library?

VestingLib is a tiny library for computing a vesting schedule for a beneficiary. Developers can initialize a Vesting object by passing in the configuration parameters and compute the total amount of tokens that can be released for the beneficiary.

## Why did you create this library?

I'm doing a begineer's workshop on Anchor programmin at [Solana Breakpoint 2022](https://solana.com/breakpoint). I'll be covering basic Solana concepts by going through a practical example: creating a token vesting contract. I want the class to focus on the Solana concepts and less on the math/business logic - so I created this library to abstract the core vesting calculation away from the students.

Students can import this library and focus more on developing the Solana program.

## Example

Below is some sample code of how the library is to be used:

```rust
// Initialize a vesting instance based on the vesting parameters
let vesting_schedule = Vesting::from_init_params(&VestingInitParams {
    cliff_seconds: 31560000,         // One year in seconds
    duration_seconds: 126240000,     // Four years in seconds
    seconds_per_slice: 2628000,      // One month in seconds,
    start_unix: 1666743504,          // Grant start time
    already_issued_token_amount: 0,  // No tokens were already issued
    grant_token_amount: 100,         // Grant is 100 tokens
    revoked: false,                  // If true, marks grant as revoked
})?;

// Get the current time
let current_time_unix = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

// Returns the amount releasable by the owner of the grant.
let releasable_amount = vesting_schedule.get_releasable_amount(&GetReleasableAmountParams{
    current_time_unix, 
})?;

```