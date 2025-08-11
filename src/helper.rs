use core::ptr;

use crate::marker::Field;

pub fn project_ref<F: Field>(r: &F::Base) -> &F::Type
where
    F::Type: Sized,
{
    unsafe { &*ptr::from_ref(r).byte_add(F::OFFSET).cast() }
}
