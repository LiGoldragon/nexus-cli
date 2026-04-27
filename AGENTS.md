# Agent instructions

Repo role: the **thin text shuttle CLI** for the nexus daemon. Reads nexus text from stdin or a file, writes it to `/tmp/nexus.sock`, prints the reply.

Read [ARCHITECTURE.md](ARCHITECTURE.md) for boundaries.

Workspace conventions live in [mentci/AGENTS.md](https://github.com/LiGoldragon/mentci/blob/main/AGENTS.md) — beauty, methods on types, full-English naming, S-expression commit messages, jj + always-push.

This crate is **stateless by design**. Each invocation opens a new connection, writes, half-closes, reads the reply, exits. Don't add caching, retries, or fallback files — durable state lives in criome.
