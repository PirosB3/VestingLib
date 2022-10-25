
#[derive(Debug, Clone, PartialEq)]
pub enum VestingError {
    Revoked,
    ConfigurationError(&'static str)
}