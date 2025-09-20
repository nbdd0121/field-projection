use core::cell::{Ref, RefMut};

use crate::helper::project_ref;

impl<T> Projectable for Ref<'_, T> {
    type Inner = T;
}

impl<T> Projectable for RefMut<'_, T> {
    type Inner = T;
}

unsafe impl<T> SafeProject for Ref<'_, T> {}
unsafe impl<T> SafeProject for RefMut<'_, T> {}

impl<'a, T, F> Project<F> for Ref<'a, T>
where
    F: Field<Base = T>,
    F::Type: 'a,
{
    type Output<'b>
        = Ref<'b, F::Type>
    where
        Self: 'b;
    unsafe fn project<'b>(this: *const Self) -> Self::Output<'b>
    where
        Self: 'b,
    {
        let this = unsafe { &*this };
        Ref::map(Ref::clone(this), |r| project_ref::<F>(r))
    }
}

impl<'a, T, F> ProjectMut<F> for RefMut<'a, T>
where
    F: Field<Base = T>,
    F::Type: 'a,
{
    type OutputMut<'b>
        = RefMut<'b, F::Type>
    where
        Self: 'b;
    unsafe fn project_mut<'b>(_this: *mut Self) -> Self::OutputMut<'b>
    where
        Self: 'b,
    {
        // this can't be implemented without being able to increment the mutable borrow count
        // but it would look like the project for `Ref`
        todo!("cannot be implemented outside of the stdlib")
    }
}

unsafe impl<T> compat::ProjectableExt for Ref<'_, T> {
    type Safety = compat::Safe;
}

unsafe impl<T> compat::ProjectableExt for RefMut<'_, T> {
    type Safety = compat::Safe;
}
