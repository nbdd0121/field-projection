use field_projection::compat::{HasFields, p, start_proj};

#[derive(HasFields)]
struct Foo {
    x: usize,
}

fn main() {
    let foo = Foo { x: 42 };
    let foo: *const Foo = &foo;
    let foo = start_proj(foo);
    p!(@foo->x);
}
