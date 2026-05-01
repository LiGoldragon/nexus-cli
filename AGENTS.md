# Agent instructions — nexus-cli

You **MUST** read AGENTS.md at `github:ligoldragon/lore` — the workspace contract.

## Repo role

The **thin text shuttle CLI** for the nexus daemon. Reads nexus text from stdin or a file, writes it to `/tmp/nexus.sock`, prints the reply.

---

## Carve-outs worth knowing

- This crate is **stateless by design**. Each invocation opens a new connection, writes, half-closes, reads the reply, exits. Durable state lives in criome.
