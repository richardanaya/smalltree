#![no_std]
use smallgraph::*;
use smallvec::*;

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
        self.graph.connect_to(parent, n);
        n
    }

    pub fn children(&self, parent: NodeHandle) -> SmallVec<[NodeHandle;8]>{
        self.graph.neighbors(parent)
    }

    pub fn remove(&mut self, node: NodeHandle) -> Option<T> {
        if node == self.root {
            panic!("cannot remove root node of tree");
        }
        for c in self.children(node) {
            self.remove(c);
        }
        self.graph.remove(node)
    }

    pub fn get(&self, node: NodeHandle) -> Option<&T> {
        self.graph.get(node)
    }

    pub fn get_mut(&mut self, node: NodeHandle) -> Option<&mut T> {
        self.graph.get_mut(node)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[derive(Debug,PartialEq)]
    struct Foo{
        v:u8
    }

    #[test]
    fn test_basic_0() {
        let mut g = SmallTree::<Foo>::new(Foo{v:44});
        let c1 = g.attach(g.root,Foo{v:55});
        let c2 = g.attach(g.root,Foo{v:66});
        assert_eq!(44,g.get(g.root).unwrap().v);
        assert_eq!(55,g.get(c1).unwrap().v);
        assert_eq!(66,g.get(c2).unwrap().v);
        assert_eq!(2,g.children(g.root).len());
        assert_eq!(55,g.get(g.children(g.root)[0]).unwrap().v);
        assert_eq!(66,g.get(g.children(g.root)[1]).unwrap().v);
        g.get_mut(c2).unwrap().v = 77;
        assert_eq!(77,g.get(g.children(g.root)[1]).unwrap().v);
    }

    #[test]
    fn test_basic_1() {
        let mut g = SmallTree::<Foo>::new(Foo{v:44});
        let c1 = g.attach(g.root,Foo{v:55});
        g.attach(c1,Foo{v:66});
        assert_eq!(3,g.graph.node_count());
        g.remove(c1);
        assert_eq!(1,g.graph.node_count());
    }
}
