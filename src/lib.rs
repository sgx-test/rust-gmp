#![crate_name = "gmp"]

#![warn(deprecated)]
#![allow(non_camel_case_types)]

#![cfg_attr(all(feature = "mesalock_sgx", not(target_env = "sgx")), no_std)]
#![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"), feature(rustc_private))]

#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
#[macro_use]
extern crate sgx_tstd as std;
use std::prelude::v1::*;

extern crate libc;
extern crate num_traits;

#[cfg(feature="serde")]
extern crate serde;
#[cfg(all(test, feature="serde"))]
extern crate serde_json;

mod ffi;
pub mod mpz;
pub mod mpq;
pub mod mpf;
pub mod rand;
pub mod sign;

#[cfg(test)]
mod test;
