// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
// kani-flags: -Z loop-contracts -Z valid-value-checks

//! Checks that loops with `loop_modifies` can copy and store under valid-value checks.

#![feature(proc_macro_hygiene)]
#![feature(stmt_expr_attributes)]

#[derive(Clone, Copy, PartialEq)]
struct Flag(bool);

struct Cursor<'a> {
    data: &'a mut [Flag; 2],
    pos: usize,
}

impl Cursor<'_> {
    fn move_to(&mut self, index: usize) {
        unsafe {
            let ptr = self.data.as_mut_ptr();
            std::ptr::copy_nonoverlapping(ptr.add(self.pos), ptr.add(index), 1);
        }
        self.pos = index;
    }
}

#[kani::proof]
fn check_copy_in_loop() {
    let src = [7u8; 2];
    let mut buf = [0u8; 2];
    let mut i: usize = 0;
    #[kani::loop_invariant(i <= 2)]
    #[kani::loop_modifies(&i, &buf)]
    while i < 2 {
        unsafe { std::ptr::copy_nonoverlapping(&src[i], &mut buf[i], 1) };
        assert_eq!(buf[i], 7);
        i += 1;
    }
}

#[kani::proof]
fn check_stores_in_loop() {
    let mut data = [Flag(true); 2];
    let mut cursor = Cursor { data: &mut data, pos: 1 };
    #[kani::loop_invariant(cursor.pos <= 1)]
    #[kani::loop_modifies(&cursor.pos, &*cursor.data)]
    while cursor.pos > 0 {
        let p = cursor.data.as_mut_ptr();
        unsafe { *p.add(cursor.pos) = Flag(true) };
        cursor.move_to(cursor.pos - 1);
        let _f = cursor.data[cursor.pos];
    }
}

#[kani::proof]
fn check_invalid_byte_in_loop() {
    let mut buf = [Flag(false); 2];
    let mut i: usize = 0;
    #[kani::loop_invariant(i <= 2)]
    #[kani::loop_modifies(&i, &buf)]
    while i < 2 {
        unsafe { std::ptr::write_bytes(&mut buf[i] as *mut Flag as *mut u8, 2, 1) };
        let _f = buf[i]; // ~ERROR: 2 is not a valid `bool`.
        i += 1;
    }
}
