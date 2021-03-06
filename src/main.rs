use std::thread;
use std::time::Duration;

use console::Term;

fn print_loop(ss: &[&str], interval: u64) -> ! {
    let term = Term::stdout();
    loop {
        ss.iter().for_each(|s| {
            term.write_line(s).unwrap();
            thread::sleep(Duration::from_millis(interval));
            term.move_cursor_up(1).unwrap();
            term.clear_line().unwrap();
        });
    }
}

fn main() {
    let ss = [" (」・ω・)」うー！", " (／・ω・)／にゃー！"];
    print_loop(&ss, 1500)
}
