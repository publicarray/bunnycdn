#![forbid(unsafe_code)]

extern crate chrono;
extern crate reqwest;

#[macro_use]
extern crate log;
pub mod serde_types;
pub mod storage;

// bunnycdn/0.0 (https://github.com/publicarray/bunnycdn)
static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("CARGO_PKG_HOMEPAGE"),
    ")",
);
