use std::thread;
use std::time::{Duration, Instant};

use console::Term;
use unicode_width::UnicodeWidthStr;

fn print_loop(ss: &[&str], interval: u64) -> ! {
    let term = Term::stdout();
    let now = Instant::now();
    loop {
        ss.iter().for_each(|s| {
            let (_, x) = term.size();
            let s = build_loop_s(s, now.elapsed().as_secs() as usize, x as usize);
            term.write_line(&s).unwrap();
            thread::sleep(Duration::from_millis(interval));
            term.move_cursor_up(1).unwrap();
            term.clear_line().unwrap();
        });
    }
}

fn build_loop_s(s: &str, head: usize, max: usize) -> String {
    let s = " ".repeat(head % max) + s;
    let (s, t) = unicode_trancate(&s, max);
    let t_width = t.width();
    t + &s[t_width..]
}

fn unicode_trancate(s: &str, max: usize) -> (String, String) {
    let mut s = s.to_string();
    let mut buf = "".to_string();
    while s.width() > max {
        if let Some(t) = s.pop() {
            buf = String::from(t) + &buf;
        }
    }

    (s, buf)
}

fn main() {
    let ss = [" (」・ω・)」うー！", " (／・ω・)／にゃー！"];
    print_loop(&ss, 1000)
}
