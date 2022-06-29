use crate::bounding_box::interface::BoundingBox;
use crate::r_nodes::interface::Children::{Boxes, Nodes};

pub enum Children<'a, T> {
    Boxes(Vec<BoundingBox<T>>),
    Nodes(Vec<&'a Node<'a, T>>),
}

pub struct Node<'a, T> {
    pub bounds: BoundingBox<T>,
    pub parent: Option<&'a Node<'a, T>>,
    pub children: Children<'a, T>,
}

pub trait NodeManipulation<T>: NodeState {
    fn insert(&self, element: BoundingBox<T>);

    fn remove(&self, element: BoundingBox<T>) -> bool;

    fn query(&self, element: BoundingBox<T>) -> bool;
}

pub trait NodeState {
    fn depth(&self) -> usize;

    fn height(&self) -> usize;

    fn is_leaf(&self) -> bool;

    fn is_root(&self) -> bool;

    fn num_elements(&self) -> usize;

    fn num_nodes(&self) -> usize;
}

impl<T> NodeState for Node<'_, T> {
    fn depth(&self) -> usize {
        match self.parent {
            None => 0,
            Some(n) => 1 + n.depth()
        }
    }

    fn height(&self) -> usize {
        // r trees are always balanced!
        // so you can use the first node on a list always
        match &self.children {
            Boxes(_) => 1,
            Nodes(n) => {
                match n.first() {
                    None => 0,
                    Some(x) => 1 + x.height(),
                }
            }
        }
    }

    fn is_leaf(&self) -> bool {
        match &self.children {
            Nodes(_) => false,
            Boxes(_) => true
        }
    }

    fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    fn num_elements(&self) -> usize {
        match &self.children {
            Nodes(nodes)
            => nodes
                .iter()
                .map(|node| { node.num_elements() })
                .sum(),
            Boxes(boxes) => boxes.len()
        }
    }

    fn num_nodes(&self) -> usize {
        match &self.children {
            Nodes(nodes)
            => nodes.iter()
                .map(|node| { node.num_nodes() })
                .sum(),
            Boxes(_) => 1
        }
    }
}
