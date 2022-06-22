
struct BoundingBox {
    lower_bound: Vec<f64>,
    upper_bound: Vec<f64>,
}


trait HasMargin<T> {
    fn margin(&self) -> T;

    fn margin_diff(&self, other: &Self) -> T;
}

trait HasVolume<T> {
    fn volume(&self) -> T;

    fn volume_diff(&self, other: &Self) -> T;
}

trait Overlap<T> {
    fn overlap(&self, other: &Self) -> Self;
}

trait Geometry<T> {
    fn center(&self) -> Vec<T>;

    fn center_along(&self, dim: i32) -> T;

    fn shape(&self) -> Vec<T>;

    fn width_of(&self, dim: i32) -> T;
}

impl HasMargin<f64> for BoundingBox {
    fn margin(&self) -> f64 {
        todo!()
    }

    fn margin_diff(&self, other: &Self) -> f64 {
        todo!()
    }
}

impl HasVolume<f64> for BoundingBox {
    fn volume(&self) -> f64 {
        todo!()
    }

    fn volume_diff(&self, other: &Self) -> f64 {
        todo!()
    }
}

impl Overlap<f64> for BoundingBox {
    fn overlap(&self, other: &Self) -> Self {
        todo!()
    }
}

impl Geometry<f64> for BoundingBox {
    fn center(&self) -> Vec<f64> {
        todo!()
    }

    fn center_along(&self, dim: i32) -> f64 {
        todo!()
    }

    fn shape(&self) -> Vec<f64> {
        todo!()
    }

    fn width_of(&self, dim: i32) -> f64 {
        todo!()
    }
}
