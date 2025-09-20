use crate::{compat, marker::PinnableField};

use core::field::Field;

/// Type supporting field projections.
///
/// The exact kind of field projection is governed by [`Project`] and [`ProjectMut`]. This trait
/// only gives the compiler access to the `Self::Inner` type which is the type containing the
/// potentially projectable fields. So given an expression `base` of type `Self`, then the `field`
/// ident in the expressions `@base->field` and `@mut base->field` refer to a field of the type
/// `Self::Inner`.
///
/// Weather the projection `@[mut] base->field` is available still depends on whether `Self`
/// implements `Project[Mut]<field_of!(Self::Inner, field)>`.
pub trait Projectable: Sized {
    type Inner: ?Sized;
}

/// Marks project operations on `Self` as safe.
///
/// # Safety
///
/// * The [`Project::project`] and [`ProjectMut::project_mut`] functions implemented for `Self`
///   must not have additional safety requirements.
pub unsafe trait SafeProject: Projectable {}

/// Shared projection operation `@base->field`.
pub trait Project<F>: Projectable
where
    F: Field<Base = Self::Inner>,
{
    /// The output of this projection operation.
    type Output<'a>
    where
        Self: 'a;

    /// Projects the base to the output.
    ///
    /// # Safety
    ///
    /// * `this` must be a valid pointer pointing at a valid value of `Self`.
    /// * for the duration of `'a`, the value at `this` is only used by other projection
    ///   operations.
    /// * for the duration of `'a`, the value at `this` is not mutably projected with `F`.
    /// * Implementers may impose additional safety requirements. These must be documented on the
    ///   implementation of this trait.
    unsafe fn project<'a>(this: *const Self) -> Self::Output<'a>
    where
        Self: 'a;
}

/// Exclusive projection operation `@mut base->field`.
pub trait ProjectMut<F>: Projectable
where
    F: Field<Base = Self::Inner>,
{
    /// The output of this projection operation.
    type OutputMut<'a>
    where
        Self: 'a;

    /// Projects the base to the output.
    ///
    /// # Safety
    ///
    /// * `this` must be a valid pointer pointing at a valid value of `Self`.
    /// * For the duration of `'a`, the value at `this` is only used by other projection
    ///   operations for fields other than `F`.
    /// * Implementers may impose additional safety requirements. These must be documented on the
    ///   implementation of this trait.
    unsafe fn project_mut<'a>(this: *mut Self) -> Self::OutputMut<'a>
    where
        Self: 'a;
}

include!("./projections/maybe_uninit.rs");
include!("./projections/non_null.rs");
include!("./projections/pin.rs");
include!("./projections/raw_ptr.rs");
include!("./projections/ref_cell.rs");
include!("./projections/unsafe_cell.rs");
