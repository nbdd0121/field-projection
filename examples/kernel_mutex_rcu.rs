#![feature(unsafe_pinned)]
#![allow(dead_code)]

use field_projection::compat::{HasFields, p, start_proj};
use std::pin::Pin;

mod rcu {
    use std::{
        cell::UnsafeCell,
        ops::{Deref, DerefMut},
        pin::{Pin, UnsafePinned},
    };

    use field_projection::{
        compat::{ProjectableExt, Safe},
        marker::UnalignedField,
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
}
use rcu::*;

struct BufferConfig {
    flush_sensitivity: u8,
}

#[derive(HasFields)]
#[fields(with_pinned)]
struct Buffer {
    // We also require `Rcu` to be pinned, because `&mut Rcu` must not exist (otherwise one could
    // call mem::swap).
    #[pin]
    cfg: Rcu<Box<BufferConfig>>,
    buf: Vec<u8>,
}

#[derive(HasFields)]
#[fields(with_pinned)]
struct MyDriver {
    // The `Mutex` in the kernel needs to be pinned.
    #[pin]
    buf: RcuMutex<Buffer>,
}

impl MyDriver {
    fn flush_sensitivity<'a>(&'a self, rcu_guard: &'a RcuGuard) -> u8 {
        let buf = start_proj(&self.buf);
        // Here we use the special projections set up for `Mutex` with fields of type `Rcu<T>`.
        let cfg: &Rcu<Box<BufferConfig>> = p!(@buf->cfg);
        cfg.read(rcu_guard).flush_sensitivity
    }

    /*
     * the following implementation of `buffer_config` *should* also compile, but doesn't due to
     * the macro having to create a value on the stack...

    fn buffer_config<'a>(&'a self, rcu_guard: &'a RcuGuard) -> &'a BufferConfig {
        let buf = start_proj(&self.buf);
        // Here we use the special projections set up for `Mutex` with fields of type `Rcu<T>`.
        let cfg: &Rcu<Box<BufferConfig>> = p!(@buf->cfg);
        //                                    --------- `buf.cfg` is borrowed here
        cfg.read(rcu_guard)
        //^^^^^^^^^^^^^^^^^ returns a value referencing data owned by the current function
    }

    */

    fn set_buffer_config(&self, flush_sensitivity: u8) {
        // Our `Mutex` pins the value.
        let mut guard: Pin<RcuMutexGuard<'_, Buffer>> = self.buf.lock();
        let mut buf = start_proj(guard.as_mut());
        // We can use pin-projections since we marked `cfg` as `#[pin]`
        let cfg: Pin<&mut Rcu<Box<BufferConfig>>> = p!(@mut buf->cfg);
        cfg.set(Box::new(BufferConfig { flush_sensitivity }));
        // ^^ this returns an `Old<Box<BufferConfig>>` and runs `synchronize_rcu` on drop.
    }
}

fn main() {}
