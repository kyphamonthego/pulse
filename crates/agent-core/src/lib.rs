//! Multi-agent core for Pulse.
//!
//! Agents communicate via typed messages. The Orchestrator runs realistic
//! "meetings" (standups / planning) where each agent speaks in turn,
//! records decisions, and produces an actionable daily plan + living roadmap.

pub mod agent;
pub mod meeting;
pub mod message;

pub use agent::{Agent, AgentId, Role};
pub use meeting::{Meeting, MeetingOutcome, RoadmapItem};
pub use message::{Message, MessageKind};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standup_produces_decision_and_roadmap() {
        let mut m = Meeting::new("test topic");
        let outcome = m.run_standup();
        assert!(!outcome.decision.is_empty());
        assert!(!outcome.next_slice.is_empty());
        assert!(outcome.roadmap.len() >= 3);
        assert!(outcome.transcript_summary.iter().any(|l| l.contains("DECISION")));
        // next item must be marked "next"
        assert!(outcome.roadmap.iter().any(|i| i.status == "next"));
        // at least one done
        assert!(outcome.roadmap.iter().any(|i| i.status == "done"));
    }

    #[test]
    fn roadmap_md_contains_table() {
        let mut m = Meeting::new("test");
        let outcome = m.run_standup();
        let md = outcome.to_roadmap_md();
        assert!(md.contains("# Pulse Roadmap"));
        assert!(md.contains("focus-cli"));
        assert!(md.contains("invoice-cli"));
        assert!(md.contains("Next vertical slice"));
    }

    #[test]
    fn all_roles_present() {
        let roles = Role::all();
        assert_eq!(roles.len(), 6);
        assert!(roles.iter().any(|r| r.id == AgentId::Orchestrator));
    }
}
