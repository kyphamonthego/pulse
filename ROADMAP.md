# Pulse Roadmap

Living roadmap maintained by the multi-agent meeting system.
Last decision: Ship 3D management office preview so user can learn management by watching agents operate. focus-cli done. invoice-cli still next for monetization.

| Priority | Mini-app / Module      | Status  | Problem solved                                      | Money / Learning path                          |
|----------|------------------------|---------|-----------------------------------------------------|------------------------------------------------|
| 1        | focus-cli              | done    | Track start/stop of a focus session and log duration | Free core; paid weekly PDF report later        |
| 2        | office-preview (3D)    | done    | Visual multi-agent standup for learning management  | Learning tool; foundation for paid sim later   |
| 3        | invoice-cli            | next    | Generate a simple invoice from a few fields         | Free single invoice; paid templates later      |
| 4        | streak-cli             | backlog | Mark a habit done today and show current streak     | Free local; paid charts / reminders            |
| 5        | quote-cli              | backlog | Show one daily quote and keep personal history      | Free local; paid themed packs                  |

## Next vertical slice
invoice-cli: generate simple invoice from CLI flags, pure std, monetizable via templates later

## Preview
Open `preview/index.html` (Canvas 2D – works everywhere) or `preview/office.html` (Three.js WebGL).
6 robot agents hold a realistic standup. Side panel shows Agenda → Status → Proposal → Risk → Decision.

## Remote
- GitHub: https://github.com/kyphamonthego/pulse
- Connected 2026-10-24 by multi-agent system

## Rules
- Only one vertical slice per day (80/20).
- Each mini-app solves the smallest possible problem.
- Pure std preferred for CLI tools; CDN Three.js for the visual preview.
- Roadmap is rewritten by agents after every meeting.
