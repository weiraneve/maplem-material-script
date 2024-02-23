use std::thread;
use std::time::Duration;

use enigo::*;

fn main() {
    thread::sleep(Duration::from_secs(3));

    let mut enigo = Enigo::new();
    enigo.key_down(Key::Control);
    enigo.key_click(Key::Layout('m'));
    enigo.key_up(Key::Control);

    thread::sleep(Duration::from_secs(1));
    enigo.key_down(Key::Control);
    enigo.key_click(Key::Layout('g'));
    enigo.key_up(Key::Control);

    thread::sleep(Duration::from_secs(1));
    enigo.key_down(Key::Control);
    enigo.key_click(Key::Layout('p'));
    enigo.key_up(Key::Control);

    thread::sleep(Duration::from_secs(1));
    enigo.key_down(Key::Control);
    enigo.key_click(Key::Layout('m'));
    enigo.key_up(Key::Control);
}
