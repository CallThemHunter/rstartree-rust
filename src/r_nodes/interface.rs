use crate::bounding_box::interface::BoundingBox;
use crate::r_nodes::interface::Children::{Boxes, Nodes};
use crate::r_nodes::interface::Parent::{NodeInst, Tree};

pub enum Children<D, R> {
    Boxes(Vec<BoundingBox<D>>),
    Nodes(Vec<Node<D, R>>),
}


pub enum Parent<D, R> {
    Tree(R),
    NodeInst(Box<Node<D, R>>),
}


pub struct Node<D, R> {
    pub bounds: BoundingBox<D>,
    pub parent: Parent<D, R>,
    pub children: Children<D, R>,
}


pub trait NodeManipulation<D>: NodeState<D> {
    fn insert(&mut self, element: BoundingBox<D>);

    fn remove(&mut self, element: BoundingBox<D>) -> bool;

    fn query(&self, element: BoundingBox<D>) -> bool;
}


pub trait NodeTraversal<D, R> {
    fn root(&self) -> &Node<D, R>;

    fn root_mut(&mut self) -> &mut Node<D, R>;
}


impl<D, R> NodeTraversal<D, R> for Node<D, R> {
    fn root(&self) -> &Node<D, R> {
        let mut parent = &self.parent;
        loop {
            match parent {
                Tree(_) => { return self }
                NodeInst(node) => { parent = &node.parent }
            }
        }
    }

    fn root_mut(&mut self) -> &mut Node<D, R> {
        let mut parent = &mut self.parent;
        loop {
            match parent {
                Tree(_) => { return self }
                NodeInst(node) => { parent = &mut node.parent }
            }
        }
    }
}


pub trait NodeState<D> {
    fn depth(&self) -> usize;

    fn height(&self) -> usize;

    fn is_leaf(&self) -> bool;

    fn is_root(&self) -> bool;

    fn num_elements(&self) -> usize;

    fn num_nodes(&self) -> usize;

    fn update_bounds(&self, element: &BoundingBox<D>);
}


impl<D, R> NodeState<D> for Node<D, R> {
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

    fn update_bounds(&self, element: &BoundingBox<D>) {
        todo!()
    }
}
