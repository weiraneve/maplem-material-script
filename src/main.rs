use std::thread;
use std::time::Duration;

use enigo::*;

fn main() {
    thread::sleep(Duration::from_secs(2));

    let mut enigo = Enigo::new();
    enigo.key_click(Key::Layout('v'));
}
