use crate::bounding_box::interface::BoundingBox;

struct Node<'a, T> {
    bounds: BoundingBox<T>,
    parent: Option<&'a Node<'a, T>>,
    children: Option<Vec<&'a Node<'a, T>>>,
}

trait Searchable<T> {
    fn search(&self) -> Option<T>;
}
