#![allow(incomplete_features)]
#![feature(field_projections)]
#![feature(allocator_api)]

use std::{
    alloc::{Allocator, Global, Layout},
    field::Field,
    mem,
    ops::Deref,
    ptr::{NonNull, drop_in_place},
    sync::atomic::{AtomicUsize, Ordering},
};

use field_projection::{
    compat,
    ops::{Project, Projectable, SafeProject},
};
use field_projection_internal::{HasFields, p, start_proj};

pub struct ArcRef<T: ?Sized> {
    info: NonNull<Info>,
    value: NonNull<T>,
}

#[repr(C)]
struct Info {
    refcount: AtomicUsize,
    layout: Layout,
    drop_value: fn(*mut Info),
}

#[repr(C)]
struct Inner<T: ?Sized> {
    info: Info,
    value: T,
}

impl<T: ?Sized> Drop for ArcRef<T> {
    fn drop(&mut self) {
        if self.refcount().fetch_sub(1, Ordering::SeqCst) == 1 {
            (self.info().drop_value)(self.info.as_ptr());
            let layout = self.info().layout;
            let ptr = self.info.cast::<u8>();
            unsafe { Global.deallocate(ptr, layout) }
        }
    }
}

impl<T: ?Sized> ArcRef<T> {
    pub fn new(val: T) -> Self
    where
        T: Sized,
    {
        let layout = Layout::new::<Inner<T>>();
        let ptr = Global.allocate(layout).unwrap().cast::<Inner<T>>();
        let info: NonNull<Info> = ptr.cast();
        let value = unsafe { NonNull::new_unchecked(&raw mut (*ptr.as_ptr()).value) };

        let info_value = Info {
            refcount: AtomicUsize::new(1),
            layout,
            drop_value: |ptr| unsafe { drop_in_place(ptr.cast::<Inner<T>>()) },
        };
        unsafe { info.write(info_value) };
        unsafe { value.write(val) };

        Self { info, value }
    }

    fn info(&self) -> &Info {
        unsafe { &(*self.info.as_ptr()) }
    }

    fn refcount(&self) -> &AtomicUsize {
        &self.info().refcount
    }
}

impl<T: ?Sized> Clone for ArcRef<T> {
    fn clone(&self) -> Self {
        self.refcount().fetch_add(1, Ordering::Relaxed);
        Self {
            info: self.info,
            value: self.value,
        }
    }
}

impl<T: ?Sized> Deref for ArcRef<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.value.as_ref() }
    }
}

impl<T: ?Sized> Projectable for ArcRef<T> {
    type Inner = T;
}

impl<T, F> Project<F> for ArcRef<T>
where
    T: ?Sized,
    F: Field<Base = T>,
    F::Type: Sized,
{
    type Output<'a>
        = ArcRef<F::Type>
    where
        Self: 'a;

    unsafe fn project<'a>(this: *const Self) -> Self::Output<'a>
    where
        Self: 'a,
    {
        let this = unsafe { &*this };
        mem::forget(this.clone());
        ArcRef {
            info: this.info,
            value: unsafe { Project::<F>::project(&this.value) },
        }
    }
}

unsafe impl<T: ?Sized> SafeProject for ArcRef<T> {}

unsafe impl<T: ?Sized> compat::ProjectableExt for ArcRef<T> {
    type Safety = compat::Safe;
}

fn main() {
    #[derive(HasFields)]
    struct Foo {
        x: usize,
        y: usize,
    }

    let foo = ArcRef::new(Foo { x: 42, y: 24 });
    start_proj!(foo);
    let x = p!(@foo->x);
    let y = p!(@foo->y);
    assert_ne!(&*x, &*y);
    assert_eq!(foo.refcount().load(Ordering::Relaxed), 3);
}
