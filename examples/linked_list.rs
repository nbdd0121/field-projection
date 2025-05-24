use std::{marker::PhantomData, ptr::NonNull};

use field_projection::compat::{HasFields, p, start_proj};

#[derive(HasFields)]
struct ListItem<T> {
    next: NonNull<ListItem<T>>,
    prev: NonNull<ListItem<T>>,
    value: T,
}

impl<T> ListItem<T> {
    fn new(prev: NonNull<Self>, next: NonNull<Self>, value: T) -> NonNull<Self> {
        NonNull::from(Box::leak(Box::new(Self { prev, next, value })))
    }
}

pub struct List<T> {
    first: Option<NonNull<ListItem<T>>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { first: None }
    }

    /*
    pub fn push_front(&mut self, item: T) {
        let new = ListItem {
            next: self.first.take(),
            prev: None,
            value: item,
        };
        self.first = Some(NonNull::from(Box::leak(Box::new(new))));
    }

    pub fn push_back(&mut self, item: T) {}
    */

    pub fn cursor(&mut self) -> Option<Cursor<'_, T>> {
        self.first.map(|cur| Cursor {
            cur,
            _lt: PhantomData,
        })
    }
}

pub struct Cursor<'a, T> {
    cur: NonNull<ListItem<T>>,
    _lt: PhantomData<&'a mut T>,
}

impl<T> Cursor<'_, T> {
    pub fn insert_left(&mut self, item: T) {
        let cur = self.cur;
        let (prev, next) = (unsafe { (*cur.as_ptr()).prev }, cur);
        let new = ListItem::new(prev, cur, item);
        unsafe { (*prev.as_ptr()).next = new };
        unsafe { (*next.as_ptr()).prev = new };
    }

    pub fn insert_right(&mut self, item: T) {
        start_proj!(let cur = self.cur);
        start_proj!(let prev = cur; let next = unsafe { p!(@cur->next).read() });
        let new = ListItem::new(prev, cur, item);
        unsafe { p!(@prev->next).write(new) };
        unsafe { p!(@next->prev).write(new) };
        self.cur = new;
    }

    pub fn current(&mut self) -> &mut T {
        unsafe { &mut (*self.cur.as_ptr()).value }
    }
}

fn main() {}
