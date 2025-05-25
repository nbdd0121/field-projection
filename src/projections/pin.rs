use core::pin::Pin;

impl<T> Projectable for Pin<&mut T> {
    type Inner = T;
}

// SAFETY: no additional safety requirements on `Project[Mut]::project[_mut]`.
unsafe impl<T> SafeProject for Pin<&mut T> {}

// No additional safety requirements for `project_mut`.
impl<'a, T, F> Project<F> for Pin<&'a mut T>
where
    F: PinableField<Base = T> + Field<Base = T>,
    F::Type: Sized + 'a,
{
    type Output<'b>
        = &'b F::Type
    where
        Self: 'b;

    unsafe fn project<'b>(this: *const Self) -> Self::Output<'b>
    where
        Self: 'b,
    {
        let ptr = unsafe { Pin::into_inner_unchecked(this.read()) };
        let ptr: *const T = ptr;
        unsafe { &*ptr.byte_add(F::OFFSET).cast() }
    }
}

// No additional safety requirements for `project_mut`.
impl<'a, T, F> ProjectMut<F> for Pin<&'a mut T>
where
    F: PinableField<Base = T> + Field<Base = T>,
    F::Type: Sized + 'a,
{
    type OutputMut<'b>
        = F::Projected<'b>
    where
        Self: 'b;

    unsafe fn project_mut<'b>(this: *mut Self) -> Self::OutputMut<'b>
    where
        Self: 'b,
    {
        let r = unsafe { Pin::into_inner_unchecked(this.read()) };
        let ptr: *mut T = r;
        let ptr = unsafe { ptr.byte_add(F::OFFSET).cast() };
        unsafe { F::from_pinned_ref(&mut *ptr) }
    }
}

// Compat

unsafe impl<T> compat::ProjectableExt for Pin<&mut T> {
    type Safety = compat::Safe;
}
