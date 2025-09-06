#![allow(incomplete_features)]
#![feature(field_projections)]

use std::{fmt::Debug, pin::Pin};

use field_projection::compat::{HasFields, p, start_proj};

#[derive(HasFields, Debug)]
#[fields(with_pinned)]
struct Foo {
    #[pin]
    bar: Bar,
}

#[derive(Debug)]
struct Bar;

impl Bar {
    fn bar(self: Pin<&mut Self>, _: Pin<&mut Self>) {}
}

impl Foo {
    fn foo(mut self: Pin<&mut Self>) {
        start_proj!(mut self);
        let bar = p!(@self->bar);
        let _ = p!(@mut self->bar);
        println!("{bar:?}");
    }
}

fn main() {}
