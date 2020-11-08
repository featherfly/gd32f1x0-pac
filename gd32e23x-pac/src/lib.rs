//! Peripheral access API for GD32E23X microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.17.0)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [gd32-rs](https://github.com/gd32-rs/gd32-rs)
//!
//! This crate supports all GD32E23X devices; for the complete list please
//! see:
//! [gd32e23x](https://github.com/gd32-rs/gd32-rs/tree/master/gd32e23x)
//!
//! Due to doc build limitations, not all devices may be shown on docs.rs;
//! a representative few have been selected instead. For a complete list of
//! available registers and fields see: [gd32-rs Device Coverage](https://gd32.agg.io/rs)

#![allow(non_camel_case_types)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "gd32e230")]
pub mod gd32e230;

// #[cfg(feature = "gd32e231")]
// pub mod gd32e231;

// #[cfg(feature = "gd32e232")]
// pub mod gd32e232;
