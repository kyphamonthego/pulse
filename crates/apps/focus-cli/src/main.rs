//! focus – minimal focus session CLI.
//!
//! Solves the smallest problem: "I started focusing, when did I stop, how long?"
//! Local log only. Future paid unlock: export PDF / weekly report / sync.
//!
//! Usage:
//!   focus start [title]
//!   focus status
//!   focus stop
//!   focus log

use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn data_dir() -> PathBuf {
    let mut p = env::temp_dir();
    p.push("pulse-focus");
    let _ = fs::create_dir_all(&p);
    p
}

fn current_path() -> PathBuf {
    data_dir().join("current.txt")
}

fn log_path() -> PathBuf {
    data_dir().join("sessions.log")
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn format_duration(secs: u64) -> String {
    let h = secs / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;
    if h > 0 {
        format!("{}h {}m {}s", h, m, s)
    } else if m > 0 {
        format!("{}m {}s", m, s)
    } else {
        format!("{}s", s)
    }
}

fn start(title: &str) -> io::Result<()> {
    let path = current_path();
    if path.exists() {
        eprintln!("A session is already running. Use `focus status` or `focus stop`.");
        std::process::exit(1);
    }
    let ts = now_secs();
    let content = format!("{}\n{}\n", ts, title);
    fs::write(&path, content)?;
    println!("\u25b6 Started: \"{}\" at unix {}", title, ts);
    Ok(())
}

fn status() -> io::Result<()> {
    let path = current_path();
    if !path.exists() {
        println!("No active focus session.");
        return Ok(());
    }
    let raw = fs::read_to_string(&path)?;
    let mut lines = raw.lines();
    let start: u64 = lines.next().unwrap_or("0").parse().unwrap_or(0);
    let title = lines.next().unwrap_or("Focus session");
    let elapsed = now_secs().saturating_sub(start);
    println!("\u25cf Active: \"{}\"", title);
    println!("  Elapsed: {}", format_duration(elapsed));
    Ok(())
}

fn stop() -> io::Result<()> {
    let path = current_path();
    if !path.exists() {
        println!("No active session to stop.");
        return Ok(());
    }
    let raw = fs::read_to_string(&path)?;
    let mut lines = raw.lines();
    let start: u64 = lines.next().unwrap_or("0").parse().unwrap_or(0);
    let title = lines.next().unwrap_or("Focus session").to_string();
    let end = now_secs();
    let elapsed = end.saturating_sub(start);
    // append to log
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path())?;
    writeln!(
        file,
        "{} | {} | {} | {}",
        start, end, elapsed, title
    )?;
    fs::remove_file(&path)?;
    println!("\u25a0 Stopped: \"{}\" – {}", title, format_duration(elapsed));
    Ok(())
}

fn show_log() -> io::Result<()> {
    let path = log_path();
    if !path.exists() {
        println!("No sessions logged yet.");
        return Ok(());
    }
    let raw = fs::read_to_string(&path)?;
    println!("Session log (start | end | secs | title):");
    for line in raw.lines() {
        println!("  {}", line);
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd = args.get(1).map(|s| s.as_str()).unwrap_or("help");

    let result = match cmd {
        "start" => {
            let title = args.get(2).map(|s| s.as_str()).unwrap_or("Focus session");
            start(title)
        }
        "status" => status(),
        "stop" => stop(),
        "log" => show_log(),
        _ => {
            println!("focus – tiny focus tracker (Pulse mini-app)");
            println!();
            println!("Commands:");
            println!("  focus start [title]  Start a session");
            println!("  focus status         Show current session");
            println!("  focus stop           Stop and log session");
            println!("  focus log            Show history");
            println!();
            println!("Data lives in {:?}", data_dir());
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
