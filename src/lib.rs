#![no_std]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![warn(unsafe_op_in_unsafe_fn)]

use core::mem::MaybeUninit;

mod pin;

pub use field_projection_internal::*;
pub use pin::*;

/// Representation of a field name.
///
/// A field name `x` is represented with `FieldName<{field_name_hash("x")}>`.
pub struct FieldName<const N: u64>(());

pub use const_fnv1a_hash::fnv1a_hash_str_64 as field_name_hash;

/// Information of a field of struct `Base`.
///
/// # Safety
/// The field must represent a field named `NAME` in a type `Base` that has the type `Type`.
/// The `map` function must be implemented such that it returns a pointer to the field.
///
/// This trait should not be implemented manually; instead, use the `#[derive(Field)]` instead.
pub unsafe trait Field<Base> {
    /// The type of the field.
    type Type: ?Sized;
    /// The name of the field.
    const NAME: &'static str;

    /// Adjust the pointer from the containing struct to the field.
    ///
    /// # Safety
    /// `ptr` must be a non-null and aligned pointer to `Self::Base`.
    unsafe fn map(ptr: *const Base) -> *const Self::Type;
}

/// Trait for a wrapper type that can be projected to a field.
///
/// `F` is a descriptor of a field (`FieldName` with some generic parameters).
pub trait Projectable<T, F: Field<T>> {
    /// Type of the wrapped projected field.
    type Target;

    /// Project the field.
    ///
    /// # Safety
    /// The function must be called only if `F` is accessible with Rust privacy
    /// rules by the caller.
    unsafe fn project(self) -> Self::Target;

    #[doc(hidden)]
    unsafe fn project_with_check(this: Self, _check: fn(&T)) -> Self::Target
    where
        Self: Sized,
    {
        unsafe { Self::project(this) }
    }
}

impl<'a, T, F> Projectable<T, F> for &'a MaybeUninit<T>
where
    F: Field<T>,
    F::Type: Sized + 'a,
{
    type Target = &'a MaybeUninit<F::Type>;

    unsafe fn project(self) -> Self::Target {
        // SAFETY: Projecting through trusted `F::map`.
        unsafe { &*F::map(self.as_ptr()).cast::<MaybeUninit<F::Type>>() }
    }
}

impl<'a, T, F> Projectable<T, F> for &'a mut MaybeUninit<T>
where
    F: Field<T>,
    F::Type: Sized + 'a,
{
    type Target = &'a mut MaybeUninit<F::Type>;

    unsafe fn project(self) -> Self::Target {
        // SAFETY: Projecting through trusted `F::map`.
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
        match $a {
            __expr => unsafe {
                $crate::Projectable::<
                    _,
                    $crate::FieldName<{ $crate::field_name_hash(core::stringify!($b)) }>,
                >::project_with_check(__expr, |__check| {
                    let _ = __check.$b;
                })
            },
        }
    };
}
