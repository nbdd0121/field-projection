use field_projection::compat::{HasFields, p, start_proj};

mod foo {
    use super::*;

    #[derive(HasFields)]
    #[fields(with_pinned)]
    pub struct Foo {
        a: usize,
        pub b: usize,
    }

    impl Foo {
        pub fn new() -> Self {
            Self { a: 42, b: 42 }
        }
    }
}

use foo::Foo;

fn main() {
    let mut foo = Box::pin(Foo::new());
    let mut foo = start_proj(foo.as_mut());
    let _ = p!(@mut foo->a);
}
