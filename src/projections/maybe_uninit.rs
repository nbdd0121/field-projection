use core::mem::MaybeUninit;

impl<T> Projectable for &MaybeUninit<T> {
    type Inner = T;
}

impl<T> Projectable for &mut MaybeUninit<T> {
    type Inner = T;
}

// SAFETY: no additional safety requirements on `Project[Mut]::project[_mut]`.
unsafe impl<T> SafeProject for &MaybeUninit<T> {}

// SAFETY: no additional safety requirements on `Project[Mut]::project[_mut]`.
unsafe impl<T> SafeProject for &mut MaybeUninit<T> {}

// No additional safety requirements for `project`.
impl<'a, T, F> Project<F> for &'a MaybeUninit<T>
where
    F: Field<Base = T>,
    F::Type: Sized + 'a,
{
    type Output<'b>
        = &'b MaybeUninit<F::Type>
    where
        Self: 'b;

    unsafe fn project<'b>(this: *const Self) -> Self::Output<'a>
    where
        Self: 'b,
    {
        let ptr: *const MaybeUninit<T> = unsafe { this.read() };
        unsafe { &*ptr.byte_add(F::OFFSET).cast() }
    }
}

// No additional safety requirements for `project`.
impl<'a, T, F> Project<F> for &'a mut MaybeUninit<T>
where
    F: Field<Base = T>,
    F::Type: Sized + 'a,
{
    type Output<'b>
        = &'b MaybeUninit<F::Type>
    where
        Self: 'b;

    unsafe fn project<'b>(this: *const Self) -> Self::Output<'a>
    where
        Self: 'b,
    {
        let ptr: *const MaybeUninit<T> = unsafe { this.read() };
        unsafe { &*ptr.byte_add(F::OFFSET).cast() }
    }
}

// No additional safety requirements for `project_mut`.
impl<'a, T, F> ProjectMut<F> for &'a mut MaybeUninit<T>
where
    F: Field<Base = T>,
    F::Type: Sized + 'a,
{
    type OutputMut<'b>
        = &'b mut MaybeUninit<F::Type>
    where
        Self: 'b;

    unsafe fn project_mut<'b>(this: *mut Self) -> Self::OutputMut<'a>
    where
        Self: 'b,
    {
        let ptr: *mut MaybeUninit<T> = unsafe { this.read() };
        unsafe { &mut *ptr.byte_add(F::OFFSET).cast() }
    }
}

// Compat

unsafe impl<T> compat::ProjectableExt for &MaybeUninit<T> {
    type Safety = compat::Safe;
}

unsafe impl<T> compat::ProjectableExt for &mut MaybeUninit<T> {
    type Safety = compat::Safe;
}
