#![feature(maybe_uninit_fill, maybe_uninit_as_bytes)]
#![allow(dead_code)]

use std::mem::MaybeUninit;

use field_projection::compat::{HasFields, p, start_proj};

#[derive(HasFields)]
struct VeryBig {
    a: usize,
    b: u64,
    buf: [u8; 1024 * 1024 * 1024],
}

impl VeryBig {
    pub fn init(this: &mut MaybeUninit<Self>) {
        let mut this = start_proj(this);
        p!(@mut this->a).write(42);
        p!(@mut this->b).write(0);
        p!(@mut this->buf).as_bytes_mut().write_filled(0xef);
    }
}

fn main() {
    let mut big: Box<MaybeUninit<VeryBig>> = Box::new_uninit();
    VeryBig::init(&mut big);
    let big: Box<VeryBig> = unsafe { big.assume_init() };
    println!("{}", size_of_val(&big));
}
