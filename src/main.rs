use std::env;
use std::thread;
use std::time::Duration;

use enigo::*;

fn main() {
    thread::sleep(Duration::from_secs(3));
    let character_count: i32 = handle_input_num(env::args().collect());
    get_free_time();
    handle_materials_instance();
    loop_character(character_count);
}

fn handle_input_num(args: Vec<String>) -> i32 {
    let input_number: i32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("请输入一个有效的数字。");
            return MY_CHARACTER_COUNT;
        }
    };
    return input_number;
}

fn handle_materials_instance() {
    click('m');
    click('g');
    thread::sleep(Duration::from_secs(2));
    click('p');
    click('m');
    click('r');
    click('o');
    click('y');
    click_function_key(6);
    click('t');
    click_esc();
    click('e');
    click_function_key(4);
    click('x');
    click('f');
    click('f');
    click('f');
    click_function_key(7);
    click('h');
    thread::sleep(Duration::from_secs(285));
    click('b');
    thread::sleep(Duration::from_secs(4));
    click('m');
    click('x');
    click('c');
    thread::sleep(Duration::from_secs(5));
}

fn clear_ad() {
    for _ in 0..=2 {
        click_esc();
    }
}

fn get_free_time() {
    click('v');
    click('n');
    click('z');
}

fn switch_character(character: i32) {
    click_function_key(character);
    click('x');
    clear_ad();
    get_free_time();
    handle_materials_instance();
}

fn loop_character(max_loops: i32) {
    let start_first_loop = 3;
    let start_other_loops = 1;
    let end = 7;
    let cycle_length = end - start_other_loops + 1;
    let mut previous_character_num = start_first_loop;

    for num in 1..=max_loops {
        let character_num = if num <= end - start_first_loop + 1 {
            num + start_first_loop - 1
        } else {
            (num - end + start_first_loop - 2) % cycle_length + start_other_loops
        };

        if previous_character_num == end && character_num == start_other_loops {
            click('q');
        }

        switch_character(character_num);
        previous_character_num = character_num;
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

fn click_function_key(character: i32) {
    let mut enigo = Enigo::new();
    thread::sleep(Duration::from_secs(1));
    match character {
        1 => enigo.key_click(Key::F1),
        2 => enigo.key_click(Key::F2),
        3 => enigo.key_click(Key::F3),
        4 => enigo.key_click(Key::F4),
        5 => enigo.key_click(Key::F5),
        6 => enigo.key_click(Key::F6),
        7 => enigo.key_click(Key::F7),
        _ => (),
    }
}

const MY_CHARACTER_COUNT: i32 = 10;
