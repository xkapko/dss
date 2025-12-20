#![allow(unused)]
use rand::random_range;
use std::boxed::Box;
use std::fmt::{Display, Write};
use std::ptr::NonNull;

#[derive(Debug, PartialEq, PartialOrd, Eq)]
struct SkipNode<T: Eq> {
    data: SkipData<T>,
    next: Option<NonNull<SkipNode<T>>>,
    prev: Option<NonNull<SkipNode<T>>>,
}

impl<T: Eq> SkipNode<T> {
    fn new(item: T) -> Self {
        Self {
            data: SkipData::Final(item),
            next: None,
            prev: None,
        }
    }

    fn new_ptr(item: NonNull<SkipNode<T>>) -> Self {
        Self {
            data: SkipData::Ptr(item),
            next: None,
            prev: None,
        }
    }
}

impl<T: Eq + Display> SkipNode<T> {
    fn show(&self) -> String {
        let mut buff = String::new();

        write!(
            buff,
            "node: {} {}",
            if matches!(self.data, SkipData::Final(_)) {
                "has data"
            } else {
                "points to data"
            },
            self.data.data(),
        );

        buff
    }
}

#[derive(Debug, Eq)]
enum SkipData<T: Eq> {
    Final(T),
    Ptr(NonNull<SkipNode<T>>),
}

impl<T: Eq> SkipData<T> {
    fn data(&self) -> &T {
        match self {
            SkipData::Final(t) => t,
            SkipData::Ptr(non_null) => {
                let mut h = unsafe { non_null.as_ref() };
                while let SkipData::Ptr(ptr) = h.data {
                    h = unsafe { ptr.as_ref() };
                }
                h.data.data()
            }
        }
    }
}

impl<T: Eq> PartialEq for SkipData<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Final(l0), Self::Final(r0)) => l0 == r0,
            (Self::Ptr(l0), Self::Ptr(r0)) => {
                let mut h1 = unsafe { l0.as_ref() };
                while let Self::Ptr(f) = h1.data {
                    h1 = unsafe { f.as_ref() };
                }
                let mut h2 = unsafe { r0.as_ref() };
                while let Self::Ptr(f) = h2.data {
                    h2 = unsafe { f.as_ref() };
                }
                h1.eq(h2)
            }
            _ => false,
        }
    }
}

impl<T: Eq + PartialOrd> PartialOrd for SkipData<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Final(l0), Self::Final(r0)) => std::cmp::PartialOrd::partial_cmp(l0, r0),
            (Self::Ptr(l0), Self::Ptr(r0)) => {
                let mut h1 = unsafe { l0.as_ref() };
                while let Self::Ptr(f) = h1.data {
                    h1 = unsafe { f.as_ref() };
                }
                let mut h2 = unsafe { r0.as_ref() };
                while let Self::Ptr(f) = h2.data {
                    h2 = unsafe { f.as_ref() };
                }
                h1.partial_cmp(h2)
            }
            _ => None,
        }
    }
}

#[derive(Debug)]
struct SkipLevel<T: Eq> {
    head: Option<NonNull<SkipNode<T>>>,
    len: usize,
    prob: f64,
}

impl<T: Eq> SkipLevel<T> {
    pub fn new(prob: f64) -> Self {
        Self {
            head: None,
            len: 0,
            prob,
        }
    }

    pub fn insert(&mut self, mut node: NonNull<SkipNode<T>>) {
        if self.head.is_none() {
            self.head = Some(node);
            self.len += 1;
        } else {
            let mut curr = self.head;
            let mut prev: Option<NonNull<SkipNode<T>>> = None;
            while let Some(mut n) = curr {
                if n >= node {
                    break;
                }
                prev = Some(n);
                curr = unsafe { n.as_ref().next };
            }
            // case 1: able to insert into the middle of the list
            if let (Some(mut n), Some(mut p)) = (curr, prev) {
                // modify new node
                {
                    let mut node = unsafe { node.as_mut() };
                    node.next = Some(n);
                    node.prev = Some(p);
                }
                // modify prev and curr
                {
                    {
                        let mut h = unsafe { p.as_mut() };
                        h.next = Some(node);
                    }
                    {
                        let mut h = unsafe { n.as_mut() };
                        h.prev = Some(node);
                    }
                }

                self.len += 1;
                // case 2: node becomes the new head
            } else if let Some(mut n) = curr {
                {
                    let mut h = unsafe { n.as_mut() };
                    h.prev = Some(node);
                }
                {
                    let mut h = unsafe { node.as_mut() };
                    h.next = Some(n);
                }
                self.head = Some(n);
                self.len += 1;
                // case 3: node becomes the new tail
            } else if let Some(mut p) = prev {
                {
                    let mut h = unsafe { p.as_mut() };
                    h.next = Some(node)
                }
                {
                    let mut h = unsafe { node.as_mut() };
                    h.prev = Some(p);
                }
                self.len += 1;
            }
        }
    }
}

impl<T: Eq + Display> SkipLevel<T> {
    pub fn show(&self) -> String {
        let mut buff = String::new();

        let mut curr = self.head;
        while let Some(n) = curr {
            let h = unsafe { n.as_ref() };
            write!(buff, "{}; ", h.show());
            curr = h.next;
        }
        write!(buff, "\n");

        buff
    }
}

impl<T: Eq> Drop for SkipLevel<T> {
    fn drop(&mut self) {
        let mut curr = self.head;
        while let Some(n) = curr {
            {
                let h = unsafe { n.as_ref() };
                curr = h.next;
            }
            let b = unsafe { Box::from_non_null(n) };
            drop(b);
        }
    }
}

#[derive(Debug)]
pub struct SkipList<T>
where
    T: Eq,
{
    levels: Vec<SkipLevel<T>>,
}

impl<T: Eq> SkipList<T> {
    pub fn new(levels: usize) -> Self {
        Self {
            levels: (0..levels)
                .map(|n| SkipLevel::new((1_f64 / 2_f64.powi(n as i32))))
                .collect(),
        }
    }

    pub fn insert(&mut self, item: T) {
        let node = Box::into_non_null(Box::new(SkipNode::new(item)));
        if self.levels[0].len == 0 {
            let mut prev_ptr: NonNull<SkipNode<T>>;
            self.levels[0].insert(node);
            prev_ptr = node;
            for (level) in self.levels.iter_mut() {
                let node = Box::into_non_null(Box::new(SkipNode::new_ptr(prev_ptr)));
                prev_ptr = node;
                level.insert(node);
            }
        }
    }

    pub fn levels(&self) -> usize {
        self.levels.len()
    }
}

impl<T: Eq + Display> SkipList<T> {
    pub fn show(&self) -> Vec<String> {
        self.levels.iter().map(SkipLevel::show).collect()
    }
}
