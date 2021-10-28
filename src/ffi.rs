use mpz::*;
use std::prelude::v1::*;
#[link(name = "sgx_tgmp")]
extern "C" {
    pub fn __gmpz_fdiv_q(q: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    pub fn __gmpz_cdiv_q(q: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
}
