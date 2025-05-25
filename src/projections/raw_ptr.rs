impl<T> Projectable for *const T {
    type Inner = T;
}

// Additional safety requirements for `project`:
// * The pointer pointed at by `this` must point to an allocated object at least as large as
//   `size_of::<T>()`.
impl<T, F> Project<F> for *const T
where
    F: UnalignedField<Base = T>,
    F::Type: Sized,
{
    type Output<'b>
        = *const F::Type
    where
        Self: 'b;

    unsafe fn project<'b>(this: *const Self) -> Self::Output<'b>
    where
        Self: 'b,
    {
        let ptr = unsafe { this.read() };
        unsafe { ptr.byte_add(F::OFFSET).cast() }
    }
}

impl<T> Projectable for *mut T {
    type Inner = T;
}

// Additional safety requirements for `project`:
// * The pointer pointed at by `this` must point to an allocated object at least as large as
//   `size_of::<T>()`.
impl<T, F> Project<F> for *mut T
where
    F: UnalignedField<Base = T>,
    F::Type: Sized,
{
    type Output<'b>
        = *mut F::Type
    where
        Self: 'b;

    unsafe fn project<'b>(this: *const Self) -> Self::Output<'b>
    where
        Self: 'b,
    {
        let ptr = unsafe { this.read() };
        unsafe { ptr.byte_add(F::OFFSET).cast() }
    }
}

// Compat

unsafe impl<T> compat::ProjectableExt for *const T {
    type Safety = compat::Unsafe;
}

unsafe impl<T> compat::ProjectableExt for *mut T {
    type Safety = compat::Unsafe;
}
