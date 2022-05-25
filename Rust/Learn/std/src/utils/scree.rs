pub fn clear_screen() {
    print!("{}[2J", 27 as char);
}

pub fn clear_screen_and_move_cursor() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
