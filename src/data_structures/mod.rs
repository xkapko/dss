pub mod bsptree;
pub mod gap_buffer;
pub mod linked_list;
pub mod rbtree;
pub mod skip_list;

pub trait NodeDebug {
    fn map_preorder<F: Fn(&Self)>(&self, func: F);
    fn map_inorder<F: Fn(&Self)>(&self, func: F);
    fn map_postorder<F: Fn(&Self)>(&self, func: F);
}
