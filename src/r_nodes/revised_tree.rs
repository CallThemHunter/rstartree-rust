use crate::bounding_box::interface::BoundingBox;
use crate::r_nodes::interface::{Node, NodeManipulation, NodeState};

struct RRSTree<'a, D> {
    root: Box<Node<'a, D, Self>>,
    config: RRSTreeConfig,
}


struct RRSTreeConfig {
    min_elems: f64,
    max_elems: f64,
    shape_param: f64,
}


impl NodeState for RRSTree<'_, f64> {
    fn depth(&self) -> usize {
        0
    }

    fn height(&self) -> usize {
        self.root.height()
    }

    fn is_leaf(&self) -> bool {
        self.root.is_leaf()
    }

    fn is_root(&self) -> bool {
        true
    }

    fn num_elements(&self) -> usize {
        self.root.num_elements()
    }

    fn num_nodes(&self) -> usize {
        self.root.num_nodes()
    }
}


impl NodeManipulation<f64> for RRSTree<'_, f64> {
    fn insert(&mut self, element: BoundingBox<f64>) {
        self.root.insert(element)
    }

    fn remove(&mut self, element: BoundingBox<f64>) -> bool {
        self.root.remove(element)
    }

    fn query(&self, element: BoundingBox<f64>) -> bool {
        self.root.query(element)
    }
}