use std::process::Child;

use crate::bounding_box::interface::BoundingBox;
use crate::r_nodes::basic::Children::{Boxes, Nodes};

enum Children<'a, T> {
    Boxes(Vec<BoundingBox<T>>),
    Nodes(Vec<&'a Node<'a, T>>),
}

struct Node<'a, T> {
    bounds: BoundingBox<T>,
    parent: Option<&'a Node<'a, T>>,
    children: Children<'a, T>,
}

trait Searchable<T> {
    fn search(&self) -> Option<T>;
}

trait TreeState {
    fn depth(&self) -> usize;

    fn height(&self) -> usize;

    fn is_leaf(&self) -> bool;

    fn is_root(&self) -> bool;
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
            Nodes(n) => 1 + n.first().unwrap().height()
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
}