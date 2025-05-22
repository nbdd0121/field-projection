use core::cell::UnsafeCell;

impl<T> Projectable for &UnsafeCell<T> {
    type Inner = T;
}

impl<'a, T, F> Project<F> for &'a UnsafeCell<T>
where
    F: Field<Base = T>,
    F::Type: 'a + Sized,
{
    type Output<'b>
        = &'b UnsafeCell<F::Type>
    where
        Self: 'b;
    unsafe fn project<'b>(this: *const Self) -> Self::Output<'b>
    where
        Self: 'b,
    {
        let ptr: *const UnsafeCell<T> = unsafe { this.read() };
        unsafe { &*ptr.byte_add(F::OFFSET).cast() }
    }
}
