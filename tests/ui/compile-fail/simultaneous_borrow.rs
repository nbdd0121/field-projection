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
    fn foo(self: Pin<&mut Self>) {
        let mut this = start_proj(self);
        p!(@mut this->bar).bar(p!(@mut this->bar));
    }
}

fn main() {}
