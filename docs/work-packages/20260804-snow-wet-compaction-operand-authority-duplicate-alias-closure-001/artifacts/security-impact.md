# Security Impact

Status: complete / anti-evasion and AUTH11 pass

Evidence mode: Static + Ran

- Secrets: no credential, token, host-specific configuration, or environment
  value is added. The materiality receipt records only removed
  `OPENWEPP_*` key names and the package's fixed effective selectors.
- Dependencies: no dependency, lockfile, license, or source-policy change.
  The root manifest changes only integration-test registration. Because the
  literal terminal diff still touches `Cargo.toml`, `cargo deny check` is
  conservatively required and recorded in `gate-results.md`.
- External sources: PySnobal authority was inspected from a checksum-pinned
  0.2.3 source archive. Runtime/package tools do not fetch network content.
- Fixture custody: canonical Snowbird `p8.cli` remains SHA-256
  `10c1ede...11a7`. The derived `DEVELOPMENT_ONLY` CLI is deterministic,
  provenance-stamped, precipitation-only, and staged by copy; the tool refuses
  a canonical source-hash mismatch.
- Output confinement: materiality runs copy fixtures and write only beneath
  `target/snow_wet_compaction_operand_closure`; the runner refuses to overwrite
  an existing evidence directory.
- Fail-closed behavior: the new runtime value uses the existing typed finite
  and nonnegative snow boundary guard. The materiality tool also refuses stale
  predecessor/current hashes, concurrent source/binary/tool changes, tolerance
  failures, and zero-materiality evidence before atomically publishing the
  final result. No fallback, clamp, unsafe block, `unwrap`, or `expect` was
  added to production code.
- Surface exposure: the production field is private. No public runtime or
  trace schema changed. Offline snowbench columns are explicit diagnostics.
- Evasion: the real production consumer and offline mirror are both source-
  bound by contract-derived tests. Required authority anti-evasion commands
  are delegated and recorded in `gate-results.md`.
