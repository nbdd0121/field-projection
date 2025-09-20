use core::ptr::NonNull;

impl<T> Projectable for NonNull<T> {
    type Inner = T;
}

// Additional safety requirements for `project`:
// * The pointer pointed at by `this` must point to an allocated object at least as large as
//   `size_of::<T>()`.
impl<T, F> Project<F> for NonNull<T>
where
    F: Field<Base = T>,
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

unsafe impl<T> compat::ProjectableExt for NonNull<T> {
    type Safety = compat::Unsafe;
}
