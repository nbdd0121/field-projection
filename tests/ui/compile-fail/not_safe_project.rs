#![allow(incomplete_features)]
#![feature(field_projections)]

use field_projection::compat::{HasFields, p, start_proj};

#[derive(HasFields)]
struct Foo {
    x: usize,
}

fn main() {
    let foo = Foo { x: 42 };
    let foo: *const Foo = &foo;
    start_proj!(foo);
    p!(@foo->x);
}
