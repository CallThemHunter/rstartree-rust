use crate::bounding_box::interface::BoundingBox;
use crate::r_nodes::interface::{Children, Node, NodeCore, NodeManipulation};

trait NodeHeuristics<D, R> {
    fn split_needed(&self) -> bool;

    fn split_node(&self);

    fn lower_check(&self);

    fn choose_subtree(&self, nodes: &Vec<Node<D, R>>) -> usize;
}


impl<D, R> NodeHeuristics<D, R> for Node<D, R> {
    fn split_needed(&self) -> bool {
        todo!()
    }

    fn split_node(&self) {
        todo!()
    }

    fn lower_check(&self) { todo!() }

    fn choose_subtree(&self, nodes: &Vec<Node<D, R>>) -> usize { todo!() }
}


// a bit messy for now
impl<D, R> NodeManipulation<D, R> for Node<D, R> {
    fn insert(&mut self, element: BoundingBox<D>) {
        if self.split_needed() {
            match &mut self.children.clone().into_inner() {
                Children::Boxes(_) => {
                    self.split_node();

                    self.root().insert(element);
                    return
                },
                Children::Nodes(_) => {
                    self.split_node();

                    self.root().insert(element);
                    return
                }
            }
        } else {
            match self.children.clone().into_inner() {
                Children::Boxes(mut boxes) => {
                    // self.lower_check();
                    // boxes.push(element);
                    // return
                },
                Children::Nodes(nodes) => {
                    // let mut child = self.choose_subtree(nodes).remake();
                    // // let child = self.choose_subtree(nodes);
                    //
                    // child.insert(element);
                    // return
                },
            }
        }
    }

    fn remove(&mut self, element: BoundingBox<D>) -> bool {
        todo!()
    }

    fn query(&self, element: BoundingBox<D>) -> bool {
        todo!()
    }
}
