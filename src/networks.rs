use std::cell::UnsafeCell;
use typed_arena::Arena;

pub type NetworkArena<'a> = Arena<Node<'a>>;


pub type NodeRef<'a> = &'a Node<'a>;

#[derive(Copy, Clone)]
pub struct Edge<'a> {
    pub source: NodeRef<'a>,
    pub target: NodeRef<'a>
}


pub type EdgeVec<'a> = Vec<Edge<'a>>;

#[derive(Debug)]
pub struct Node<'a> {
    pub label: &'static str,
    _in_edges: UnsafeCell<EdgeVec<'a>>,
    _out_edges: UnsafeCell<EdgeVec<'a>>
}

impl<'a> Node<'a> {

    pub fn new<'b>(label: &'static str, arena: &'b NetworkArena<'b>) -> &'b Node<'b> {
        arena.alloc(Node{
            label,
            _in_edges: UnsafeCell::new(Vec::new()),
            _out_edges: UnsafeCell::new(Vec::new())
        })
    } 

    pub fn in_edges(&self) -> &EdgeVec<'a> {
        unsafe {
            &(*self._in_edges.get())
        }
    }

    pub fn out_edges(&self) -> &EdgeVec<'a> {
        unsafe {
            &(*self._out_edges.get())
        }
    }

    pub fn in_nodes(&self) -> Vec<NodeRef<'a>> {
        self.in_edges().iter().map(|e| e.source).collect()
    }

    pub fn out_nodes(&self) -> Vec<NodeRef<'a>> {
        self.out_edges().iter().map(|e| e.target).collect()
    }

}

pub fn attach_to<'a>(source: &'a Node<'a>, target: &'a Node<'a>) {
    let edge = Edge{source, target};
    unsafe {
        (*source._out_edges.get()).push(edge);
        (*target._in_edges.get()).push(edge);
    }
}

use std::collections::HashSet;

type NodePtrSet<'a> = HashSet<*const Node<'a>>;
fn _dfs<'a, F>(f: &F, node: &Node<'a>, seen: &mut NodePtrSet<'a>)
    where F: Fn(&Node<'a>)
{
    let node_ptr = node as *const Node;
    if !seen.contains(&node_ptr) {
        f(node);
    } else {
        return
    }

    seen.insert(node_ptr);

    for neighbor in node.out_nodes() {
        _dfs(f, neighbor, seen)
    }
}

pub fn dfs<'a, F>(f: &F, start_node: &Node<'a>)
    where F: Fn(&Node<'a>)
{
    _dfs(f, start_node, &mut NodePtrSet::new());
}
