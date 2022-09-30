#![no_std]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![feature(fundamental)]
#![warn(unsafe_op_in_unsafe_fn)]

use core::marker::PhantomData;
use core::mem::MaybeUninit;

mod pin;

pub use field_projection_internal::{Field, PinField};
pub use pin::PinField;

/// Representation of a possible field of a specific name of a struct.
///
/// A field of `T` with name `x` is represented with `FieldOffset<T, {field_name_hash("x")}>`.
///
/// It should be noted that presence of `FieldOffset` does not represent that the field exists in
/// `T`; presence of the field is indicated by implementation of the `Field` trait.
///
/// This type is fundamental so that downstream crates can implement trait for it.
#[fundamental]
pub struct FieldOffset<T: ?Sized, const N: u64>(PhantomData<T>);

pub use const_fnv1a_hash::fnv1a_hash_str_64 as field_name_hash;

/// Information of a field of a struct.
///
/// # Safety
/// The field must represent a field named `NAME` in a type `Base` that has the type `Type`.
/// The `map` function must be implemented such that it returns a pointer to the field.
///
/// This trait should not be implemented manually; instead, use the `#[derive(Field)]` instead.
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

/// Trait for a wrapper type that can be projected to a field.
///
/// `F` is a descriptor of a field (`FieldOffset` with some generic parameters).
pub trait Projectable<F: Field> {
    /// Type of the wrapped projected field.
    type Target;

    /// Project the field.
    fn project(self) -> Self::Target;
}

impl<'a, T, F> Projectable<F> for &'a MaybeUninit<T>
where
    F: Field<Base = T>,
    F::Type: Sized + 'a,
{
    type Target = &'a MaybeUninit<F::Type>;

    fn project(self) -> Self::Target {
        unsafe { &*F::map(self.as_ptr()).cast::<MaybeUninit<F::Type>>() }
    }
}

impl<'a, T, F> Projectable<F> for &'a mut MaybeUninit<T>
where
    F: Field<Base = T>,
    F::Type: Sized + 'a,
{
    type Target = &'a mut MaybeUninit<F::Type>;

    fn project(self) -> Self::Target {
        unsafe {
            &mut *F::map(self.as_mut_ptr())
                .cast_mut()
                .cast::<MaybeUninit<F::Type>>()
        }
    }
}

#[macro_export]
macro_rules! project {
    ($a:expr => $b:ident) => {
        $crate::Projectable::<
            $crate::FieldOffset<_, { $crate::field_name_hash(core::stringify!($b)) }>,
        >::project($a)
    };
}
