#![cfg(test)]

use super::super::data_structures::skip_list::*;

#[test]
fn skiplist_probs() {
    let sl: SkipList<u32> = SkipList::new(4);

    dbg!(sl);
}

#[test]
fn skiplist_insert() {
    let mut sl: SkipList<u32> = SkipList::new(4);

    sl.insert(1);
    sl.insert(2);
    sl.insert(3);
    sl.insert(4);
    sl.insert(5);

    for (i, level) in sl.show().iter().enumerate() {
        print!("level {i}: {level}");
    }
}
