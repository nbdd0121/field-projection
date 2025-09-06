use std::{
    cell::UnsafeCell,
    field::UnalignedField,
    ops::{Deref, DerefMut},
    pin::{Pin, UnsafePinned},
};

use field_projection::{
    compat::{ProjectableExt, Safe},
    ops::{Project, Projectable, SafeProject},
};

pub struct RcuGuard(());

impl Drop for RcuGuard {
    fn drop(&mut self) {
        /* bindings::rcu_read_unlock() */
    }
}

pub fn read_lock() -> RcuGuard {
    /* bindings::rcu_read_lock() */
    RcuGuard(())
}

pub struct Rcu<P> {
    inner: UnsafePinned<UnsafeCell<P>>,
}

impl<P: Deref> Rcu<P> {
    pub fn read<'a>(&'a self, _guard: &'a RcuGuard) -> &'a P::Target {
        // FIX: this should actually use an atomic ptr read
        unsafe { &*UnsafeCell::raw_get(self.inner.get()) }
    }

    // FIX: the return type should actually be `impl PinInit<Old<P>>`, since the value *must*
    // be dropped and is not allowed to be forgotten, so we use the pin guarantee.
    pub fn set(self: Pin<&mut Self>, new: P) -> Old<P> {
        let ptr = UnsafeCell::raw_get(self.inner.get());
        // FIX: need to use atomic write for this operation, so need some additional trait on P.
        let old = unsafe { ptr.read() };
        unsafe { ptr.write(new) };
        Old(old)
    }
}

pub struct Old<P>(P);

impl<P> Drop for Old<P> {
    fn drop(&mut self) {
        /* bindings::synchronize_rcu() */
    }
}

pub struct RcuMutex<T> {
    data: UnsafeCell<T>,
}

pub struct RcuMutexGuard<'a, T> {
    mtx: &'a RcuMutex<T>,
}

impl<T> Deref for RcuMutexGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mtx.data.get() }
    }
}

impl<T> DerefMut for RcuMutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mtx.data.get() }
    }
}

impl<T> RcuMutex<T> {
    pub fn lock(&self) -> Pin<RcuMutexGuard<'_, T>> {
        let res = RcuMutexGuard { mtx: self };
        unsafe { Pin::new_unchecked(res) }
    }
}

impl<T> Projectable for &RcuMutex<T> {
    type Inner = T;
}

unsafe impl<T> SafeProject for &RcuMutex<T> {}

unsafe impl<T> ProjectableExt for &RcuMutex<T> {
    type Safety = Safe;
}

impl<'a, T, U, F> Project<F> for &'a RcuMutex<T>
where
    F: UnalignedField<Base = T, Type = Rcu<U>>,
    U: 'a,
{
    type Output<'b>
        = &'b F::Type
    where
        Self: 'b;

    unsafe fn project<'b>(this: *const Self) -> Self::Output<'b>
    where
        Self: 'b,
    {
        let ptr: *const RcuMutex<T> = unsafe { this.read() };
        let ptr = UnsafeCell::raw_get(unsafe { &raw const (*ptr).data });
        unsafe { &*ptr.byte_add(F::OFFSET).cast() }
    }
}
