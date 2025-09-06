#![feature(arbitrary_self_types)]
#![allow(incomplete_features)]
#![feature(field_projections)]

use field_projection::compat::{HasFields, p, start_proj};

mod cpp_ref;
use cpp_ref::CppMutRef;

// Shared between C++ and Rust.
#[derive(HasFields)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn set_x(mut self: CppMutRef<Self>, value: i32) {
        start_proj!(mut self);
        p!(@mut self->x).write(value);
    }

    pub fn set_y(mut self: CppMutRef<Self>, value: i32) {
        start_proj!(mut self);
        p!(@mut self->y).write(value);
    }
}

fn main() {}
