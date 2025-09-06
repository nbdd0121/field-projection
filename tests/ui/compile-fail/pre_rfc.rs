#![allow(incomplete_features)]
#![feature(field_projections)]
#![allow(clippy::disallowed_names)]

use std::mem::MaybeUninit;

use field_projection::compat::{HasFields, p, start_proj};

#[derive(HasFields)]
struct Foo {
    field: usize,
    bar: Bar,
}

#[derive(HasFields)]
struct Bar {
    x: u32,
}

// Very simple first example with type annotations.
fn example1(mut foo: &mut MaybeUninit<Foo>) -> usize {
    fn validate(_: &MaybeUninit<usize>) {}

    start_proj!(mut foo);
    let field: &MaybeUninit<usize> = p!(@foo->field);
    // we are allowed to share-project `field` again:
    validate(p!(@foo->field));
    let mut bar: &mut MaybeUninit<Bar> = p!(@mut foo->bar);
    start_proj!(mut bar);
    let x: &mut MaybeUninit<u32> = p!(@mut bar->x);
    x.write(42);
    unsafe { field.assume_init_read() }
}

// Exclusive projections are exclusive, so this doesn't compile.
fn example2(mut foo: &mut MaybeUninit<Foo>) {
    start_proj!(mut foo);
    let a = p!(@mut foo->field);
    //      ------------------- first exclusive projection occurs here
    let b = p!(@mut foo->field);
    //      ^^^^^^^^^^^^^^^^^^^ second exclusive projection occurs here
    MaybeUninit::write(a, 42);
    //                 - first projection later used here
}

// When a value is being projected in any way, it must not be accessed normally.
fn example3(foo: &mut MaybeUninit<Foo>) -> usize {
    start_proj!(foo);
    let field = p!(@foo->field);
    //          --------------- shared projection occurs here
    let value = Foo {
        field: 42,
        bar: Bar { x: 72 },
    };
    MaybeUninit::write(foo, value);
    //^^^^^^^^^^^^^^^^^^^^^^^^^^^^ projected value used here
    unsafe { field.assume_init_read() }
    //       ----- shared projection used here
}

fn example3_allowed_in_exp_impl(foo: &mut MaybeUninit<Foo>) -> usize {
    start_proj!(foo);
    let field = p!(@foo->field);
    let _ = unsafe { foo.assume_init_read() };
    unsafe { field.assume_init_read() }
}

fn move_projections<'a>(foo: &'a mut MaybeUninit<Foo>) -> &'a mut MaybeUninit<Bar> {
    start_proj!(move foo);
    p!(@move mut foo->bar)
}

fn move_projections_motivation<'a>(mut foo: &'a mut MaybeUninit<Foo>) -> &'a mut MaybeUninit<Bar> {
    start_proj!(mut foo);
    //------------------ `foo` is borrowed here
    p!(@mut foo->bar)
    // ^^^^^^^^^^^^^ returns a value referencing data owned by the current function
}

fn main() {}
