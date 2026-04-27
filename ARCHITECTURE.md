# ARCHITECTURE — nexus-cli

The thin text client for the [nexus](https://github.com/LiGoldragon/nexus)
daemon. Reads nexus text from stdin or a file, writes it to the
daemon over UDS at `/tmp/nexus.sock`, prints the daemon's reply
text. Stateless — the daemon (and ultimately criome) holds all
durable state.

## Role

nexus-cli is one client of the nexus daemon; future clients
include editor LSPs, agent harnesses, scripts. **All clients
exchange pure nexus text** with the daemon over its UDS socket
— no envelope, no rkyv, no framing format. The text grammar
self-delimits on matched parens.

nexus-cli is the *reference* client: the simplest possible
shape of a text-in-text-out shuttle.

## Boundaries

Owns:

- Reading nexus text from stdin or a file argument.
- Connecting to `/tmp/nexus.sock` and writing the text bytes.
- Reading the reply bytes back and printing them to stdout.

Does not own:

- Parsing nexus text. The daemon does that, via
  [nota-serde-core](https://github.com/LiGoldragon/nota-serde-core)
  (for the typed verbs) and the daemon's own
  [`QueryParser`](https://github.com/LiGoldragon/nexus/blob/main/src/parse.rs)
  (for query containers).
- Validation. criome does that.
- Sema state. criome owns it.
- Any wire framing. The CLI ↔ daemon leg is plain text;
  the daemon ↔ criome leg is length-prefixed rkyv (signal),
  but that's the daemon's concern.

## Edit surface

Per the [nexus grammar](https://github.com/LiGoldragon/nexus/blob/main/spec/grammar.md),
the verbs the daemon accepts (and therefore the CLI can shuttle)
are:

| Sigil + delimiter      | Verb         |
|------------------------|--------------|
| `(R …)`                | Assert       |
| `~(R …)`               | Mutate       |
| `!(R …)` / `!slot`     | Retract      |
| `?(...)`               | Validate     |
| `(\| pat \|)`          | Query        |
| `*(\| pat \|)`         | Subscribe    |
| `[\| op1 op2 … \|]`    | Atomic batch |

Atomic batches use the `[\| \|]` form (square-bracket-pipe).
Pattern-driven mutations expand at the daemon: the user writes
`~(\| pat \|) (NewRecord …)` once; the daemon expands to one
MutateOp per match inside an AtomicBatch.

## Diagnostics as iteration substrate

When validation fails, criome returns an `OutcomeMessage::
Diagnostic(Diagnostic)` (per [signal/src/reply.rs](https://github.com/LiGoldragon/signal/blob/main/src/reply.rs))
which the daemon renders as a `(Diagnostic …)` record in the
reply text. The Diagnostic carries:

- `code` (`E0001`–`E9999` by failure class)
- `level` (Error / Warning / Info)
- `primary_site` — `Slot` reference, source span, or
  op-in-batch index
- `suggestions` with applicability flags
- `durable_record: Option<Slot>` — when the diagnostic was
  also asserted as a `Diagnostic` record in sema for review

The iteration loop: edit → reject → diagnostic → fix → re-send.
Diagnostics with `MachineApplicable` suggestions can be
auto-applied by an LLM front-end.

## Invariants

- **Stateless.** Each invocation opens a connection, writes
  text, reads response text, exits. No fallback files, no
  resume-after-disconnect; durable state lives in criome's
  sema and is retrieved by issuing a Query.
- **Text is text.** nexus-cli does not parse nexus; it just
  shuttles bytes between stdin/file and the socket.
- **No handshake on this leg.** The CLI ↔ daemon leg has no
  protocol-version negotiation. The daemon ↔ criome leg
  carries the signal handshake; that's the daemon's
  responsibility.

## Code map

```
src/
├── main.rs    — CLI entry: argument parsing, UDS connect, byte shuttle
└── error.rs   — error type (I/O wrapper)
```

(`main.rs` is currently a stub returning `Ok(())`; body lands
alongside the M0 daemon body — see
[mentci/reports/089](https://github.com/LiGoldragon/mentci/blob/main/reports/089-m0-implementation-plan-step-3-onwards.md).)

## Cross-cutting context

- nexus grammar:
  [nexus/spec/grammar.md](https://github.com/LiGoldragon/nexus/blob/main/spec/grammar.md)
- nexus daemon (the other end of this socket):
  [nexus/ARCHITECTURE.md](https://github.com/LiGoldragon/nexus/blob/main/ARCHITECTURE.md)
- Project-wide architecture:
  [criome/ARCHITECTURE.md](https://github.com/LiGoldragon/criome/blob/main/ARCHITECTURE.md)

## Status

**Skeleton.** Body lands alongside the M0 nexus daemon body.
