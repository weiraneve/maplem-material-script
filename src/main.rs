use std::thread;
use std::time::Duration;

use enigo::*;

fn main() {
    handle_materials_instance();
    loop_first();
    loop_second();
}

fn handle_materials_instance() {
    click('m');
    click('g');
    click('p');
    click('m');
    click('r');
    click('o');
    click('y');
    click('6');
    click('t');
    click_esc();
    click('e');
    click('4');
    click('x');
    click('f');
    click('f');
    click('f');
    click('f');
    click('7');
    click('h');
    click('m');
    click('x');
    click('c');
}

fn clear_ad() {
    click_esc();
    click_esc();
    click_esc();
}

fn switch_character(character: char) {
    click(character);
    click('x');
    clear_ad();
    handle_materials_instance();
}

fn loop_first() {
    for i in 3..7 {
        switch_character((i as u8) as char);
    }
}

fn loop_second() {
    for i in 1..3 {
        switch_character((i as u8) as char);
    }
}

fn click(letter: char) {
    let mut enigo = Enigo::new();
    thread::sleep(Duration::from_secs(1));
    enigo.key_down(Key::Control);
    enigo.key_click(Key::Layout(letter));
    enigo.key_up(Key::Control);
}

fn click_esc() {
    let mut enigo = Enigo::new();
    thread::sleep(Duration::from_secs(1));
    enigo.key_click(Key::Escape);
}
