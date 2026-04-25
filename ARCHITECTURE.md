# ARCHITECTURE — nexus-cli

The thin text client. Pipes nexus text into nexusd over UDS;
serialises replies back to text for the terminal. Stateless —
nexusd holds the connection state.

## Role

nexus-cli is one client of nexusd; future clients include editor
LSPs, agent harnesses, scripts. All speak [client-msg](https://github.com/LiGoldragon/nexusd/tree/main/src/client_msg)
(rkyv envelope around nexus text payloads + heartbeat / cancel /
resume control verbs). nexus-cli is the *reference* client.

## Boundaries

Owns:
- Reading nexus text from stdin / arguments.
- Wrapping it in a `client_msg::Send` with optional fallback path.
- Writing the resulting text reply to stdout.
- Heartbeat probes for long-running operations.
- Cancel / resume from a fallback file when the socket flaps.

Does not own:
- Parsing nexus text (nexusd does that, via nota-serde-core).
- Validation (criomed does that).
- Sema state (criomed owns it).

## Edit UX

The shell pattern is **request-composing**: nexus text in,
criomed-validated records out. Five write verbs (`Assert`,
`Mutate`, `Retract`, `Patch`, `TxnBatch`) and read verbs
(`Query`, `Subscribe`, `Validate`).

Two complementary read surfaces:

- **rsc-projected `.rs`** — for "what does this code do?"
  questions. Compile-friendly. Same view rustc/cargo see.
- **structured tree-view** — for "how is this structured in
  sema?" questions. Records, slots, change-log, derivations.

Both are first-class. Pick by the task: text projection for
flow, tree-view for structural manipulation.

Atomic batches use the `{|| ... ||}` syntax. Pattern-driven
mutations are client-side: query first, build N mutates, wrap
in a batch, send. Mechanism stays transparent.

## Diagnostics as iteration substrate

When validation fails, criomed returns a `Diagnostic` (signal-
side `Reply::Rejected`). For LLM iteration, the Diagnostic
record carries:

- `code` (`E0001`–`E9999` by failure class)
- `primary_site` — `Slot` reference, source span, or
  op-in-batch index
- `suggestions` with applicability flags
- `durable_record: Option<Slot>` — when the diagnostic was
  also asserted as a `Diagnostic` record in sema for review

The iteration loop: edit → reject → diagnostic → fix → re-send.
Diagnostics with `MachineApplicable` suggestions can be
auto-applied by the LLM front-end.

## Invariants

- **Stateless.** Each invocation builds one client-msg, sends,
  reads replies until done, exits. State that survives — like
  pending work — lives in nexusd, retrievable via `Resume`.
- **Text is text.** nexus-cli does not parse nexus; it just
  shuttles bytes.

## Code map

(All `todo!()` skeleton at present.)

- `src/main.rs` — CLI entry, argument parsing, UDS connection.
- `src/lib.rs` — re-exports for embedding.

## Cross-cutting context

- nexus language: [github.com/LiGoldragon/nexus](https://github.com/LiGoldragon/nexus)
- client-msg envelope (the wire format to nexusd):
  [nexusd::client_msg](https://github.com/LiGoldragon/nexusd/tree/main/src/client_msg)
- Project-wide architecture:
  [mentci-next/docs/architecture.md](https://github.com/LiGoldragon/mentci-next/blob/main/docs/architecture.md)

## Status

**Skeleton-as-design.** Body fills land alongside criomed
scaffolding (per mentci-next/reports/076 §4).
