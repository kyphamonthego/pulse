//! Agent identities and roles.

use std::fmt;

/// Unique identifier for an agent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AgentId {
    Orchestrator,
    Researcher,
    Architect,
    Implementer,
    Reviewer,
    Documenter,
}

impl fmt::Display for AgentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgentId::Orchestrator => write!(f, "Orchestrator"),
            AgentId::Researcher => write!(f, "Researcher"),
            AgentId::Architect => write!(f, "Architect"),
            AgentId::Implementer => write!(f, "Implementer"),
            AgentId::Reviewer => write!(f, "Reviewer"),
            AgentId::Documenter => write!(f, "Documenter"),
        }
    }
}

/// Role description – what the agent is responsible for.
#[derive(Debug, Clone)]
pub struct Role {
    pub id: AgentId,
    pub mission: &'static str,
    pub speaks_first: bool,
}

impl Role {
    pub fn all() -> Vec<Role> {
        vec![
            Role {
                id: AgentId::Orchestrator,
                mission: "Set agenda, keep scope small (80/20), decide done, facilitate meetings",
                speaks_first: true,
            },
            Role {
                id: AgentId::Researcher,
                mission: "Research best practices, competitors, libraries; write short notes",
                speaks_first: false,
            },
            Role {
                id: AgentId::Architect,
                mission: "Evaluate architecture impact, keep it simple, update ADRs if needed",
                speaks_first: false,
            },
            Role {
                id: AgentId::Implementer,
                mission: "Write clean, typed code + tests; ship the vertical slice",
                speaks_first: false,
            },
            Role {
                id: AgentId::Reviewer,
                mission: "Find edge cases, security, performance issues; only pass when solid",
                speaks_first: false,
            },
            Role {
                id: AgentId::Documenter,
                mission: "Update README, CHANGELOG, architecture docs; keep them accurate",
                speaks_first: false,
            },
        ]
    }
}

/// Runtime agent state (simple for now – no LLM, pure simulation).
#[derive(Debug, Clone)]
pub struct Agent {
    pub role: Role,
    pub last_statement: Option<String>,
}

impl Agent {
    pub fn new(role: Role) -> Self {
        Self {
            role,
            last_statement: None,
        }
    }

    pub fn speak(&mut self, statement: impl Into<String>) {
        self.last_statement = Some(statement.into());
    }
}
