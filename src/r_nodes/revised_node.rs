use crate::bounding_box::interface::BoundingBox;
use crate::r_nodes::interface::{Children, Node, NodeManipulation, NodeTraversal};

trait NodeHeuristics<'a, D, R> {
    fn split_needed(&self) -> bool;

    fn split_node(&self);

    fn lower_check(&self);

    fn choose_subtree(&self, nodes: &Vec<&Node<'a, D, R>>) -> Node<'a, D, R>;
}


impl<'a, D, R> NodeHeuristics<'a, D, R> for Node<'a, D, R> {
    fn split_needed(&self) -> bool {
        todo!()
    }

    fn split_node(&self) {
        todo!()
    }

    fn lower_check(&self) { todo!() }

    fn choose_subtree(&self, nodes: &Vec<&Node<'a, D, R>>) -> Node<'a, D, R> { todo!() }
}


impl<'a, D, R> NodeManipulation<'a, D> for Node<'a, D, R> {
    fn insert(&'a mut self, element: BoundingBox<D>) {
        match &self.children {
            Children::Boxes(mut boxes) => {
                if self.split_needed() {
                    self.split_node();

                    self.root_mut().insert(element);
                    return
                } else {
                    self.lower_check();
                    boxes.push(element);
                    return
                }
            }
            Children::Nodes(mut nodes) => {
                if self.split_needed() {
                    self.split_node();

                    self.root().insert(element);
                    return
                } else {
                    let child = &mut self.choose_subtree(nodes.as_ref());

                    child.insert(element);
                    return
                }
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
