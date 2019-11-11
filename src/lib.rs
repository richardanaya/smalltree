#![no_std]
use smallgraph::*;

pub struct SmallTree<T> {
    pub graph: SmallGraph<T>,
    pub root: NodeHandle,
}

impl<T> SmallTree<T> {
    pub fn new(root: T) -> SmallTree<T> {
        let mut g = SmallGraph::<T>::new();
        let root = g.insert(root);
        SmallTree {
            graph: g,
            root: root,
        }
    }

    pub fn attach(&mut self, parent: NodeHandle, child: T) -> NodeHandle {
        let n = self.graph.insert(child);
        self.graph.directed_connect(parent, n);
        n
    }

    pub fn remove(&mut self, node: NodeHandle) -> Result<T, GraphError> {
        self.graph.remove(node)
    }

    pub fn get(&self, node: NodeHandle) -> Result<&T, GraphError> {
        self.graph.get(node)
    }

    pub fn get_mut(&mut self, node: NodeHandle) -> Result<&mut T, GraphError> {
        self.graph.get_mut(node)
    }
}
