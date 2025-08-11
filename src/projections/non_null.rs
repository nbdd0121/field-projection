use core::ptr::NonNull;

impl<T: ?Sized> Projectable for NonNull<T> {
    type Inner = T;
}

// Additional safety requirements for `project`:
// * The pointer pointed at by `this` must point to an allocated object at least as large as
//   `size_of::<T>()`.
impl<T: ?Sized, F> Project<F> for NonNull<T>
where
    F: UnalignedField<Base = T>,
    F::Type: Sized,
{
    type Output<'a>
        = NonNull<F::Type>
    where
        Self: 'a;
    unsafe fn project<'a>(this: *const Self) -> Self::Output<'a>
    where
        Self: 'a,
    {
        let ptr = unsafe { this.read() };
        unsafe { ptr.byte_add(F::OFFSET).cast() }
    }
}

// Compat

unsafe impl<T: ?Sized> compat::ProjectableExt for NonNull<T> {
    type Safety = compat::Unsafe;
}
