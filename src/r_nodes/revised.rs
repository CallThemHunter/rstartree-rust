use crate::bounding_box::interface::BoundingBox;
use crate::r_nodes::basic::{Node, TreeManipulation, TreeState};

struct Tree<'a, T> {
    root: Node<'a, T>,
    config: TreeConfig,
}


struct TreeConfig {
    min_elems: f64,
    max_elems: f64,
    shape_param: f64,
}


trait RevisedTreeSplits<T> {
    fn split_needed(&self) -> bool;

    fn split_node(&self);
}

impl RevisedTreeSplits<f64> for Node<'_, f64> {
    fn split_needed(&self) -> bool {
        todo!()
    }

    fn split_node(&self) {
        todo!()
    }
}

impl TreeManipulation<f64> for Node<'_, f64> {
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
