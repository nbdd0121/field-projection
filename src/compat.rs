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
    type RefChecker<'a>: ProjectionRefChecker<'a, Proj = P>;
    type ValChecker: ProjectionValChecker<Proj = P>;
}

pub unsafe trait ProjectionRefChecker<'a> {
    type Proj: ProjectableExt<Inner: CheckedProject<Self::Proj>>;

    unsafe fn __create(
        proj: RawProjectedRef<'a, Self::Proj, <Self::Proj as Projectable>::Inner>,
    ) -> Self;
}

pub unsafe trait ProjectionValChecker {
    type Proj: ProjectableExt<Inner: CheckedProject<Self::Proj>>;

    unsafe fn __create(
        proj: RawProjectedVal<Self::Proj, <Self::Proj as Projectable>::Inner>,
    ) -> Self;
}

pub fn __start_proj<'a, P>(proj: &'a P) -> <P::Inner as CheckedProject<P>>::RefChecker<'a>
where
    P: ProjectableExt,
    P::Inner: CheckedProject<P>,
{
    let ptr: *const P = proj;
    let proj = RawProjectedRef(ptr, PhantomData);
    unsafe { <P::Inner as CheckedProject<P>>::RefChecker::__create(proj) }
}

pub fn __start_proj_mut<'a, P>(proj: &'a mut P) -> <P::Inner as CheckedProject<P>>::RefChecker<'a>
where
    P: ProjectableExt,
    P::Inner: CheckedProject<P>,
{
    let ptr: *mut P = proj;
    let proj = RawProjectedRef(ptr, PhantomData);
    unsafe { <P::Inner as CheckedProject<P>>::RefChecker::__create(proj) }
}

pub fn __start_proj_move<P>(proj: P) -> <P::Inner as CheckedProject<P>>::ValChecker
where
    P: ProjectableExt,
    P::Inner: CheckedProject<P>,
{
    let proj = RawProjectedVal(proj, PhantomData);
    unsafe { <P::Inner as CheckedProject<P>>::ValChecker::__create(proj) }
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

    pub unsafe fn project<'a>(
        self: &'a ProjectedField<P, F>,
        raw: RawProjectedPtr<P>,
    ) -> <P as Project<F>>::Output<'a>
    where
        P: Project<F>,
    {
        unsafe { <P as Project<F>>::project(raw.0) }
    }

    pub unsafe fn project_mut<'a>(
        self: &'a mut ProjectedField<P, F>,
        raw: RawProjectedPtr<P>,
    ) -> <P as ProjectMut<F>>::OutputMut<'a>
    where
        P: ProjectMut<F>,
    {
        unsafe { <P as ProjectMut<F>>::project_mut(raw.0.cast_mut()) }
    }
}

pub struct RawProjectedPtr<P>(*const P);

pub struct RawProjectedRef<'a, P, T: ?Sized>(*const P, PhantomData<(&'a (), &'a mut (), P, T)>);

pub struct RawProjectedVal<P, T: ?Sized>(P, PhantomData<(P, T)>);

pub trait RawProjectionAccess<P> {
    fn access(&self) -> RawProjectedPtr<P>;
    fn access_mut(&mut self) -> RawProjectedPtr<P>;
}

impl<'a, P, T: ?Sized> RawProjectionAccess<P> for RawProjectedRef<'a, P, T> {
    fn access(&self) -> RawProjectedPtr<P> {
        RawProjectedPtr(self.0)
    }

    fn access_mut(&mut self) -> RawProjectedPtr<P> {
        RawProjectedPtr(self.0)
    }
}

impl<P, T: ?Sized> RawProjectionAccess<P> for RawProjectedVal<P, T> {
    fn access(&self) -> RawProjectedPtr<P> {
        RawProjectedPtr(&raw const self.0)
    }

    fn access_mut(&mut self) -> RawProjectedPtr<P> {
        RawProjectedPtr(&raw mut self.0)
    }
}
