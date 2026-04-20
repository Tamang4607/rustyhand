# Meta-Agent Pattern for RustyHand

Three coordinated agents that turn RustyHand from "set of fixed tools" into a
self-extending assistant.

```
┌──────────────┐
│ User (Tg/Web)│
└──────┬───────┘
       │ task / question
       ▼
┌─────────────────────────────────────────┐
│  COORDINATOR  — front door, only routes │
│  • agent_list to discover specialists   │
│  • agent_send to delegate               │
│  • synthesizes responses                │
└─────┬───────────────────────────┬───────┘
      │ known task                │ no agent fits
      ▼                           ▼
┌──────────────┐         ┌────────────────────┐
│  Specialist  │         │ CAPABILITY-BUILDER │
│  (coder,     │         │ • Bucket 1 → skill │
│  researcher, │         │ • Bucket 2 → MCP   │
│  analyst,    │         │ • Bucket 3 → spec  │
│  ops, ...)   │         │   for human        │
└──────┬───────┘         └────────────────────┘
       │ failure
       ▼
┌──────────────┐
│ DIAGNOSTIC   │  reads audit log, KV state, recent sessions
│              │  → root-cause report + workaround
└──────────────┘
```

## What's in this folder

| File | Role |
|------|------|
| `coordinator/agent.toml` | Routes user requests, delegates, synthesizes |
| `capability-builder/agent.toml` | Adds new capabilities (skills/MCP/spec) |
| `diagnostic/agent.toml` | Investigates failures, proposes fixes |
| `coordinator/example_skill.py` | Template showing what a generated skill looks like |

## Setup

1. **Copy agents to your `~/.rustyhand/agents/` (or use bundled spawn):**
   ```bash
   rustyhand agent spawn --manifest agents/coordinator/agent.toml
   rustyhand agent spawn --manifest agents/capability-builder/agent.toml
   rustyhand agent spawn --manifest agents/diagnostic/agent.toml
   ```

2. **Wire Coordinator to Telegram** (`~/.rustyhand/config.toml`):
   ```toml
   [channels.telegram]
   bot_token_env = "TELEGRAM_BOT_TOKEN"
   default_agent = "coordinator"   # all incoming messages go here first
   ```

3. **Restart kernel** to load agents:
   ```bash
   rustyhand restart
   ```

4. **Test**: send any message to your Telegram bot. Coordinator will pick up,
   classify, and route.

## How it learns

- Every time a delegation works well, Coordinator stores a routing hint in
  `memory.routing.<task_kind>` so similar tasks go to the right agent faster.
- Every capability Capability-Builder adds is logged to
  `memory.capabilities.added.<name>` — over time, this is your portfolio of
  custom-built tools.
- Every incident Diagnostic analyses is saved to
  `memory.incidents.<timestamp>` — recurring patterns get flagged.

## What it CANNOT do (honest limits)

- **Modify Rust source code** — Capability Builder only writes Python/JS skills
  or MCP configs. For new built-in tools / channel adapters / LLM drivers,
  it produces a SPEC and pings you on Telegram.
- **Solve CAPTCHAs** — needs an external service (CapSolver) wired in.
- **Computer Use (GUI control)** — RustyHand has browser automation but not
  desktop-app control.

## Cost expectation

With Anthropic Sonnet for all 3 agents:
- Coordinator: ~50-200 tokens per routing decision = ~$0.001 per request
- Capability Builder: ~2-5K tokens per new capability = ~$0.02 per skill added
- Diagnostic: ~3-8K tokens per incident = ~$0.03 per RCA report

For typical "30 requests/day, 1 new skill/week, 2 incidents/month": ~$2-5/month.

## Iteration

This is a starting point. Tune by:
- Adding more specialist agents to your registry — Coordinator will discover them
- Refining Coordinator's prompt with real routing failures you observe
- Adding domain-specific skills under `~/.rustyhand/skills/` (Capability Builder
  will use them too)
