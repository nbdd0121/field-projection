#![allow(incomplete_features)]
#![feature(field_projections)]

use field_projection::compat::{HasFields, start_proj};

#[derive(HasFields)]
#[fields(with_pinned)]
pub struct Foo {
    a: usize,
}

impl Foo {
    pub fn new() -> Self {
        Self { a: 42 }
    }
}

fn main() {
    let mut foo = Box::pin(Foo::new());
    let mut foo = foo.as_mut();
    start_proj!(mut foo);
    // sadly we can access these
    let _: () = ___projections_raw_ptr_for_foo;
    let _: () = ___projections_checker_for_foo;
}
