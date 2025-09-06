use std::field::Field;
use std::ops::Receiver;

use field_projection::{
    compat::{ProjectableExt, Safe},
    ops::{Project, ProjectMut, Projectable, SafeProject},
};

pub struct CppMutRef<T: ?Sized>(*mut T);

impl<T: ?Sized> CppMutRef<T> {
    pub fn write(&mut self, value: T)
    where
        T: Sized + Copy,
    {
        unsafe { self.0.write(value) }
    }
}

impl<T: ?Sized> Receiver for CppMutRef<T> {
    type Target = T;
}

impl<T: ?Sized> Projectable for CppMutRef<T> {
    type Inner = T;
}

unsafe impl<T: ?Sized> SafeProject for CppMutRef<T> {}

unsafe impl<T: ?Sized> ProjectableExt for CppMutRef<T> {
    type Safety = Safe;
}

impl<T, F> ProjectMut<F> for CppMutRef<T>
where
    F: Field<Base = T>,
    F::Type: Sized,
{
    type OutputMut<'a>
        = CppMutRef<F::Type>
    where
        Self: 'a;

    unsafe fn project_mut<'a>(this: *mut Self) -> Self::OutputMut<'a>
    where
        Self: 'a,
    {
        CppMutRef(unsafe { <*mut T as Project<F>>::project(&raw const (*this).0) })
    }
}
