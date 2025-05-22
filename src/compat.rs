#![allow(clippy::missing_safety_doc)]

use core::marker::PhantomData;

use crate::{
    marker::UnalignedField,
    ops::{Project, ProjectMut, Projectable, SafeProject},
};

pub use ::field_projection_internal::{HasFields, p, start_proj};

#[macro_export]
macro_rules! field_of {
    ($ty:ty, $field:ident) => {
        <$ty as $crate::compat::HasFields>::FieldInfo::<
            { $crate::compat::hash_field_name(::core::stringify!($field)) },
        >
    };
}

pub unsafe trait HasFields {
    type FieldInfo<const NAME: u64>;
}

pub use const_fnv1a_hash::fnv1a_hash_str_64 as hash_field_name;

pub unsafe trait ProjectableExt: Projectable {
    type Safety: private::Safety<Self>;
}

mod private {
    pub trait Safety<P>: Default {}
}

#[derive(Default)]
pub struct Safe;
impl<P: SafeProject> private::Safety<P> for Safe {}
impl Safe {
    pub fn check(self) {}
}

#[derive(Default)]
pub struct Unsafe;
impl<P: Projectable> private::Safety<P> for Unsafe {}
impl Unsafe {
    pub unsafe fn check(self) {}
}

pub unsafe trait CheckedProject<P: ProjectableExt> {
    type Checker: ProjectionChecker<Proj = P>;
}

pub unsafe trait ProjectionChecker {
    type Proj: ProjectableExt<Inner: CheckedProject<Self::Proj>>;
}

pub fn __start_proj<P>(proj: &P) -> (&<P::Inner as CheckedProject<P>>::Checker, *const P)
where
    P: ProjectableExt,
    P::Inner: CheckedProject<P>,
{
    (unsafe { core::mem::transmute(&()) }, proj)
}

pub fn __start_proj_mut<P>(proj: &mut P) -> (&mut <P::Inner as CheckedProject<P>>::Checker, *mut P)
where
    P: ProjectableExt,
    P::Inner: CheckedProject<P>,
{
    (unsafe { core::mem::transmute(&mut ()) }, proj)
}

pub fn __start_proj_move<P>(proj: P) -> (<P::Inner as CheckedProject<P>>::Checker, P)
where
    P: ProjectableExt,
    P::Inner: CheckedProject<P>,
{
    (unsafe { core::mem::transmute_copy(&()) }, proj)
}

pub struct ProjectedField<P, F>(PhantomData<P>, PhantomData<F>);

impl<P, F> ProjectedField<P, F>
where
    P: ProjectableExt,
    F: UnalignedField<Base = P::Inner>,
{
    pub unsafe fn __new() -> Self {
        Self(PhantomData, PhantomData)
    }

    pub fn safety_check(&self) -> P::Safety {
        Default::default()
    }

    pub unsafe fn project<'a>(&'a self, raw: *const P) -> <P as Project<F>>::Output<'a>
    where
        P: Project<F>,
    {
        unsafe { <P as Project<F>>::project(raw) }
    }

    pub unsafe fn project_mut<'a>(&'a mut self, raw: *mut P) -> <P as ProjectMut<F>>::OutputMut<'a>
    where
        P: ProjectMut<F>,
    {
        unsafe { <P as ProjectMut<F>>::project_mut(raw) }
    }

    pub unsafe fn project_move<'a>(self, raw: *const P) -> <P as Project<F>>::Output<'a>
    where
        P: Project<F>,
    {
        unsafe { <P as Project<F>>::project(raw) }
    }
}
