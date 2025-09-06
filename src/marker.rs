use core::field::UnalignedField;

/// Type representing a field of a `struct`, `union` or tuple with structural pinning information.
///
/// # Safety
///
/// `Self::Projected<'a>` either is `Pin<&'a mut Self::Type>` or `&'a mut Self::Type`. In the first
/// case the field is structurally pinned.
pub unsafe trait PinnableField: UnalignedField {
    /// The pin-projected type of `Self`.
    ///
    /// Either `Pin<&'a mut Self::Type>` or `&'a mut Self::Type`.
    type Projected<'a>
    where
        Self::Type: 'a;

    /// Sets the correct value for a pin projection.
    ///
    /// # Safety
    ///
    /// The supplied reference must be derived from a `Pin<&mut Self::Base>`.
    unsafe fn from_pinned_ref(r: &mut Self::Type) -> Self::Projected<'_>;
}

pub use crate::field_of;
