use std::cell::RefCell;
use std::rc::{Rc, Weak};

// lowest flag bit is for color
const COLOR_MASK: u64 = 0x1u64;
// second lowest flag bit is for direction
const DIR_MASK: u64 = 0x10;

const RED: u64 = 0b1;
const BLACK: u64 = 0b0;
const LEFT: u64 = 0b0;
const RIGHT: u64 = 0b10;

// Upper 32 bits are for length
const DEPTH_MASK: u64 = 0xff_ff_ff_ff_00000000;

pub struct RBNode<T>
where
    T: PartialEq + PartialOrd,
{
    element: Option<T>,
    parent: Option<Weak<RefCell<RBNode<T>>>>,
    left: Option<Rc<RefCell<RBNode<T>>>>,
    right: Option<Rc<RefCell<RBNode<T>>>>,
    flags: u64,
}

impl<T: PartialEq + PartialOrd> RBNode<T> {
    fn new(flags: u64) -> Self {
        Self {
            element: None,
            parent: None,
            left: None,
            right: None,
            flags,
        }
    }
}

pub struct RBTree<T>
where
    T: PartialEq + PartialOrd,
{
    root: Rc<RefCell<RBNode<T>>>,
    flags: u64,
}

impl<T: PartialEq + PartialOrd> RBTree<T> {
    pub fn new() -> Self {
        Self {
            root: Rc::new(RefCell::new(RBNode::new(BLACK))),
            flags: 0,
        }
    }

    fn rotate(&mut self, sub: Rc<RefCell<RBNode<T>>>, dir: u64) -> Option<Rc<RefCell<RBNode<T>>>> {
        let sub_parent = sub.borrow().parent.clone();
        let new_root = match dir & DIR_MASK {
            LEFT => sub.borrow().right.clone(),
            RIGHT => sub.borrow().left.clone(),
            _ => unreachable!(),
        }?;
        let new_child = match dir & DIR_MASK {
            LEFT => new_root.borrow().left.clone(),
            RIGHT => new_root.borrow().right.clone(),
            _ => unreachable!(),
        };

        match dir & DIR_MASK {
            LEFT => sub.borrow_mut().right = new_child.clone(),
            RIGHT => sub.borrow_mut().left = new_child.clone(),
            _ => unreachable!(),
        }

        if let Some(new_child_) = new_child {
            new_child_.borrow_mut().parent = Some(Rc::downgrade(&sub));
        }

        match dir & DIR_MASK {
            LEFT => new_root.borrow_mut().left = Some(sub.clone()),
            RIGHT => new_root.borrow_mut().right = Some(sub.clone()),
            _ => unreachable!(),
        }

        new_root.borrow_mut().parent = sub_parent
            .as_ref()
            .and_then(|parent| parent.upgrade().and_then(|rc| Some(Rc::downgrade(&rc))));

        sub.borrow_mut().parent = Some(Rc::downgrade(&new_root));

        if let Some(sub_parent_) = sub_parent.and_then(|x| x.upgrade()) {
            match dir & DIR_MASK {
                LEFT => sub_parent_.borrow_mut().left = Some(new_root.clone()),
                RIGHT => sub_parent_.borrow_mut().right = Some(new_root.clone()),
                _ => unreachable!(),
            };
        } else {
            self.root = new_root.clone();
        }

        Some(new_root)
    }
}
