#![no_std]
#![forbid(unsafe_code)]
#![warn(
    missing_debug_implementations,
//    missing_docs,
    clippy::missing_const_for_fn,
)]

extern crate alloc;

use core::{fmt, marker::PhantomData};

pub mod cartesian;
pub mod reexports;

#[derive(Debug)]
pub struct Controller<M: Model> {
    _marker: PhantomData<M>,
}

impl<M: Model> Controller<M> {
    pub fn process_events(&mut self, _model: &mut M) {}
}

pub trait Model: Sized {
    type View: View<Self, ()>;
    type Msg: 'static;
}

pub trait View<ModelTy, Parent>: Sized
where
    ModelTy: Model,
    Parent: Model,
{
    /// Type defining the root element of the view.
    type Root: fmt::Debug;

    /// Returns a reference to the root of the view.
    fn root(&mut self) -> &mut Self::Root;
}

impl Model for () {
    type View = ();
    type Msg = ();
}

impl View<(), ()> for () {
    type Root = ();

    fn root(&mut self) -> &mut Self::Root {
        self
    }
}
