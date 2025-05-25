/// Type representing a field of a `struct`, `union` or tuple.
///
/// # Safety
///
/// Given a valid value of type `Self::Base`, there exists a valid value of type `Self::Type` at
/// byte offset `OFFSET`.
pub unsafe trait UnalignedField: Sized {
    /// The type of the base where this field exists in.
    type Base: ?Sized;

    /// The type of the field.
    type Type: ?Sized;

    /// The offset of the field in bytes.
    const OFFSET: usize;
}

/// Type representing an aligned field of a `struct`, `union` or tuple.
///
/// # Safety
///
/// Given a well-aligned value of type `Self::Base`, the field at `Self::OFFSET` of type
/// `Self::Type` is well-aligned.
pub unsafe trait Field: UnalignedField {}

/// Type representing a field of a `struct`, `union` or tuple with structural pinning information.
///
/// # Safety
///
/// `Self::Projected<'a>` either is `Pin<&'a mut Self::Type>` or `&'a mut Self::Type`. In the first
/// case the field is structurally pinned.
pub unsafe trait PinableField: UnalignedField {
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
