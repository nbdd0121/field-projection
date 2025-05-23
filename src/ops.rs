use crate::{
    compat,
    marker::{Field, PinableField, UnalignedField},
};

/// Type supporting projections.
///
/// The exact kind of projection is governed by [`Project`] and [`ProjectMut`]. This trait only
/// gives the compiler access to the `Self::Inner` type which is the type containing the projected
/// fields. So given an expression `base` of type `Self`, then the `field` ident in the expressions
/// `@base->field` and `@mut base->field` refer to a field of the type `Self::Inner`.
pub trait Projectable: Sized {
    type Inner: ?Sized;
}

/// Marks project operations as safe.
///
/// # Safety
///
/// * The `@base->field` and `@mut base->field` operations implemented by the [`Project`] and
///   [`ProjectMut`] traits must not have additional safety requirements.
pub unsafe trait SafeProject: Projectable {}

/// Shared projection operation `@base->field`.
///
/// # Safety
///
///
pub unsafe trait Project<F>: Projectable
where
    F: UnalignedField<Base = Self::Inner>,
{
    /// The output of this projection operation.
    type Output<'a>
    where
        Self: 'a;

    /// Projects the base to the output.
    ///
    /// # Safety
    ///
    /// * `this` must be a dereferenceable pointer pointing at a valid value of `Self`.
    /// * for the duration of `'a`, the value at `this` is only used by other projection
    ///   operations.
    /// * for the duration of `'a`, the value at `this` is not mutably projected with `F`.
    unsafe fn project<'a>(this: *const Self) -> Self::Output<'a>
    where
        Self: 'a;
}

/// Exclusive projection operation `@mut base->field`.
///
/// # Safety
///
///
pub unsafe trait ProjectMut<F>: Projectable
where
    F: UnalignedField<Base = Self::Inner>,
{
    /// The output of this projection operation.
    type OutputMut<'a>
    where
        Self: 'a;

    /// Projects the base to the output.
    ///
    /// # Safety
    ///
    /// * `this` must be a dereferenceable pointer pointing at a valid value of `Self`.
    /// * for the duration of `'a`, the value at `this` is only used by other projection
    ///   operations for fields other than `F`.
    unsafe fn project_mut<'a>(this: *mut Self) -> Self::OutputMut<'a>
    where
        Self: 'a;
}

include!("./projections/maybe_uninit.rs");
include!("./projections/non_null.rs");
include!("./projections/pin.rs");
include!("./projections/raw_ptr.rs");
include!("./projections/unsafe_cell.rs");
