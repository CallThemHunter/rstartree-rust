use crate::bounding_box::interface::BoundingBox;
use crate::r_nodes::basic::Children::{Boxes, Nodes};

pub enum Children<'a, T> {
    Boxes(Vec<BoundingBox<T>>),
    Nodes(Vec<&'a Node<'a, T>>),
}

pub struct Node<'a, T> {
    bounds: BoundingBox<T>,
    parent: Option<&'a Node<'a, T>>,
    children: Children<'a, T>,
}

trait Searchable<T> {
    fn search(&self) -> Option<T>;
}

pub trait TreeState {
    fn depth(&self) -> usize;

    fn height(&self) -> usize;

    fn is_leaf(&self) -> bool;

    fn is_root(&self) -> bool;

    fn num_elements(&self) -> usize;

    fn num_nodes(&self) -> usize;
}

impl<T> TreeState for Node<'_, T> {
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
