use std::thread;
use std::time::Duration;

use console::Term;

fn main() {
    let vs = [" (」・ω・)」うー！", " (／・ω・)／にゃー！"].to_vec();
    let term = Term::stdout();
    loop {
        vs.iter().for_each(|s| {
            term.write_line(s).unwrap();
            thread::sleep(Duration::from_millis(1500));
            term.move_cursor_up(1).unwrap();
            term.clear_line().unwrap();
        });
    }
}
