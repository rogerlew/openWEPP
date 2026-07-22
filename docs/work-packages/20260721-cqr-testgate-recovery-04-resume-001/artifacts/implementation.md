# Implementation

Static: production refactor commit `4b8f0ccef69304a170158d8f282326b3c99cf5b5`
mechanically decomposes the three owned trust decisions without changing public
APIs or serialized contracts.

- Candidate discovery now delegates record-root parsing, archive inspection,
  prior-plan/receipt loading, provenance-claims checks, and checkpoint admission
  along the original ordered boundaries.
- Native attestation separates command execution from retained-output
  interpretation while preserving the exact `gh` argument vector and parse
  before status/empty-result precedence.
- Checkpoint verification delegates identity, roots, same-execution,
  claims/attempt, and artifact checks in the original order.

Static: characterization commit `dc99797b` added isolated native-attestation,
checkpoint, candidate, and constructed-audit cases before production edits.
Test-only commits `7faa45f9`, `7f650cb0`, `47eb418d`, and `9c0db17d`
respectively closed Clippy shape, the first metric's receipt/reuse gaps,
reviewer hardening, and a fresh review finding about same-process checkpoint
fixture isolation. The production SHA-256 remained
`e2d5d61e24e7f695d87530a6a585fa84b2962cb2737516265b4e24bd4b1246d9`
after `4b8f0cce`. The isolation correction uses a process-local atomic sequence
in the owned fixture path while retaining RAII cleanup.
