# Review Disposition

Status: `EXECUTED-COMPLETE`

Delegated:

- `rust_code_reviewer` (`Hume`): read-only correctness review requested for
  W2 Rust changes, subprocess safety, fail-closed behavior, generated runfile
  path semantics, pass inventory/latest-event handling, and reuse-mode
  regressions.
- `rust_qa_reviewer` (`Kierkegaard`): read-only QA review requested for test
  coverage, maintainability, line-count governance, fixture realism, and
  consumer-path proof.
- `comparator_suite_runner` (`Faraday`): final closure gates delegated per W2
  package and prompt-wording guidance.

Reviewer findings accepted and fixed:

- Stale generated HBP artifacts could be routed because produced-by-job
  inventory state was not freshness-checked.
  Disposition: accepted. W2 now removes stale generated pass/manifest/timing
  artifacts before launch, writes a freshness marker immediately before child
  launch, and validates generated pass/manifest/timing modified times against
  that marker before routing. A focused fake-child test covers this.
- Routed-stage reuse was implicit because omitted `use_existing_pass_file`
  defaulted to true, and mixed `run_file` plus reuse could be ignored.
  Disposition: accepted. W2 now requires explicit `use_existing_pass_file` on
  every hillslope block and rejects reuse blocks that also declare `run_file`.
- Generated mode broke relative `--output-dir` semantics because the parent and
  child resolved generated runfile paths against different directories.
  Disposition: accepted. W2 resolves `--output-dir` to an absolute path before
  plan construction. A focused relative-output-dir test covers this.
- Clippy closure failed on `map(...).unwrap_or_else(...)` and
  `push_str(&format!(...))`.
  Disposition: accepted. The touched-crate clippy check now passes locally.

Reviewer residuals:

- Fixture realism remains intentionally limited to a small generated CLI fixture
  for W2; carnivorous-adobo remains an adopted input/runfile fixture, not an
  end-to-end HBP-output fixture.
- Typed error enums for the public supervisor API remain follow-on debt; the
  current CLI path still returns `String` errors as the surrounding binary does.
