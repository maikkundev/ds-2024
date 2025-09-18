use std::ffi::{c_char, c_float, c_int, c_void};

#[repr(C)]
struct Node {
    am: [c_char; 10],
    on: [c_char; 50],
    ep: [c_char; 50],
    b: c_float,
    ap: c_int,
    left: *const Node,
    right: *const Node,
}

// #[link(name = "tree")]
extern "C" {
    fn createNode() -> *const Node;
    fn addInOrder(root: *const Node, newNode: *const Node) -> c_void;
    fn showInOrder(root: *const Node) -> c_void;
    fn countNodes(root: *const Node) -> c_int;
    fn findByAM(root: *const Node, e: *const c_char) -> *const Node;
    fn maxAM(root: *const Node) -> *const Node;
    fn maxGrade(root: *const Node) -> c_float;
}

fn main() {
    unsafe {
        createNode();
    }
}
