#![no_std]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![feature(fundamental)]
#![warn(unsafe_op_in_unsafe_fn)]

use core::marker::PhantomData;

pub use field_projection_internal::Field;

/// Representation of a possible field of a specific name of a struct.
///
/// A field of `T` with name `x` is represented with `FieldOffset<T, {field_name_hash("x")}>`.
///
/// It should be noted that presence of `FieldOffset` does not represent that the field exists in
/// `T`; presence of the field is indicated by implementation of the `Field` trait.
///
/// This type is fundamental so that downstream crates can implement trait for it.
#[fundamental]
pub struct FieldOffset<T: ?Sized, const N: u64>(PhantomData<*mut T>);

pub use const_fnv1a_hash::fnv1a_hash_str_64 as field_name_hash;

/// Information of a field of a struct.
///
/// # Safety
/// The field must represent a field named `NAME` in a type `Base` that has the type `Type`.
/// The `map` function must be implemented such that it returns a pointer to the field.
pub unsafe trait Field {
    /// The type that contains the field.
    type Base: ?Sized;
    /// The type of the field.
    type Type: ?Sized;
    /// The name of the field.
    const NAME: &'static str;

    /// Adjust the pointer from the containing struct to the field.
    fn map(ptr: *const Self::Base) -> *const Self::Type;
}
