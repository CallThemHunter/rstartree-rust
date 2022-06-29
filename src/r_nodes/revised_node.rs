use crate::bounding_box::interface::BoundingBox;
use crate::r_nodes::interface::{Children, Node, NodeManipulation, NodeState, NodeTraversal};
use crate::r_nodes::interface::Parent::Tree;

trait NodeHeuristics<D, R> {
    fn split_needed(&mut self) -> bool;

    fn split_node(&self);

    fn lower_check(&self);

    fn choose_subtree(&self) -> Node<'_, D, R>;
}


impl<D, R> NodeHeuristics<D, R> for Node<'_, D, R> {
    fn split_needed(&mut self) -> bool {
        todo!()
    }

    fn split_node(&self) {
        todo!()
    }

    fn lower_check(&self) { todo!() }

    fn choose_subtree(&self) -> Node<'_, D, R> { todo!() }
}


impl<D, R> NodeManipulation<D> for Node<'_, D, R> {
    fn insert(&mut self, element: BoundingBox<D>) {
        match &mut self.children {
            Children::Boxes(boxes) => {
                if self.split_needed() {
                    self.split_node();

                    self.root().insert(element);
                    return
                } else {
                    self.lower_check();
                    boxes.push(element);
                }
            }
            Children::Nodes(nodes) => {
                if self.split_needed() {
                    self.split_node();

                    self.root().insert(element);
                } else {
                    let &mut child = &mut self.choose_subtree();
                }
            }
        }
        todo!()
    }

    fn remove(&mut self, element: BoundingBox<D>) -> bool {
        todo!()
    }

    fn query(&self, element: BoundingBox<D>) -> bool {
        todo!()
    }
}
