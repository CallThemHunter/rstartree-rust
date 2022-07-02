use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct BoundingBox<T> {
    pub lower_bounds: Vec<T>,
    pub upper_bounds: Vec<T>,
}

pub struct IndexOutOfBoundsError;

impl Display for IndexOutOfBoundsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "incorrect index bounds")
    }
}

pub trait HasMargin<T> {
    fn margin(&self) -> T;

    fn margin_diff(&self, other: &Self) -> T;
}

pub trait HasVolume<T> {
    fn volume(&self) -> T;

    fn volume_diff(&self, other: &Self) -> T;
}

pub trait Overlap<T>: Sized {
    fn overlap(&self, other: &Self) -> Option<Self>;

    fn encloses(&self, other: &Self) -> bool;
}

pub trait Geometry<T> {
    fn center(&self) -> Vec<T>;

    fn center_along(&self, dim: usize) -> Result<T, IndexOutOfBoundsError>;

    fn shape(&self) -> Vec<T>;

    fn width_of(&self, dim: usize) -> Result<f64, IndexOutOfBoundsError>;

    fn asymmetry(&self, other: &Self, dim: usize) -> Result<f64, IndexOutOfBoundsError>;
}
