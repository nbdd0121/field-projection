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

impl<'a, T, F> Project<F> for Ptr<'a, T>
where
    T: 'a,
    F: Field<Base = T>,
    F::Type: 'a + Sized,
{
    type Output<'b>
        = Ptr<'b, F::Type>
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

impl<'a, T: 'a> Ptr<'a, T> {
    /// Turn this pointer-to-a-field into a pointer to the entire container.
    ///
    /// # Safety
    ///
    /// * `self` points at a `T` that is contained as the field `F` inside of a `U`.
    /// * `self` is derived from a pointer that originally pointed at the entire `U` that contains
    ///   this `T` as the field `F`.
    pub unsafe fn container_of<U, F>(self) -> Ptr<'a, U>
    where
        U: 'a,
        F: Field<Base = U, Type = T>,
    {
        let inner = unsafe { self.inner.byte_sub(F::OFFSET) };
        Ptr {
            inner: inner.cast(),
            _phantom: PhantomData,
        }
    }
}

fn main() {}
