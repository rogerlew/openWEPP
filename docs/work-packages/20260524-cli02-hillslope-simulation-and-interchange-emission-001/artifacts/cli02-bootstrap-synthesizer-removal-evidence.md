# CLI02 Bootstrap Synthesizer Removal Evidence

Status: complete
Evidence mode: Static

## Static
- Production acceptance semantics were updated to remove bootstrap-synthesized
  placeholder output posture.
- Canonical authority now requires `.run`-declared required outputs:
  - `outputs.pass` (`.hbp`)
  - `outputs.loss` (`.json`)
- Optional parquet outputs remain opt-in via `.run` output keys.
- Runtime code-path removal/verification of any remaining bootstrap synthesis
  behavior is deferred to CLI03 implementation.

## Ran
- None in CLI02 planning scope.
