use super::*;

pub mod vector;
use vector::*;

pub mod matrix;
use matrix::*;

pub mod rectangle;
use rectangle::*;

pub mod grid;
use grid::*;

//pub mod experimental_grid;
//use experimental_grid::*;

pub mod transform;
use transform::*;

pub mod angle;
use angle::*;

pub mod coefficient;
use coefficient::*;

pub(crate) mod _prelude
{
    pub use super::{angle::{Angle, AngleOf}, coefficient::Coef, grid::prelude::*, matrix::prelude::*, rectangle::prelude::*, vector::prelude::*};
}

pub(crate) mod _traits 
{
    pub use super::{angle::{ToAngle,ToAngleComposite}, grid::traits::*, matrix::traits::*, rectangle::prelude::*, transform::*, vector::traits::*};
}
