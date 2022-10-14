use crate::*;
use core::pin::Pin;
use core::mem::MaybeUninit;

/// Additional information on a field of a struct regarding to pinning.
///
/// # Safety
/// `PinWrapper` must be layout-compatible with `&mut Self::Type`. If the field is pinned, then
/// it should be `Pin<&mut Self::Type>`, otherwise it should be `&mut Self::Type`.
///
/// This trait should not be implemented manually; instead, use the `#[derive(PinField)]` instead.
pub unsafe trait PinField<T>: Field<T> {
    /// The type when this field is projected from a `Pin<&mut Self::Base>`.
    type PinWrapper<'a, U: ?Sized + 'a>;

    /// The type when this field is projected from a `Pin<&mut MaybeUninit<Self::Base>>`.
    type PinMaybeUninitWrapper<'a, U: 'a>;
}

impl<'a, T, F> Projectable<T, F> for Pin<&'a mut T>
where
    F: PinField<T>,
    F::Type: 'a,
    T: HasField,
{
    type Target = F::PinWrapper<'a, F::Type>;

    fn project(self) -> Self::Target {
        // SAFETY: This pointer will not be moved out, and the resulting projection will be wrapped
        // with `Pin` back if the field is pinned.
        let inner = unsafe { Self::into_inner_unchecked(self) };
        // SAFETY: Project the pointer through raw pointer. Note that the `*mut _` cast is important
        // as otherwise the `&mut` to `*const` cast will go through `&` reference which will retag it.
        let ptr = unsafe { &mut *F::map(inner as *mut _).cast_mut() };
        // This is either a `Pin<&mut T>` or `&mut T`, both layout compatible with `&mut T`.
        // Use `transmute_copy` here because the compiler can't prove that `F::PinWrapper` is of
        // the same size.
        unsafe { core::mem::transmute_copy(&ptr) }
    }
}

impl<'a, T, F> Projectable<T, F> for Pin<&'a mut MaybeUninit<T>>
where
    F: PinField<T>,
    F::Type: Sized + 'a,
{
    type Target = F::PinMaybeUninitWrapper<'a, F::Type>;

    fn project(self) -> Self::Target {
        // SAFETY: This pointer will not be moved out, and the resulting projection will be wrapped
        // with `Pin` back if the field is pinned.
        let inner = unsafe { Self::into_inner_unchecked(self) };
        // SAFETY: Project the pointer through raw pointer.
        let ptr = unsafe { &mut *F::map(inner.as_mut_ptr()).cast_mut() };
        unsafe { core::mem::transmute_copy(&ptr) }
    }
}


#[doc(hidden)]
pub struct AlwaysUnpin<T>(T);
impl<T> Unpin for AlwaysUnpin<T> {}
