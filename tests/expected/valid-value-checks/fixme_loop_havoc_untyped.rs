// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
// kani-flags: -Z loop-contracts -Z valid-value-checks

//! FIXME: a loop target copied into untyped and then havocked is reported as an invalid value.
//! See https://github.com/model-checking/kani/issues/[PLACEHOLDER: fix before putting up PR] for details.

#![feature(proc_macro_hygiene)]
#![feature(stmt_expr_attributes)]

#[derive(Clone, Copy, PartialEq)]
struct Flag(bool);

#[kani::proof]
fn check_copy_in_loop() {
    let src = [Flag(true); 2];
    let mut buf = [Flag(false); 2];
    let mut i: usize = 0;
    #[kani::loop_invariant(i <= 2)]
    #[kani::loop_modifies(&i, &buf)]
    while i < 2 {
        unsafe { std::ptr::copy_nonoverlapping(&src[i], &mut buf[i], 1) };
        assert!(buf[i].0);
        i += 1;
    }
}
