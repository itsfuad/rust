// Check that partially initialized SIMD constants keep all of their bytes.

//@ run-pass

#![feature(repr_simd)]

use std::hint::black_box;
use std::mem::MaybeUninit;

#[repr(simd)]
#[derive(Copy, Clone)]
struct U64x2([u64; 2]);

static X: u8 = 7;

const HALF_LANE: MaybeUninit<U64x2> = unsafe {
    let mut m = MaybeUninit::<U64x2>::uninit();
    (m.as_mut_ptr() as *mut u32).write(0xdeadbeef);
    m
};

const POINTER_LANE: MaybeUninit<U64x2> = unsafe {
    let mut m = MaybeUninit::<U64x2>::uninit();
    (m.as_mut_ptr() as *mut *const u8).write(&X);
    m
};

fn main() {
    let half = HALF_LANE;
    let pointer = POINTER_LANE;
    unsafe {
        assert_eq!((black_box(&raw const half) as *const u32).read(), 0xdeadbeef);
        assert_eq!((black_box(&raw const pointer) as *const *const u8).read(), &raw const X);
    }
}
