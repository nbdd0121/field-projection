use std::{ops::Deref, ptr::NonNull, sync::atomic::AtomicUsize};

use field_projection::compat::{HasFields, p, start_proj};

#[derive(HasFields)]
struct ArcInner<T> {
    refcount: AtomicUsize,
    value: T,
}

pub struct Arc<T> {
    inner: NonNull<ArcInner<T>>,
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let inner = start_proj(self.inner);
        let value = unsafe { p!(@inner->value) };
        unsafe { value.as_ref() }
    }
}

impl<T> Arc<T> {
    pub fn new(value: T) -> Self {
        let inner = Box::into_raw(Box::new(ArcInner {
            refcount: AtomicUsize::new(1),
            value,
        }));
        let inner = unsafe { NonNull::new_unchecked(inner) };
        Self { inner }
    }
}

fn main() {
    let arc: Arc<i32> = Arc::new(42);
    let val: &i32 = arc.deref();
    println!("{val}");
}
