#![no_std]
use smallvec::SmallVec;

type NodeIndex = usize;
type Generation = usize;
type NodeHandle = (NodeIndex,Generation);

pub enum GraphError {
    GenerationMismatch,
    Unexpected
}

pub struct Graph<T> {
    pub free:SmallVec<[NodeHandle; 128]>,
    pub nodes:SmallVec<[(Generation,Option<T>); 128]>,
    pub connections:SmallVec<[(NodeHandle,NodeHandle); 256]>,
}

impl<T> Graph<T>{
    pub fn insert(&mut self,value:T) -> NodeHandle{
        if self.free.len() == 0 {
            let index = self.nodes.len();
            let gen = 0;
            self.nodes.push((gen,Some(value)));
            (index,gen)
        } else {
            let n = self.free.remove(0);
            let index = n.0;
            let gen = n.1;
            self.nodes[index] = (gen+1,Some(value));
            (n.0,gen+1)
        }
    }

    pub fn directed_connect(&mut self, parent:NodeHandle,child:NodeHandle) {
        self.connections.push((parent,child));
    }

    pub fn connect(&mut self, a:NodeHandle,b:NodeHandle) {
        self.connections.push((a,b));
        self.connections.push((b,a));
    }

    pub fn remove(&mut self,n:NodeHandle) -> Result<T,GraphError>{
        let index = n.0;
        let gen = n.1;
        if  self.nodes[index].0 == gen {
            let mut r = (gen+1,None);
            core::mem::swap(&mut self.nodes[index], &mut r);
            self.free.push(n);
            Ok(r.1.unwrap())
        } else {
            Err(GraphError::GenerationMismatch)
        }
    }

    pub fn get(&self,n:NodeHandle) -> Result<&T,GraphError> {
        let index = n.0;
        let gen = n.1;
        if  self.nodes[index].0 == gen {
            if let Some(value) = &self.nodes[index].1 {
                Ok(value)
            } else {
                Err(GraphError::Unexpected)
            }
        } else {
            Err(GraphError::GenerationMismatch)
        }
    }

    pub fn get_mut(&mut self,n:NodeHandle) -> Result<&mut T,GraphError>{
        let index = n.0;
        let gen = n.1;
        if  self.nodes[index].0 == gen {
            if let Some(value) = &mut self.nodes[index].1 {
                Ok(value)
            } else {
                Err(GraphError::Unexpected)
            }
        } else {
            Err(GraphError::GenerationMismatch)
        }
    }
}