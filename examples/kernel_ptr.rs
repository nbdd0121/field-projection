use core::ptr::NonNull;
use std::marker::PhantomData;

use field_projection::{
    compat::{HasFields, ProjectableExt, Safe, p, start_proj},
    marker::Field,
    ops::{Project, Projectable, SafeProject},
};

/// # Invariants
///
/// * `inner` is dereferenceable and points at an allocation of at least size `size_of::<T>`.
#[derive(HasFields)]
pub struct Ptr<'a, T: 'a> {
    inner: NonNull<T>,
    _phantom: PhantomData<(&'a T, &'a mut T)>,
}

impl<'a, T: 'a> Projectable for Ptr<'a, T> {
    type Inner = T;
}

unsafe impl<'a, T: 'a> SafeProject for Ptr<'a, T> {}

unsafe impl<'a, T: 'a> ProjectableExt for Ptr<'a, T> {
    type Safety = Safe;
}

unsafe impl<'a, T, F> Project<F> for Ptr<'a, T>
where
    T: 'a,
    F: Field<Base = T>,
    F::Type: 'a + Sized,
{
    type Output<'b>
        = Ptr<'a, F::Type>
    where
        Self: 'b;

    unsafe fn project<'b>(this: *const Self) -> Self::Output<'b>
    where
        Self: 'b,
    {
        start_proj!(this);
        let ptr = unsafe { *p!(@this->inner) };
        Ptr {
            inner: unsafe { ptr.byte_add(F::OFFSET).cast() },
            _phantom: PhantomData,
        }
    }
}

fn main() {}
