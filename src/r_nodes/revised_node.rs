use crate::bounding_box::interface::BoundingBox;
use crate::r_nodes::interface::{Node, NodeManipulation, NodeState};

trait NodeSplits<D> {
    fn split_needed(&self) -> bool;

    fn split_node(&self);
}


impl<D, R> NodeSplits<f64> for Node<'_, D, R> {
    fn split_needed(&self) -> bool {
        todo!()
    }

    fn split_node(&self) {
        todo!()
    }
}


impl<D, R> NodeManipulation<f64> for Node<'_, D, R> {
    fn insert(&self, element: BoundingBox<f64>) {
        if self.is_leaf() {
            if self.split_needed() {
                self.split_node();

                if self.is_root() {
                    match self.parent {
                        None => panic!(),
                        Some(node) => node.insert(element)
                    }
                    todo!()
                }
            } else {}
        }
        todo!()
    }

    fn remove(&self, element: BoundingBox<f64>) -> bool {
        todo!()
    }

    fn query(&self, element: BoundingBox<f64>) -> bool {
        todo!()
    }
}
