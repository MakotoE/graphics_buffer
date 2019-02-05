#![deny(missing_docs)]

//! This library provides a buffer which can be used as a render target for [Piston's graphics library](https://github.com/PistonDevelopers/graphics). This buffer can be loaded from and/or saved to a file on disk. This allows for things like screenshots in games.

// !There is also an optional feature for `RenderBuffer` that allows it to be converted into a `G2dTexture` so that it can be rendered with [`piston_window`](https://github.com/PistonDevelopers/piston_window). To enable this, add `features = ["piston_window_texture"]` to the `graphics_buffer` dependency in your `cargo.toml`.

mod glyphs;
mod hardware_impl;
mod software_impl;
pub use crate::glyphs::*;
pub use crate::hardware_impl::*;
pub use crate::software_impl::*;

use std::{error, fmt};

use graphics::math::Matrix2d;

/// The identity matrix: `[[1.0, 0.0, 0.0], [0.0, 1.0, 0.0]]`.
pub const IDENTITY: Matrix2d = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];

/// An Error type for `RenderBuffer`.
#[derive(Debug, Clone)]
pub enum Error {
    /// There is a mismatch between container size and number of pixels
    ContainerTooSmall(usize, usize),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ContainerTooSmall(len, area) => write!(
                f,
                "Container is too small for the given dimensions. \
                 \nContainer has {} bytes, which encode {} pixels, \
                 \nbut the given demensions contain {} pixels",
                len,
                len / 4,
                area
            ),
        }
    }
}

impl error::Error for Error {}
