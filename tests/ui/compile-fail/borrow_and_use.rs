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
    fn foo(self: Pin<&mut Self>) {
        let mut this = start_proj(self);
        let bar = p!(@this->bar);
        let _ = p!(@mut this->bar);
        println!("{bar:?}");
    }
}

fn main() {}
