//! Typed messages exchanged between agents during meetings.

use crate::agent::AgentId;
use std::fmt;

#[derive(Debug, Clone)]
pub enum MessageKind {
    /// Opening the meeting / setting agenda
    Agenda,
    /// Status update from an agent
    Status,
    /// Proposal for next work
    Proposal,
    /// Objection or risk raised
    Risk,
    /// Final decision recorded by Orchestrator
    Decision,
}

impl fmt::Display for MessageKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageKind::Agenda => write!(f, "AGENDA"),
            MessageKind::Status => write!(f, "STATUS"),
            MessageKind::Proposal => write!(f, "PROPOSAL"),
            MessageKind::Risk => write!(f, "RISK"),
            MessageKind::Decision => write!(f, "DECISION"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Message {
    pub from: AgentId,
    pub kind: MessageKind,
    pub content: String,
}

impl Message {
    pub fn new(from: AgentId, kind: MessageKind, content: impl Into<String>) -> Self {
        Self {
            from,
            kind,
            content: content.into(),
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {}: {}",
            self.kind, self.from, self.content
        )
    }
}
