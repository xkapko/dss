extern crate dss;

use dss::data_structures::bsptree::{BSPTree, MoveDirection, Rectangle};
use raylib::color::Color;
use raylib::prelude::{RaylibDraw, RaylibDrawHandle, Rectangle as RRect};

fn convert(rect: Rectangle) -> RRect {
    RRect::new(rect.x as f32, rect.y as f32, rect.w as f32, rect.h as f32)
}

fn draw_tree(tree: &BSPTree, mut c: RaylibDrawHandle) -> Result<(), Box<dyn std::error::Error>> {
    for node in tree.walk() {
        let n = node.borrow();
        let rect = n.get_rect();
        if n.get_data().is_some() {
            let rect = convert(rect);
            if n.is_focused() {
                c.draw_rectangle_rec(rect, Color::GREEN);
                c.draw_rectangle_lines_ex(rect, 3., Color::BLACK);
            } else {
                c.draw_rectangle_rec(rect, Color::RED);
                c.draw_rectangle_lines_ex(rect, 3., Color::BLACK);
            }
        } else {
            let rect = convert(rect);
            c.draw_rectangle_rec(rect, Color::BLUE);
            c.draw_rectangle_lines_ex(rect, 3.0, Color::BLACK);
        }
    }
    println!("===");
    tree.print(1);
    println!("===");
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (w, h): (i32, i32) = (640, 480);
    let mut tree = BSPTree::new(Rectangle::new(0, 0, w as u32, h as u32));

    let (mut rl, thread) = raylib::init().size(w, h).title("BSPTree demo").build();
    let mut count = 0;

    while !rl.window_should_close() {
        if let Some(key) = rl.get_key_pressed() {
            match key {
                // left
                raylib::ffi::KeyboardKey::KEY_H => tree.move_focus(MoveDirection::Left),
                // down
                raylib::ffi::KeyboardKey::KEY_J => tree.move_focus(MoveDirection::Down),
                // up
                raylib::ffi::KeyboardKey::KEY_K => tree.move_focus(MoveDirection::Up),
                // right
                raylib::ffi::KeyboardKey::KEY_L => tree.move_focus(MoveDirection::Right),
                // delete
                raylib::ffi::KeyboardKey::KEY_D => tree.delete_focused(),
                // toggle layout
                raylib::ffi::KeyboardKey::KEY_S => tree.toggle_split(),
                // new client
                raylib::ffi::KeyboardKey::KEY_N => {
                    count += 1;
                    tree.insert(count)
                }
                _ => {}
            }
        }
        let pos = rl.get_mouse_position();
        tree.focus_coords(pos.x as i32, pos.y as i32);
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        draw_tree(&tree, d)?;
    }

    Ok(())
}
