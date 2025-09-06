#![allow(incomplete_features)]
#![feature(field_projections)]

use std::pin::Pin;

use field_projection::compat::{HasFields, p, start_proj};

#[derive(HasFields)]
#[fields(with_pinned)]
struct Foo {
    #[pin]
    bar: Bar,
}

struct Bar;

impl Bar {
    fn bar(self: Pin<&mut Self>, _: Pin<&mut Self>) {}
}

impl Foo {
    fn foo(mut self: Pin<&mut Self>) {
        start_proj!(mut self);
        p!(@mut self->bar).bar(p!(@mut self->bar));
    }
}

fn main() {}
