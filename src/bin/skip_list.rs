extern crate dss;

use dss::data_structures::skip_list::*;

fn main() {
    let mut sl: SkipList<i32> = SkipList::new(4);

    (0..10).for_each(|x| sl.insert(x));
}
