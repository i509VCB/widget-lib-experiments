//! A kind of model which represents a view in a cartesian coordinate space.

use alloc::collections::TryReserveError;
use tinyvec::TinyVec;

use crate::{Model, View};

/// Layout constraints.
///
///
#[derive(Debug, Clone, Copy)]
pub struct LayoutConstraints {
    min: Size<u32>,
    max: Size<u32>,
}

impl LayoutConstraints {
    pub const UNBOUNDED: Self = Self::new(
        Size { w: 0, h: 0 },
        Size {
            w: u32::MAX,
            h: u32::MAX,
        },
    );

    pub const fn new(min: Size<u32>, max: Size<u32>) -> Self {
        Self { min, max }
    }

    pub const fn tight(size: Size<u32>) -> Self {
        Self {
            min: size,
            max: size,
        }
    }

    pub const fn min(&self) -> Size<u32> {
        self.min
    }

    pub const fn max(&self) -> Size<u32> {
        self.max
    }

    pub const fn loosen(&self) -> Self {
        todo!()
    }

    pub const fn constrain(&self, _size: Size<u32>) -> Self {
        todo!()
    }

    // TODO: aspect ratio helpers?
    // TODO: lerp?
}

/// A view which is positioned in a cartesian coordinate space.
pub trait CartesianView<ModelTy, Parent>: View<ModelTy, Parent>
where
    ModelTy: Model,
    Parent: Model,
{
    fn layout(&mut self, model: &ModelTy, constraints: LayoutConstraints) -> Rect<u32>;

    /// Returns the damage of the element since it's last update.
    fn accumulate_damage(
        &self,
        model: &ModelTy,
    ) -> Result<TinyVec<[Rect<u32>; 4]>, TryReserveError>;
}

/// A size is defined by it's x and y position.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Point<T> {
    /// The x position.
    pub x: T,

    /// The y position.
    pub y: T,
}

/// A size is defined by it's width and height.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Size<T> {
    /// The vertical coordinate.
    pub w: T,

    /// The horizontal coordinate.
    pub h: T,
}

/// A rectangle defined by it's top-left corner and dimensions.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Rect<T> {
    pub position: Point<T>,

    pub size: Size<T>,
}
