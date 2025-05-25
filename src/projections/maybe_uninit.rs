use core::mem::MaybeUninit;

impl<T> Projectable for &mut MaybeUninit<T> {
    type Inner = T;
}

// SAFETY: no additional safety requirements on `Project[Mut]::project[_mut]`.
unsafe impl<T> SafeProject for &mut MaybeUninit<T> {}

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

unsafe impl<T> compat::ProjectableExt for &mut MaybeUninit<T> {
    type Safety = compat::Safe;
}
