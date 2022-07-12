use std::cell::{Ref, RefMut};

use crate::bounding_box::interface::BoundingBox;
use crate::r_nodes::interface::{Children, Node, NodeCore, NodeLink, NodeManipulation, Parent};

#[derive(Debug, Clone)]
struct RRSTree<D> {
    root: Node<D, Self>,
    config: RRSTreeConfig,
}


#[derive(Debug, Clone)]
struct RRSTreeConfig {
    min_elems: f64,
    max_elems: f64,
    shape_param: f64,
}


impl<D> NodeCore<D, RRSTree<D>> for RRSTree<D> {
    fn depth(&self) -> usize {
        0
    }

    fn height(&self) -> usize {
        self.root.height()
    }

    fn is_leaf(&self) -> bool {
        self.root.is_leaf()
    }

    fn is_root(&self) -> bool {
        true
    }

    fn num_elements(&self) -> usize {
        self.root.num_elements()
    }

    fn num_nodes(&self) -> usize {
        self.root.num_nodes()
    }

    fn update_bounds(&self, element: &BoundingBox<D>) {
        return
    }

    fn remake(self) -> Node<D, Self> {
        todo!()
    }

    fn parent(&self) -> Ref<Parent<D, RRSTree<D>>> {
        todo!()
    }

    fn parent_mut(&self) -> RefMut<Parent<D, RRSTree<D>>> {
        todo!()
    }

    fn children(&self) -> Ref<Children<D, RRSTree<D>>> {
        self.root.children()
    }

    fn children_mut(&self) -> RefMut<Children<D, RRSTree<D>>> {
        self.root.children_mut()
    }

    fn root(&self) -> &Node<D, Self> {
        &self.root
    }

    fn root_mut(&mut self) -> &mut Node<D, Self> {
        &mut self.root
    }
}


impl<D> NodeManipulation<D, RRSTree<D>> for RRSTree<D> {
    fn insert(&self, element: BoundingBox<D>) {
        self.root.insert(element)
    }

    fn remove(&mut self, element: BoundingBox<D>) -> bool {
        self.root.remove(element)
    }

    fn query(&self, element: BoundingBox<D>) -> bool {
        self.root.query(element)
    }
}