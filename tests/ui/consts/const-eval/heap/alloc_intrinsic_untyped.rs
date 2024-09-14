// We unleash Miri here since this test demonstrates code that bypasses the checks against interning
// mutable pointers, which currently ICEs. Unleashing Miri silence the ICE.
//@ compile-flags: -Zunleash-the-miri-inside-of-you
#![feature(core_intrinsics)]
#![feature(const_heap)]
#![feature(const_mut_refs)]
use std::intrinsics;

const BAR: *mut i32 = unsafe { intrinsics::const_allocate(4, 4) as *mut i32 };
//~^ error: mutable pointer in final value of constant

fn main() {}
