# LANED-T3-AGG - QA/governance review

Status: **EXECUTED** (2026-07-07). Verdict: **GO-WITH-AMENDMENTS**.

Evidence mode:

- **Static:** reviewed the package artifacts, prompt, rev-30 contract entry,
  README index entry, parent handoff, backlog update, touched Rust/source
  surfaces, and durable workflow docs.
- **Ran:** read-only timing/profile log inspection, line-count check, `rg`
  evidence search, and delegated `rust_qa_reviewer` review. No cargo gates or
  H2637 runs were rerun by this review.

## Findings

### High

None.

### Medium

**QA-M1 - Gate evidence does not exactly match the required closure commands.**

Anchor: `docs/work-packages/20260706-laned-router-t3-aggressive-deficit-carry-001/artifacts/gate-results.md:10`
and `docs/work-packages/20260706-laned-router-t3-aggressive-deficit-carry-001/artifacts/gate-results.md:12`.

The package records `cargo clippy --workspace --all-targets` rather than the
required `cargo clippy --workspace --all-targets -- -D warnings`, and records
`cargo nextest run --workspace` rather than
`cargo nextest run --workspace --profile full`. The canonical closure command
is in `crates/AGENTS.md:44`, and the `full` profile itself says to use that
form in `.config/nextest.toml:54`.

Required amendment: rerun or explicitly reconcile the exact required command
forms in `gate-results.md` before package closeout.

**QA-M2 - Rev-30 authority is concentrated in revision history, not the
normative contract tables.**

Anchor: `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md:480`,
with missing table surfaces around `:204`, `:428`, and `:443`.

The rev-30 changelog is detailed and contract-first, but the Branch and Guard
table has no explicit active-implicit selector/carry row, the Test-Vector
Obligations omit the rev-30 deficit-carry vectors, and the Binding Exposure
Index has no binding row for the experimental hybrid selector/carry behavior.

Failure scenario: a future contract-derived test, BEI audit, or refactor reads
the canonical tables rather than the revision-history paragraph and misses the
aggressive selector plus deficit-carry obligations while still appearing
contract-compliant.

Required amendment: promote the rev-30 selector/carry semantics into the
normative tables and BEI, while preserving the EXPERIMENTAL/UNRATIFIED posture.

**QA-M3 - Stale-binary mitigation is package-local and should be promoted.**

Anchor: `docs/work-packages/20260706-laned-router-t3-aggressive-deficit-carry-001/artifacts/fix-evidence.md:68`
and `docs/work-packages/20260706-laned-router-t3-aggressive-deficit-carry-001/artifacts/gate-results.md:32`.

The stale-binary near-miss is honestly documented: workspace
`cargo build --release` did not relink `openwepp-cli-hill`, and the session
caught it because books were bit-identical to a prior strict record. Keeping
this only in the package depends on future timing workers discovering this
specific artifact.

Required amendment: promote the evidence-build rule to durable workflow
guidance, for example `docs/work-packages/AGENTS.md`, `tools/local_ci/README.md`,
or the H2637 timing recipe: build with
`cargo build --release -p openwepp-runner --bins` and verify binary mtime/hash
before timing.

### Low

**QA-L1 - The active-runtime selector comment still describes the superseded
strict rule.**

Anchor: `crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs:123`.

The `hybrid_implicit` field comment still says the selector routes "strict
smooth bins" and references rev 28 behavior. Rev 30 supersedes strict with the
aggressive zero-source-only rule (`SC-OFEROUTE-001.md:480`). This is not a
runtime behavior bug, but it is a stale API-facing explanation of an
experimental selector.

Required amendment: update the comment when production-code amendments are
authorized.

**QA-L2 - Line-count governance is not recorded in package evidence.**

Anchor: `crates/AGENTS.md:49` and
`docs/work-packages/20260706-laned-router-t3-aggressive-deficit-carry-001/artifacts/gate-results.md:7`.

The touched Rust files remain below the warning threshold
(`cascade.rs` 1188 lines, `kinematic_wave.rs` 1858 lines), but the package gate
artifact does not record the line-count disposition required for closeout
audits.

Required amendment: add the line-count check result to package gate or
disposition evidence.

## Non-Findings

- Timing/prize disposition is honest. The package logs record aggressive
  endpoint user times of `38.28`, `38.32`, and `38.04` seconds, and the profile
  logs support the explicit-step reduction: total aggressive solver steps
  `5,806,728` minus `1,146,432` implicit steps gives `4,660,296` explicit
  steps, versus `10,479,200` plain active steps.
- Status naming is consistent across the package, README, parent handoff, and
  backlog: defect closure executed; timing prize not realized.
- The selector remains EXPERIMENTAL/UNRATIFIED in the reviewed package,
  contract changelog, parent handoff, and backlog text.

