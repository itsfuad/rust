// Regression test for <https://github.com/rust-lang/rust/issues/166089>.

//@ compile-flags: -C opt-level=2 -C no-prepopulate-passes

#![feature(repr_simd)]
#![crate_type = "lib"]
#![allow(invalid_value)]

use std::mem::MaybeUninit;

#[repr(simd)]
#[derive(Copy, Clone)]
pub struct U64x4([u64; 4]);

#[inline(never)]
#[no_mangle]
pub fn take(_: U64x4) {}

// CHECK-LABEL: @uninit
#[no_mangle]
pub fn uninit() {
    // CHECK-NOT: load
    // CHECK: store <4 x i64> poison
    let v: U64x4 = unsafe { MaybeUninit::uninit().assume_init() };
    take(v);
}

// CHECK-LABEL: @init
#[no_mangle]
pub fn init() {
    // CHECK-NOT: load
    // CHECK: store <4 x i64> <i64 1, i64 2, i64 3, i64 -1>
    take(const { U64x4([1, 2, 3, u64::MAX]) });
}
