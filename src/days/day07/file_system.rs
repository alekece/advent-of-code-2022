use std::cell::RefCell;
use std::rc::Rc;

use enum_dispatch::enum_dispatch;
use num_bigint::BigUint;

use super::command::Command;
use crate::solver::Result;

#[derive(Debug)]
pub struct Context {
    root_node: Rc<RefCell<Node>>,
    node_pointer: Vec<Rc<RefCell<Node>>>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            root_node: Rc::new(RefCell::new(Node::new_root_directory())),
            node_pointer: Vec::default(),
        }
    }
}

impl Context {
    pub fn reset_to_root(&mut self) {
        self.node_pointer = vec![Rc::clone(&self.root_node)];
    }

    pub fn move_back(&mut self) -> Option<NodeHandle> {
        self.node_pointer.pop().as_ref().map(NodeHandle::new)
    }

    pub fn go_to(&mut self, node: NodeHandle) {
        self.node_pointer.push(node.into_inner())
    }

    pub fn working_directory(&self) -> Option<NodeHandle> {
        self.node_pointer.last().map(NodeHandle::new)
    }

    pub fn root(&self) -> NodeHandle {
        NodeHandle::new(&self.root_node)
    }
    pub fn update(&mut self, commands: &[Box<dyn Command>]) -> Result<()> {
        for command in commands.iter() {
            command.execute(self)?;
        }

        Ok(())
    }

    pub fn browse_from_root(&self, predicate: impl Fn(&NodeHandle) -> bool) -> Vec<NodeHandle> {
        [self.root()]
            .into_iter()
            .chain(self.root_node.borrow().successors())
            .into_iter()
            .filter(predicate)
            .collect()
    }
}

#[enum_dispatch(Node)]
pub trait NodeLike {
    fn name(&self) -> String;
    fn add_child(&mut self, node: Node);
    fn children(&self) -> Vec<NodeHandle>;
    fn successors(&self) -> Vec<NodeHandle>;
    fn size(&self) -> BigUint;
    fn is_file(&self) -> bool;
    fn is_directory(&self) -> bool;
}

#[derive(Debug, Clone)]
pub struct NodeHandle {
    inner: Rc<RefCell<Node>>,
}

impl NodeHandle {
    pub fn new(node: &Rc<RefCell<Node>>) -> Self {
        Self { inner: Rc::clone(node) }
    }

    pub fn into_inner(self) -> Rc<RefCell<Node>> {
        self.inner
    }
}

impl NodeLike for NodeHandle {
    fn name(&self) -> String {
        self.inner.borrow().name()
    }

    fn add_child(&mut self, node: Node) {
        self.inner.borrow_mut().add_child(node)
    }

    fn children(&self) -> Vec<NodeHandle> {
        self.inner.borrow().children()
    }

    fn successors(&self) -> Vec<NodeHandle> {
        self.inner.borrow().successors()
    }

    fn size(&self) -> BigUint {
        self.inner.borrow().size()
    }

    fn is_directory(&self) -> bool {
        self.inner.borrow().is_directory()
    }

    fn is_file(&self) -> bool {
        self.inner.borrow().is_file()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[enum_dispatch]
pub enum Node {
    File(FileNode),
    Directory(DirectoryNode),
}

impl Node {
    pub fn new_root_directory() -> Self {
        Self::Directory(DirectoryNode::new("/".to_string()))
    }

    pub fn new_directory(name: String) -> Self {
        Self::Directory(DirectoryNode::new(name))
    }

    pub fn new_file(name: String, size: BigUint) -> Self {
        Self::File(FileNode::new(name, size))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileNode {
    name: String,
    size: BigUint,
}

impl FileNode {
    pub fn new(name: String, size: BigUint) -> Self {
        Self { name, size }
    }
}

impl NodeLike for FileNode {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn add_child(&mut self, _node: Node) {}

    fn children(&self) -> Vec<NodeHandle> {
        Vec::default()
    }

    fn successors(&self) -> Vec<NodeHandle> {
        Vec::default()
    }

    fn size(&self) -> BigUint {
        self.size.clone()
    }

    fn is_file(&self) -> bool {
        true
    }

    fn is_directory(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectoryNode {
    name: String,
    children: Vec<Rc<RefCell<Node>>>,
}

impl DirectoryNode {
    pub fn new(name: String) -> Self {
        Self {
            name,
            children: Vec::default(),
        }
    }
}

impl NodeLike for DirectoryNode {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn add_child(&mut self, node: Node) {
        self.children.push(Rc::new(RefCell::new(node)));
    }

    fn children(&self) -> Vec<NodeHandle> {
        self.children.iter().map(NodeHandle::new).collect()
    }

    fn successors(&self) -> Vec<NodeHandle> {
        self.children()
            .into_iter()
            .flat_map(|node| {
                let successors = node.successors();

                [node].into_iter().chain(successors)
            })
            .collect()
    }

    fn size(&self) -> BigUint {
        self.children.iter().map(|node| node.borrow().size()).sum()
    }

    fn is_file(&self) -> bool {
        false
    }

    fn is_directory(&self) -> bool {
        true
    }
}
