use crate::bounding_box::interface::BoundingBox;
use crate::r_nodes::interface::Children::{Boxes, Nodes};
use crate::r_nodes::interface::Parent::{NodeInst, Tree};

pub enum Children<'a, D, R> {
    Boxes(Vec<BoundingBox<D>>),
    Nodes(Vec<&'a Node<'a, D, R>>),
}


pub enum Parent<'a, D, R> {
    Tree(R),
    NodeInst(Box<Node<'a, D, R>>),
}


pub struct Node<'a, D, R> {
    pub bounds: BoundingBox<D>,
    pub parent: Parent<'a, D, R>,
    pub children: Children<'a, D, R>,
}


pub trait NodeManipulation<'a, D>: NodeState {
    fn insert(&'a mut self, element: BoundingBox<D>);

    fn remove(&mut self, element: BoundingBox<D>) -> bool;

    fn query(&self, element: BoundingBox<D>) -> bool;
}


pub trait NodeTraversal<'a, D, R> {
    fn root(&self) -> &Node<D, R>;

    fn root_mut<'b: 'a>(&'b mut self) -> &mut Node<'b, D, R>;
}


impl<'node, D, R> NodeTraversal<'node, D, R> for Node<'node, D, R> {
    fn root(&self) -> &Node<D, R> {
        match &self.parent {
            Tree(_) => self,
            NodeInst(node) => node.root()
        }
    }

    fn root_mut<'out: 'node>(&'out mut self) -> &mut Node<'out, D, R> {
        match &mut self.parent {
            Tree(_) => self,
            NodeInst(node) => node.root_mut()
        }
    }
}


pub trait NodeState {
    fn depth(&self) -> usize;

    fn height(&self) -> usize;

    fn is_leaf(&self) -> bool;

    fn is_root(&self) -> bool;

    fn num_elements(&self) -> usize;

    fn num_nodes(&self) -> usize;
}


impl<'a, D, R> NodeState for Node<'a, D, R> {
    fn depth(&self) -> usize {
        match &self.parent {
            Tree(_) => 0,
            NodeInst(n) => 1 + n.depth()
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
        match &self.parent {
            Tree(_) => true,
            NodeInst(_) => false,
        }
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
