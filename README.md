# Pulse

Rust multi-agent monorepo of tiny, monetizable mini-apps.

## Status
**v0.3.1** – Multi-agent meeting simulation + first mini-app `focus-cli` + living ROADMAP (2026-10-24).

## Quick start

```bash
# Build (incremental disabled because of FS)
CARGO_INCREMENTAL=0 cargo build --workspace

# Run the multi-agent standup (copy binary if mount is noexec)
cp target/debug/pulse-agents /tmp/ && chmod +x /tmp/pulse-agents
/tmp/pulse-agents

# Focus mini-app
cp target/debug/focus /tmp/ && chmod +x /tmp/focus
/tmp/focus start "Deep work"
/tmp/focus status
/tmp/focus stop
/tmp/focus log
```

## Agents
| Agent        | Mission                                      |
|--------------|----------------------------------------------|
| Orchestrator | Agenda, scope control, final decision        |
| Researcher   | Best practices, competitors, constraints     |
| Architect    | Keep architecture simple                     |
| Implementer  | Ship the vertical slice                      |
| Reviewer     | Edge cases, security, performance            |
| Documenter   | README / CHANGELOG / ARCHITECTURE accuracy   |

They hold a realistic standup; the transcript is printed by `pulse-agents`.

## Mini-apps
| App        | Problem solved                          | Money path                     |
|------------|-----------------------------------------|--------------------------------|
| focus-cli  | Track start/stop of focus sessions      | Paid export / weekly reports   |

## Living Roadmap
See [ROADMAP.md](./ROADMAP.md) — maintained automatically by the multi-agent meeting system after every standup. It is the single source of truth for what is done, what is next, and what is backlog.

## License
MIT
