use crate::*;
use core::pin::Pin;

/// Additional information on a field of a struct regarding to pinning.
///
/// # Safety
/// `PinWrapper` must be layout-compatible with `&mut Self::Type`. If the field is pinned, then
/// it should be `Pin<&mut Self::Type>`, otherwise it should be `&mut Self::Type`.
///
/// This trait should not be implemented manually; instead, use the `#[derive(PinField)]` instead.
pub unsafe trait PinField: Field {
    /// The type when this field is projected from a `Pin<&mut Self::Base>`.
    type PinWrapper<'a, T: ?Sized + 'a>;
}

impl<'a, T, F> Projectable<F> for Pin<&'a mut T>
where
    F: PinField<Base = T>,
    F::Type: 'a,
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
