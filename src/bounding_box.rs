use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::iter::zip;

struct IndexOutOfBoundsError;

impl Display for IndexOutOfBoundsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "incorrect index bounds")
    }
}

struct BoundingBox {
    lower_bounds: Vec<f64>,
    upper_bounds: Vec<f64>,
}

impl Display for BoundingBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Debug for BoundingBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

trait HasMargin<T> {
    fn margin(&self) -> T;

    fn margin_diff(&self, other: &Self) -> T;
}

impl HasMargin<f64> for BoundingBox {
    fn margin(&self) -> f64 {
        let mut margin = 0.0;

        let shape = self.shape();
        let length_iter = shape.iter();
        for length in length_iter {
            margin += length + length
        }

        margin
    }

    fn margin_diff(&self, other: &Self) -> f64 {
        num::abs(self.margin() - other.margin())
    }
}

trait HasVolume<T> {
    fn volume(&self) -> T;

    fn volume_diff(&self, other: &Self) -> T;
}

impl HasVolume<f64> for BoundingBox {
    fn volume(&self) -> f64 {
        let mut volume = 1.0;

        let shape = self.shape();
        let length_iter = shape.iter();
        for length in length_iter {
            volume *= length;
        }

        volume
    }

    fn volume_diff(&self, other: &Self) -> f64 {
        num::abs(self.volume() - other.volume())
    }
}

trait Overlap<T>: Sized {
    fn overlap(&self, other: &Self) -> Option<Self>;

    fn encloses(&self, other: &Self) -> bool;
}

impl Overlap<f64> for BoundingBox {
    fn overlap(&self, other: &Self) -> Option<Self> {
        let mut overlap_lower: Vec<f64> = vec![];
        let mut overlap_upper: Vec<f64> = vec![];

        let lows_iter = self.lower_bounds.iter().zip(other.lower_bounds.iter());
        for (own_lower, other_lower) in lows_iter {
            overlap_lower.push(own_lower.max(*other_lower))
        }

        let high_iter = self.upper_bounds.iter().zip(other.upper_bounds.iter());
        for (own_upper, other_upper) in high_iter {
            overlap_upper.push(own_upper.min(*other_upper))
        }

        let validation_iter = overlap_lower.iter().zip(overlap_upper.iter());
        for (lower, upper) in validation_iter {
            if lower.gt(upper) {
                return None
            }
        }

        Some(Self { lower_bounds: overlap_lower, upper_bounds: overlap_upper })
    }

    fn encloses(&self, other: &Self) -> bool {
        for (own_bound, other_bound) in zip(&self.lower_bounds, &other.lower_bounds) {
            if other_bound < own_bound {
                return false
            }
        }

        for (own_bound, other_bound) in zip(&self.upper_bounds, &other.upper_bounds) {
            if other_bound > own_bound {
                return false
            }
        }

        true
    }
}

trait Geometry<T> {
    fn center(&self) -> Vec<T>;

    fn center_along(&self, dim: usize) -> Result<T, IndexOutOfBoundsError>;

    fn shape(&self) -> Vec<T>;

    fn width_of(&self, dim: usize) -> Result<f64, IndexOutOfBoundsError>;

    fn asymmetry(&self, other: &Self, dim: usize) -> f64;
}

impl Geometry<f64> for BoundingBox {
    fn center(&self) -> Vec<f64> {
        let mut center: Vec<f64> = vec![];
        for (lower, upper) in self.lower_bounds.iter().zip(self.upper_bounds.iter()) {
            center.push((lower + upper) / 2.0)
        };

        center
    }

    fn center_along(&self, dim: usize) -> Result<f64, IndexOutOfBoundsError> {
        let lower = self.lower_bounds.get(dim);
        let lower = match lower {
            None => return Err(IndexOutOfBoundsError),
            Some(x) => x
        };

        let upper = self.upper_bounds.get(dim);
        let upper = match upper {
            None => return Err(IndexOutOfBoundsError),
            Some(x) => x
        };

        Ok((upper + lower) / 2.0)
    }

    fn shape(&self) -> Vec<f64> {
        let mut shape: Vec<f64> = vec![];
        for (lower, upper) in self.lower_bounds.iter().zip(self.upper_bounds.iter()) {
            shape.push(upper - lower)
        }

        shape
    }

    fn width_of(&self, dim: usize) -> Result<f64, IndexOutOfBoundsError> {
        let upper = self.upper_bounds.get(dim);
        let upper = match upper {
            None => return Err(IndexOutOfBoundsError),
            Some(x) => x
        };

        let lower = self.lower_bounds.get(dim);
        let lower = match lower {
            None => return Err(IndexOutOfBoundsError),
            Some(x) => x
        };

        Ok(upper - lower)
    }

    fn asymmetry(&self, other: &Self, dim: usize) -> f64 {
        todo!()
    }
}
