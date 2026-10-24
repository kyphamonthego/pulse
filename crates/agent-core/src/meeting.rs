//! Realistic multi-agent meeting simulation.
//!
//! Agents take turns in a fixed order (standup / planning style).
//! Orchestrator opens, each reports status + proposal,
//! Reviewer raises risks, Orchestrator closes with a Decision
//! and a concrete prioritized roadmap of monetizable mini-apps.

use crate::agent::{Agent, AgentId, Role};
use crate::message::{Message, MessageKind};
use std::fmt;

#[derive(Debug)]
pub struct Meeting {
    pub topic: String,
    pub agents: Vec<Agent>,
    pub transcript: Vec<Message>,
}

/// One item on the product roadmap.
#[derive(Debug, Clone)]
pub struct RoadmapItem {
    pub priority: u8,
    pub name: String,
    pub problem: String,
    pub money_path: String,
    pub status: String, // "done" | "next" | "backlog"
}

#[derive(Debug, Clone)]
pub struct MeetingOutcome {
    pub topic: String,
    pub decision: String,
    pub next_slice: String,
    pub roadmap: Vec<RoadmapItem>,
    pub transcript_summary: Vec<String>,
}

impl Meeting {
    pub fn new(topic: impl Into<String>) -> Self {
        let roles = Role::all();
        let agents = roles.into_iter().map(Agent::new).collect();
        Self {
            topic: topic.into(),
            agents,
            transcript: Vec::new(),
        }
    }

    fn push(&mut self, from: AgentId, kind: MessageKind, content: impl Into<String>) {
        let msg = Message::new(from, kind, content);
        if let Some(agent) = self.agents.iter_mut().find(|a| a.role.id == from) {
            agent.speak(msg.content.clone());
        }
        self.transcript.push(msg);
    }

    /// Run a full realistic planning meeting that produces a concrete roadmap.
    pub fn run_standup(&mut self) -> MeetingOutcome {
        // 1. Orchestrator opens
        self.push(
            AgentId::Orchestrator,
            MessageKind::Agenda,
            format!(
                "Daily cycle. Topic: {}. Rule: only 1 vertical slice today (80/20). We must leave a concrete, prioritized ROADMAP of monetizable mini-apps so the path is visible.",
                self.topic
            ),
        );

        // 2. Researcher
        self.push(
            AgentId::Researcher,
            MessageKind::Status,
            "Current state: agent-core + focus-cli shipped (v0.3.0). User feedback: missing concrete roadmap. Constraint: pure std, noexec mount, flaky crates.io.",
        );
        self.push(
            AgentId::Researcher,
            MessageKind::Proposal,
            "Roadmap candidates (smallest problems first): 1) focus-cli (done) 2) invoice-cli (generate simple invoice text/PDF-ready) 3) streak-cli (habit streak counter) 4) quote-cli (daily quote + local history). All have clear paid unlock paths (export, multi-device, branding).",
        );

        // 3. Architect
        self.push(
            AgentId::Architect,
            MessageKind::Status,
            "Architecture stays workspace + crates/apps/*. ROADMAP.md becomes the single source of truth maintained by Documenter after every meeting.",
        );
        self.push(
            AgentId::Architect,
            MessageKind::Proposal,
            "Keep MeetingOutcome carrying a structured Vec<RoadmapItem>. Binary writes ROADMAP.md. No new crates.",
        );

        // 4. Implementer
        self.push(
            AgentId::Implementer,
            MessageKind::Status,
            "Can extend meeting.rs today to emit roadmap items and write ROADMAP.md. Then the next mini-app can be invoice-cli.",
        );
        self.push(
            AgentId::Implementer,
            MessageKind::Proposal,
            "Today's slice: make the meeting produce + persist a concrete roadmap. Do not start a new mini-app until the roadmap is visible and committed.",
        );

        // 5. Reviewer
        self.push(
            AgentId::Reviewer,
            MessageKind::Risk,
            "Risk: roadmap becomes stale or aspirational. Mitigation: every meeting re-writes ROADMAP.md from the Decision; status field is only done/next/backlog.",
        );
        self.push(
            AgentId::Reviewer,
            MessageKind::Status,
            "Pass criteria: ROADMAP.md exists, has prioritized items with problem + money_path, next_slice matches the #1 next item, tests still green.",
        );

        // 6. Documenter
        self.push(
            AgentId::Documenter,
            MessageKind::Status,
            "Will own ROADMAP.md, update CHANGELOG and ARCHITECTURE to point to it as the living plan.",
        );

        // 7. Orchestrator Decision + concrete roadmap
        let decision = "Ship a living ROADMAP.md produced by this meeting. focus-cli is done. Next vertical slice = invoice-cli (smallest billable problem). streak-cli and quote-cli stay backlog.".to_string();
        self.push(
            AgentId::Orchestrator,
            MessageKind::Decision,
            decision.clone(),
        );

        let roadmap = vec![
            RoadmapItem {
                priority: 1,
                name: "focus-cli".into(),
                problem: "Track start/stop of a focus session and log duration".into(),
                money_path: "Free core; paid weekly PDF report / multi-device sync later".into(),
                status: "done".into(),
            },
            RoadmapItem {
                priority: 2,
                name: "invoice-cli".into(),
                problem: "Generate a simple invoice (text + ready for print/PDF) from a few fields".into(),
                money_path: "Free single invoice; paid templates, logo, recurring, CSV export".into(),
                status: "next".into(),
            },
            RoadmapItem {
                priority: 3,
                name: "streak-cli".into(),
                problem: "Mark a habit done today and show current streak".into(),
                money_path: "Free local streaks; paid charts, reminders, cloud backup".into(),
                status: "backlog".into(),
            },
            RoadmapItem {
                priority: 4,
                name: "quote-cli".into(),
                problem: "Show one daily quote and keep a personal history".into(),
                money_path: "Free local; paid custom collections / themed packs".into(),
                status: "backlog".into(),
            },
        ];

        let next_slice = "invoice-cli: generate simple invoice from CLI flags, pure std, monetizable via templates later".to_string();

        let transcript_summary: Vec<String> = self
            .transcript
            .iter()
            .map(|m| m.to_string())
            .collect();

        MeetingOutcome {
            topic: self.topic.clone(),
            decision,
            next_slice,
            roadmap,
            transcript_summary,
        }
    }
}

impl MeetingOutcome {
    /// Render a markdown ROADMAP.md content.
    pub fn to_roadmap_md(&self) -> String {
        let mut out = String::new();
        out.push_str("# Pulse Roadmap\n\n");
        out.push_str("Living roadmap maintained by the multi-agent meeting system.\n");
        out.push_str(&format!("Last decision: {}\n\n", self.decision));
        out.push_str("| Priority | Mini-app     | Status  | Problem solved                                      | Money path                                      |\n");
        out.push_str("|----------|--------------|---------|-----------------------------------------------------|-------------------------------------------------|\n");
        for item in &self.roadmap {
            out.push_str(&format!(
                "| {}        | {:12} | {:7} | {:51} | {:47} |\n",
                item.priority, item.name, item.status, item.problem, item.money_path
            ));
        }
        out.push_str("\n## Next vertical slice\n");
        out.push_str(&format!("{}\n", self.next_slice));
        out.push_str("\n## Rules\n");
        out.push_str("- Only one vertical slice per day (80/20).\n");
        out.push_str("- Each mini-app solves the smallest possible problem.\n");
        out.push_str("- Pure std preferred until network is reliable.\n");
        out.push_str("- Roadmap is rewritten by agents after every meeting.\n");
        out
    }
}

impl fmt::Display for MeetingOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== MEETING OUTCOME ===")?;
        writeln!(f, "Topic   : {}", self.topic)?;
        writeln!(f, "Decision: {}", self.decision)?;
        writeln!(f, "Next    : {}", self.next_slice)?;
        writeln!(f, "\n--- Roadmap ---")?;
        for item in &self.roadmap {
            writeln!(
                f,
                "  [{}] {} ({}) – {}",
                item.status, item.name, item.priority, item.problem
            )?;
        }
        writeln!(f, "\n--- Transcript ---")?;
        for line in &self.transcript_summary {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}
