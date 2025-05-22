use field_projection::compat::{HasFields, p, start_proj};

use core::mem::MaybeUninit;
use core::pin::Pin;

#[derive(HasFields)]
#[fields(with_pinned)]
struct Foo {
    #[pin]
    a: usize,
    b: usize,
}

#[derive(HasFields)]
#[fields(with_pinned)]
struct Bar {
    #[pin]
    foo: Foo,
    c: usize,
}

#[test]
fn maybe_uninit_projection() {
    let mut x: MaybeUninit<Bar> = MaybeUninit::uninit();
    let mut x = &mut x;
    start_proj!(mut x);
    p!(@mut x->c).write(1);
    let mut foo = p!(@mut x->foo);
    start_proj!(mut foo);
    p!(@mut foo->a).write(1);
    p!(@mut foo->b).write(1);
}

#[allow(clippy::disallowed_names)]
fn pin_projection(mut x: Pin<&mut Bar>) {
    start_proj!(mut x);
    let mut foo: Pin<&mut Foo> = p!(@mut x->foo);
    start_proj!(mut foo);
    *p!(@mut foo->a) = 1;
    let _c: &mut usize = p!(@mut x->c);
}

#[test]
fn test_pin_project() {
    let mut pin = Box::pin(Bar {
        foo: Foo { a: 0, b: 0 },
        c: 0,
    });
    pin_projection(pin.as_mut());
}
