use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::ErrorKind::InvalidData;

//type Result<T> = std::result::Result<T, IndexOutOfBoundsError>;

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

trait HasMargin<T> {
    fn margin(&self) -> T;

    fn margin_diff(&self, other: &Self) -> T;
}

trait HasVolume<T> {
    fn volume(&self) -> T;

    fn volume_diff(&self, other: &Self) -> T;
}

trait Overlap<T>: Sized {
    fn overlap(&self, other: &Self) -> Option<Self>;
}

trait Geometry<T> {
    fn center(&self) -> Vec<T>;

    fn center_along(&self, dim: usize) -> Result<T, IndexOutOfBoundsError>;

    fn shape(&self) -> Vec<T>;

    fn width_of(&self, dim: usize) -> Result<f64, IndexOutOfBoundsError>;
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
        todo!()
    }
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
        todo!()
    }
}

impl Overlap<f64> for BoundingBox {
    fn overlap(&self, other: &Self) -> Option<Self> {
        todo!()
    }
}

impl Geometry<f64> for BoundingBox {
    fn center(&self) -> Vec<f64> {
        let mut center: Vec<f64> = vec![];
        for (lower, upper) in self.upper_bounds.iter().zip(self.upper_bounds.iter()) {
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
        todo!()
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
}
