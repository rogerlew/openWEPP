# Harness Design

Evidence class: Static/Ran.

Status: queued.

Required architecture:

1. Dataset fetch/normalize command:
   - network-enabled by explicit operator command only;
   - writes raw files to `target/` or documented local cache;
   - writes normalized redistributable files and provenance locks only when
     license permits.
2. Local comparison command:
   - no network;
   - runs `openwepp-cli-hill` over site fixtures;
   - extracts WAT `frdp` in millimeters and converts to meters;
   - extracts modeled snow depth from an approved diagnostic;
   - aligns modeled rows to observation dates;
   - emits JSON and Markdown reports.
3. Test harness:
   - validates schema and provenance locks;
   - exercises at least one small local site comparison;
   - enforces measurement correspondence and censoring;
   - enforces that legacy/compatibility frost output is not the target.

Open design decisions:

- Whether the harness is a Rust integration test helper, a `tools/` command
  with Rust tests around fixtures, or both.
- Exact modeled snow-depth diagnostic surface. WAT `Snow-Water` is not enough
  for the `TOL-SNOWFREEZE-009` snow-depth gate.
- Whether normalized public observation extracts can be committed, or whether
  local tests must use a tiny synthetic contract fixture while full validation
  uses local cache.
