use std::borrow::Borrow;
use std::cell::{Ref, RefCell};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use crate::bounding_box::interface::BoundingBox;
use crate::r_nodes::interface::Children::{Boxes, Nodes};
use crate::r_nodes::interface::Parent::{NodeInst, Tree};

#[derive(Debug, Clone)]
pub enum Children<D, R> {
    Boxes(Vec<BoundingBox<D>>),
    Nodes(Vec<Node<D, R>>),
}


#[derive(Debug, Clone)]
pub enum Parent<D, R> {
    Tree(R),
    NodeInst(Node<D, R>),
}


pub type NodeLink<T> = Rc<RefCell<T>>;


#[derive(Debug, Clone)]
pub struct Node<D, R> {
    pub bounds: BoundingBox<D>,
    pub parent: NodeLink<Parent<D, R>>,
    pub children: NodeLink<Children<D, R>>,
}


pub trait NodeManipulation<D, R>: NodeCore<D, R> {
    fn insert(&self, element: BoundingBox<D>);

    fn remove(&mut self, element: BoundingBox<D>) -> bool;

    fn query(&self, element: BoundingBox<D>) -> bool;
}


pub trait NodeCore<D, R> {
    fn depth(&self) -> usize;

    fn height(&self) -> usize;

    fn is_leaf(&self) -> bool;

    fn is_root(&self) -> bool;

    fn num_elements(&self) -> usize;

    fn num_nodes(&self) -> usize;

    fn update_bounds(&self, element: &BoundingBox<D>);

    fn remake(self) -> Node<D, R>;

    fn parent(&self) -> Ref<Parent<D, R>>;

    fn parent_mut(&self) -> RefMut<Parent<D, R>>;

    fn children(&self) -> Ref<Children<D, R>>;

    fn children_mut(&self) -> RefMut<Children<D, R>>;

    fn root_mut(&mut self) -> &mut Node<D, R>;
}


impl<D, R> NodeCore<D, R> for Node<D, R> {
    fn depth(&self) -> usize {
        let cell: &RefCell<Parent<D, R>> = Borrow::borrow(&self.parent);

        match RefCell::borrow(cell).deref() {
            Tree(_) => 0,
            NodeInst(n) => 1 + n.depth()
        }
    }

    fn height(&self) -> usize {
        // r trees are always balanced!
        // so you can use the first node on a list always

        let borrowed: &RefCell<Children<D, R>> = self.children.borrow();
        match RefCell::borrow(borrowed).deref() {
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
        let borrowed: &RefCell<Children<D, R>> = Borrow::borrow(&self.children);
        match RefCell::borrow(borrowed).deref() {
            Nodes(_) => false,
            Boxes(_) => true
        }
    }

    fn is_root(&self) -> bool {
        let borrowed: &RefCell<Parent<D, R>> = self.parent.borrow();
        match RefCell::borrow(borrowed).deref() {
            Tree(_) => true,
            NodeInst(_) => false,
        }
    }

    fn num_elements(&self) -> usize {
        let borrowed: &RefCell<Children<D, R>> = self.children.borrow();
        match RefCell::borrow(borrowed).deref() {
            Nodes(nodes)
            => nodes
                .iter()
                .map(|node| { node.num_elements() })
                .sum(),
            Boxes(boxes) => boxes.len()
        }
    }

    fn num_nodes(&self) -> usize {
        let borrowed: &RefCell<Children<D, R>> = self.children.borrow();
        match RefCell::borrow(borrowed).deref() {
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

    fn remake(self) -> Node<D, R> {
        let bounds = self.bounds;
        let parent = self.parent;
        let children = self.children;

        Node { bounds, parent, children }
    }

    fn parent(&self) -> Ref<Parent<D, R>> {
        let cell: &RefCell<Parent<D, R>> = self.parent.borrow();
        cell.deref().borrow()
    }

    fn parent_mut(&self) -> RefMut<Parent<D, R>> {
        self.parent.borrow_mut()
    }

    fn children(&self) -> Ref<Children<D, R>> {
        let cell: &RefCell<Children<D, R>> = self.children.borrow();
        cell.deref().borrow()
    }

    fn children_mut(&self) -> RefMut<Children<D, R>> {
        self.children.borrow_mut()
    }

    fn root(&self) -> Ref<Node<D, R>> {
        let mut parent = self.parent().deref();
        loop {
            match parent.deref() {
                Tree(_) => { return self }
                NodeInst(node) => {
                    let parent_cell: &RefCell<Parent<D, R>> = Rc::borrow(&node.parent);
                    parent = RefCell::borrow(parent_cell);
                }
            }
        }
    }

    fn root_mut(&mut self) -> &mut Node<D, R> {
        let mut parent = &mut self.parent.clone().into_inner();
        loop {
            match parent.deref_mut() {
                Tree(_) => { return self }
                NodeInst(node) => { parent = &mut node.parent.clone().into_inner() }
            }
        }
    }
}
