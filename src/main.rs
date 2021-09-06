use std::thread;
use std::time::Duration;

use console::Term;

fn main() {
    let ss = [" (」・ω・)」うー！", " (／・ω・)／にゃー！"];
    let term = Term::stdout();
    loop {
        ss.iter().for_each(|s| {
            term.write_line(s).unwrap();
            thread::sleep(Duration::from_millis(1500));
            term.move_cursor_up(1).unwrap();
            term.clear_line().unwrap();
        });
    }
}
