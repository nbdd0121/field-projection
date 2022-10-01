use field_projection::*;

use core::mem::MaybeUninit;
use core::pin::Pin;

#[derive(Field, PinField)]
struct Foo {
    #[pin]
    a: usize,
    b: usize,
}

#[derive(Field, PinField)]
struct Bar {
    #[pin]
    foo: Foo,
    c: usize,
}

#[test]
fn maybe_uninit_projection() {
    let mut x: MaybeUninit<Bar> = MaybeUninit::uninit();
    project!(&mut x => c).write(1);
    let foo = project!(&mut x => foo);
    project!(&mut *foo => a).write(1);
    project!(foo => b).write(1);
}

fn pin_projection(mut x: Pin<&mut Bar>) {
    let foo: Pin<&mut Foo> = project!(x.as_mut() => foo);
    *project!(foo => a) = 1;
    let _c: &mut usize = project!(x.as_mut() => c);
}

#[test]
fn test_pin_project() {
    let mut pin = Box::pin(Bar {
        foo: Foo { a: 0, b: 0 },
        c: 0,
    });
    pin_projection(pin.as_mut());
}
