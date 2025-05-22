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

/// Shared projection operation `@base->field`.
pub trait Project<F>: Projectable
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
    unsafe fn project<'a>(this: *const Self) -> Self::Output<'a>
    where
        Self: 'a;
}

/// Exclusive projection operation `@mut base->field`.
pub trait ProjectMut<F>: Projectable
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
    /// * this function must only be called once.
    unsafe fn project_mut<'a>(this: *mut Self) -> Self::OutputMut<'a>
    where
        Self: 'a;
}

/// Marks project operations as safe.
///
/// # Safety
///
/// * the `@base->field` and `@mut base->field` operations must be safe.
pub unsafe trait SafeProject: Projectable {}

include!("./projections/maybe_uninit.rs");
include!("./projections/non_null.rs");
include!("./projections/pin.rs");
include!("./projections/raw_ptr.rs");
include!("./projections/unsafe_cell.rs");
