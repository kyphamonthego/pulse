# Architecture – Pulse (Rust multi-agent monorepo)

## Vision
A single PROJECT_ROOT that hosts:
1. A **multi-agent system** (Orchestrator, Researcher, Architect, Implementer, Reviewer, Documenter) that holds realistic meetings and produces decisions.
2. Multiple **mini-apps** – each solves one tiny, monetizable problem.

Current version: **0.3.1**

## Layout
```
.
├── Cargo.toml                 # workspace
├── crates/
│   ├── agent-core/            # multi-agent runtime + meeting simulation
│   │   └── src/
│   │       ├── agent.rs
│   │       ├── message.rs
│   │       ├── meeting.rs
│   │       └── main.rs        # `pulse-agents` binary
│   └── apps/
│       └── focus-cli/         # first mini-app
│           └── src/main.rs    # `focus` binary
├── src/                       # legacy Python (kept for reference)
├── docs/
└── ...
```

## Multi-agent meeting protocol
1. Orchestrator opens with Agenda (scope rule: 1 vertical slice, 80/20).
2. Each agent in order emits Status + Proposal (or Risk).
3. Reviewer surfaces risks.
4. Orchestrator records a Decision + next_slice.
5. Transcript is deterministic and human-readable.

Agents communicate only via typed `Message` (no shared mutable state beyond the meeting).

## Mini-app principles
- One tiny problem only.
- Pure `std` preferred while network is unreliable.
- Clear future monetization path (export, report, sync, unlock).
- Lives under `crates/apps/<name>`.

## First mini-app: focus-cli
Solves: “I started focusing – when did I stop and for how long?”
Commands: `start`, `status`, `stop`, `log`.
Data: local files under temp. Future paid: PDF export / weekly report.

## Non-goals (current)
- LLM calls, external crates, persistence beyond local files, multi-user.


## Living Roadmap
`ROADMAP.md` is the single source of truth. It is rewritten by the multi-agent meeting after every standup. Status values are only `done`, `next`, `backlog`.
