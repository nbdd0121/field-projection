#![feature(unsafe_pinned)]
#![allow(dead_code)]

use field_projection::compat::{HasFields, p, start_proj};
use std::pin::Pin;

mod rcu;
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
        let buf: &'a RcuMutex<Buffer> = &self.buf;
        start_proj!(buf);
        // Here we use the special projections set up for `Mutex` with fields of type `Rcu<T>`.
        let cfg: &Rcu<Box<BufferConfig>> = p!(@buf->cfg);
        cfg.read(rcu_guard).flush_sensitivity
    }

    fn buffer_config<'a>(&'a self, rcu_guard: &'a RcuGuard) -> &'a BufferConfig {
        let buf: &'a RcuMutex<Buffer> = &self.buf;
        start_proj!(move buf);
        let cfg: &Rcu<Box<BufferConfig>> = p!(@move buf->cfg);
        cfg.read(rcu_guard)
    }

    fn set_buffer_config(&self, flush_sensitivity: u8) {
        // `RcuMutex` pins the value.
        let mut guard: Pin<RcuMutexGuard<'_, Buffer>> = self.buf.lock();
        let mut buf: Pin<&mut Buffer> = guard.as_mut();
        start_proj!(mut buf);
        // We can use pin-projections since we marked `cfg` as `#[pin]`
        let cfg: Pin<&mut Rcu<Box<BufferConfig>>> = p!(@mut buf->cfg);
        cfg.set(Box::new(BufferConfig { flush_sensitivity }));
        //  ^^^ this returns an `Old<Box<BufferConfig>>` and runs `synchronize_rcu` on drop.
    }
}

fn main() {}
