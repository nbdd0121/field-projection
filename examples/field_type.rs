use field_projection::{
    compat::HasFields,
    field_of,
    marker::{Field, UnalignedField},
};

use core::ptr::null_mut;
use std::sync::Arc;

/// An intrusive struct to link all queued work together.
pub struct Work {
    state: usize,
    func: Option<fn(*mut Work)>,
    next: *mut Work,
}

pub trait WorkItem<F: Field> {
    fn run(&self);
}

impl WorkItem<field_of!(Foo, work)> for Foo {
    fn run(&self) {
        println!("hello work");
    }
}

impl WorkItem<field_of!(Foo, work2)> for Foo {
    fn run(&self) {
        println!("hello work2");
    }
}

fn bridge<F: Field<Type = Work>>(ptr: *mut Work)
where
    F::Base: WorkItem<F> + Sized,
{
    let ptr = unsafe { ptr.byte_sub(F::OFFSET) }.cast::<F::Base>();

    unsafe { &*ptr }.run();
}

impl Work {
    /// TODO: this should return PinInit.
    ///
    /// # Safety
    ///
    /// Callers must guarantee the `Work` is the exact field described by `F`.
    pub const unsafe fn new<F: Field<Type = Work>>() -> Self
    where
        F::Base: WorkItem<F> + Sized,
    {
        Self {
            state: 0,
            func: Some(bridge::<F>),
            next: null_mut(),
        }
    }
}

#[derive(HasFields)]
pub struct Foo {
    a: i32,
    b: i32,
    work: Work,
    work2: Work,
}

impl Foo {
    pub fn new_arc(a: i32, b: i32) -> Arc<Self> {
        Arc::new(Self {
            a,
            b,
            // SAFETY: `work` is Foo::work.
            work: unsafe { Work::new::<field_of!(Foo, work)>() },
            // SAFETY: `work2` is Foo::work2.
            work2: unsafe { Work::new::<field_of!(Foo, work2)>() },
        })
    }
}

pub trait WorkItemPointer<F: Field<Type = Work>>: Sized {
    // Simulates RawWorkItem
    fn into_work(self) -> *mut Work;

    fn c_run(ptr: *mut Work);
}

impl<T, F: Field<Base = T, Type = Work>> WorkItemPointer<F> for Arc<T>
where
    T: WorkItem<F>,
{
    fn into_work(self) -> *mut Work {
        let ptr = Arc::into_raw(self);
        unsafe { ptr.byte_add(F::OFFSET).cast_mut().cast() }
    }

    fn c_run(ptr: *mut Work) {
        let ptr = unsafe { ptr.byte_sub(F::OFFSET).cast::<T>() };

        let arc = unsafe { Arc::from_raw(ptr) };

        arc.run();
    }
}

/// # Safety
///
/// `work` must point to a valid pointer of `Work`.
unsafe fn c_run_work(work: *mut Work) {
    if let Some(func) = unsafe { (*work).func } {
        func(work)
    }
}

pub fn queue_work<F: Field<Base = T, Type = Work>, T: WorkItem<F>, P: WorkItemPointer<F>>(p: P) {
    // Simulate queue work and run
    //
    // TODO: simulate linking work and execute them all together.
    let ptr = p.into_work();

    // SAFETY: `ptr` is a valid pointer of `Work`.
    unsafe { c_run_work(ptr) };
}

#[test]
fn main() {
    let foo = Foo::new_arc(12, 42);
    let foo2 = Foo::new_arc(12, 42);

    queue_work::<field_of!(Foo, work), _, _>(foo);
    queue_work::<field_of!(Foo, work2), _, _>(foo2);
}
