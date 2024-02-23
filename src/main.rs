use std::thread;
use std::time::Duration;

use enigo::*;

fn main() {
    thread::sleep(Duration::from_secs(3));

    click('m');
    click('g');
    click('p');
    click('m');
}

fn click(letter: char) {
    let mut enigo = Enigo::new();
    thread::sleep(Duration::from_secs(1));
    enigo.key_down(Key::Control);
    enigo.key_click(Key::Layout(letter));
    enigo.key_up(Key::Control);
}
