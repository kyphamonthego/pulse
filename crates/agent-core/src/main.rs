//! pulse-agents – run a realistic multi-agent planning meeting
//! and persist the living ROADMAP.md.

use agent_core::meeting::Meeting;
use std::fs;
use std::path::Path;

fn main() {
    let mut meeting = Meeting::new(
        "Produce concrete prioritized roadmap of monetizable mini-apps",
    );
    let outcome = meeting.run_standup();
    println!("{}", outcome);

    // Persist the living roadmap at PROJECT_ROOT
    let roadmap_md = outcome.to_roadmap_md();
    let path = Path::new("ROADMAP.md");
    match fs::write(path, &roadmap_md) {
        Ok(()) => println!("\n\u2713 ROADMAP.md written ({} bytes)", roadmap_md.len()),
        Err(e) => eprintln!("\n\u2717 failed to write ROADMAP.md: {}", e),
    }
}
