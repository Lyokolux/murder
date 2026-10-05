use std::env;
use std::io::{self, Write};
use std::process::{self, Command};
use std::thread::sleep;
use std::time::Duration;

const SIGNALS: [(u32, u64); 4] = [(15, 3), (2, 3), (1, 4), (9, 0)];

fn run(cmd: &str, args: &[&str]) -> String {
    Command::new(cmd)
        .args(args)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
        .unwrap_or_default()
}

fn is_running(pid: &str) -> bool {
    run("ps", &["-p", pid]).lines().count() == 2
}

fn go_ahead() -> bool {
    io::stdout().flush().ok();
    let mut input = String::new();
    if io::stdin().read_line(&mut input).unwrap_or(0) == 0 {
        return false;
    }
    matches!(input.trim().to_lowercase().as_str(), "y" | "yes" | "yas")
}

fn kill(pid: &str, code: u32) {
    run("kill", &[&format!("-{code}"), pid]);
}

fn murder_pid(pid: &str) {
    for (code, wait) in SIGNALS {
        if !is_running(pid) {
            break;
        }

        kill(pid, code);
        sleep(Duration::from_millis(500));
        if is_running(pid) {
            sleep(Duration::from_secs(wait));
        }
    }
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn contains_word(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return false;
    }
    let haystack = haystack.to_lowercase();
    let needle = needle.to_lowercase();
    haystack.char_indices().any(|(start, _)| {
        if !haystack[start..].starts_with(&needle) {
            return false;
        }
        let end = start + needle.len();
        let before_ok = haystack[..start].chars().next_back().is_none_or(|c| !is_word_char(c));
        let after_ok = haystack[end..].chars().next().is_none_or(|c| !is_word_char(c));
        before_ok && after_ok
    })
}

fn murder_names(name: &str) {
    let own_pid = process::id();
    let output = run("ps", &["-eo", "pid command"]);
    for line in output.lines().skip(1) {
        if !contains_word(line, name) {
            continue;
        }

        let mut parts = line.trim_start().splitn(2, char::is_whitespace);
        let pid = parts.next().unwrap_or("");
        let fullname = parts.next().unwrap_or("").trim();

        if pid.parse::<u32>() == Ok(own_pid) {
            continue;
        }

        print!("murder {fullname} (pid {pid})? ");
        if go_ahead() {
            murder_pid(pid);
        }
    }
}

fn murder_port(arg: &str) {
    let output = run("lsof", &["-t", "-i", arg, "-sTCP:LISTEN"]);
    for pid in output.lines().map(str::trim).filter(|p| !p.is_empty()) {
        let ps = run("ps", &["-o", "command=", "-p", pid]);
        let fullname = ps.trim();

        print!("murder {fullname} (pid {pid})? ");
        if go_ahead() {
            murder_pid(pid);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let Some(arg) = args.first() else {
        println!("usage:");
        println!("murder 123    # kill by pid");
        println!("murder ruby   # kill by process name");
        println!("murder :3000  # kill by port");
        process::exit(1);
    };

    let is_pid = arg.parse::<u32>().is_ok_and(|p| p != 0);
    let is_port = arg
        .strip_prefix(':')
        .is_some_and(|p| p.parse::<u16>().is_ok_and(|p| p != 0));

    if is_pid {
        murder_pid(arg);
    } else if is_port {
        murder_port(arg);
    } else {
        murder_names(arg);
    }
}
