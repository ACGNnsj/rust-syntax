// use std::cell::{Ref, RefCell};
// use std::rc::Rc;
use core::cell::{Ref, RefCell};
extern crate alloc;
use alloc::rc::Rc;
// extern crate panic_abort;

struct Node<'a> {
    v: i32,
    next: Option<&'a Node<'a>>,
}

impl<'a> Node<'a> {
    // 静态的new方法，创建一个next成员为None的节点
    fn new(v: i32) -> Self {
        Self { v, next: None }
    }
    fn set_next(&mut self, next: &'a Node<'a>) {
        self.next = Some(next);
    }
}

// 定义节点的打印格式
impl std::fmt::Debug for Node<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, next:{:?}", self.v, self.next)
    }
}


#[test]
fn test_linked_list() {
    let mut head = Node::new(1);
    let mut second = Node::new(2);
    let mut third = Node::new(3);
    let fourth = Node::new(4);

    // head.set_next(&second);
    // second.set_next(&third);
    // third.set_next(&fourth);

    third.set_next(&fourth);
    second.set_next(&third);
    head.set_next(&second);

    println!("head: {:?}", head);
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[test]
pub fn test_borrow() {
    let mut root = TreeNode::new(1);
    let left = TreeNode::new(2);
    let right = TreeNode::new(3);
    root.left = Some(Rc::new(RefCell::new(left)));
    root.right = Some(Rc::new(RefCell::new(right)));
    println!("root: {:?}", root);
    let l_r_rc = root.left.unwrap();
    let l_r = l_r_rc.borrow();
    let l_r = &**&l_r;
    let i_r = Rc::new(1);
    // let i=i_r.borrow();
    let i = &**&i_r;

    let str_r = Rc::new("hello".to_string());
    println!("{}", str_r.len());

    let str_r_rc = Rc::new(RefCell::new("hello".to_string()));
    println!("{}", str_r_rc.borrow().len());
    
    let ref a = 1;
    let b: &i32 = a;
}
