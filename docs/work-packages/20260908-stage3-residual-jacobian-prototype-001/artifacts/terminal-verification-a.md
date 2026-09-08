# Independent terminal verification A — corrected cut

Static: root/package instructions, verifier procedure, testing strategy sections
7/9/10/18, science obligations, corrected v33 authority and package artifacts,
terminal reviews/disposition, and detached Phase-A source. Ran: all commands
below on the corrected cut.

Role/session: terminal verifier A; configured/requested effort: independent
bounded verification; effective runtime setting: UNOBSERVED. Reviewed identity:
primary scaffold HEAD `827a7470e058a5e09f9feced9375b776e5959789`
plus the current documented v33/package diff; detached HEAD
`e89befa4678eadec039b3e7f7fe0a176af8e9dc5`; corrected oracle blobs
`b39fe2b56be540369fa7800fcd69224ebc7f95c0` and
`7786a69d1438bc0f6dbbbaf0df0fcd336958cd7e`; terminal bundle-v2 manifest
SHA-256 `ea4b42170f458bb24a3e073e734758cf52f320112ab49376bc5ea7e132726f26`.
Assigned scope: corrected authority/binding/unit checks, exact corpus and
lifecycle custody, oracle self-tests, machine-checked 16/0 result, warnings
gate, accepted-finding closure, and terminal-cut freshness. No source or
configuration was edited.

## Independent checks

- `.venv/bin/python tools/check_sc_binding_exposure.py --strict
  docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`:
  exit 0; PASS, directory-v1, 18 binding-exposure rows and 102 definitions.
- `bash tools/release/check_sc_unit_compliance.sh --path
  docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`:
  exit 0; PASS, no unit-compliance findings.
- `nix develop --offline -c cargo nextest run --test
  land_surface_energy_balance_authority_contract
  version_thirty_three_binds_experimental_residual_jacobian_policy`: exit 0;
  PASS 1/1 with 26 skipped. The primary tree emitted one unrelated existing
  orchestrator dead-code warning; this command was not represented as the
  warnings-denied gate.
- `.venv/bin/python tools/agents/evidence_bundle.py verify
  /tmp/openwepp-rj-phase-a-terminal-bundle-v2`: exit 0. The manifest and sidecar
  both hash to `ea4b4217...726f26`; all 17 members passed inventory, size and
  digest verification. Independent SHA-256 comparison found all 12 captured
  source files byte-identical to the live detached source.
- The v2 protocol member binds and matches the current entry contract SHA-256
  `47d4590b...eeb02f`, numerical-methods SHA-256
  `b05a332f...d680`, and authority/protocol artifact SHA-256
  `7285ffd4...a275`. The bundle truthfully retains a no-executable marker and
  disclaims bit-reproducible rebuilding.
- Independent corpus inspection and regeneration reproduce SHA-256
  `a05ca09031c28f863bc8dd8f3cc8d605459a683abc2af6e0f37dd84c713068c0`,
  509,132 bytes, 8 records split 4 Potential/4 FixedFinal and 4 first/4 last,
  N=2/S=2/D=25 throughout, and 4,636 finite exact-tagged binary64 values. Both
  executed stem areas are positive in every record (0.72 and 0.4176). The eight
  lifecycle records have distinct scopes, nonzero solve/parent/scope joins, and
  source-real potential/fixed-final solve/iteration relationships.
- `nix develop --offline -c cargo test -p openwepp-land-surface-energy
  solver_stem_jacobian_tests::v33_oracle_selector_self_tests_precede_candidate_code
  -- --exact --nocapture` from the detached tree: exit 0; PASS 1/1. The corrected
  source checks abs-zero sign crossing, shared-heat max-floor crossing and exact
  tie rejection, and applies shared-heat pre-floor-region stability to every
  corpus probe.
- With verifier-specific corpus output,
  `OPENWEPP_RESIDUAL_CORPUS_PATH=/tmp/openwepp-rj-terminal-verifier-a-v2.json
  nix develop --offline -c cargo test -p openwepp-land-surface-energy
  solver::tests::covered_v8_block_matches_frozen_joint_solution -- --exact
  --nocapture`: exit 0; PASS 1/1. The test now machine-asserts exactly 16
  candidates and 0 supported, emits one nonempty missing-row set for every
  lifecycle/column pair, and prints `0/16`. The regenerated corpus is byte-for-
  byte identical to the retained corpus. My complete 30-line test output is
  also byte-identical to bundled result `openwepp-rj-phase-a-final-oracle.log`
  (SHA-256 `e691b078...59a4`).
- `nix develop --offline -c cargo clippy -p
  openwepp-land-surface-energy --tests -- -D warnings` in the detached tree:
  exit 0; PASS. No prior observation-hook warning remains.
- Static negative inspection again found no `solver_stem_jacobian.rs`, v33
  derivative capability, derivative-integrity implementation, or candidate
  executable. Candidate correctness, full-solve equivalence, consumer, cost,
  timing, memory and scale gates therefore remain honestly NOT RUN.

## Accepted-finding and freshness reconciliation

TV-A-001 is fixed: package, preimplementation, kernel checklist, catalog,
active locator and roadmap consistently say terminal correction is in progress,
identify the zero-stem corpus as invalid, and identify the 0/16 Phase-A boundary.
The status is not prematurely COMPLETE. The detached line-count artifact now
inventories the retained Rust, records the 2,685-line WARN rationale/split
trigger, and correctly reports no 3,000-line file.

TV-A-002 is fixed: the authentic transaction now asserts `candidates == 16`
and `supported == 0`; the exact per-column missing-row log, corrected source,
protocol hashes, corpus and no-executable marker are jointly retained in the
verified v2 bundle. The new shared-floor checks resolve the terminal oracle-mask
finding without widening any v33 step, basin or tolerance constant. The exact
0/16 result and corpus remain unchanged.

The final freshness check found the package in `TERMINAL REVIEW CORRECTION`, its
last terminal-role checkbox intentionally open, the handoff saying terminal
review pending, and both terminal review and verification paths present. This
is a coherent pre-completion state. Under the non-deferral rule, stopping before
J is legitimate: the prospectively frozen oracle admitted no complete column,
so later candidate-only gates are ineligible rather than deferred or passed.

Findings: none on the corrected cut. Previously accepted terminal findings were
fixed and independently rechecked as described above.

Uncertainty: effective model/runtime settings and workflow-total reading
exposure remain UNOBSERVED. This verification does not claim bit-reproducible
rebuilding, derivative correctness, candidate equivalence, performance, scale
behavior, or production readiness.

Verdict: **PASS on the corrected terminal cut.** The exact authentic transaction,
machine-asserted 16-candidate/0-supported baseline oracle, byte-identical corpus
and log, corrected protocol/source custody, and warnings-denied focused source
gate support `INCONCLUSIVE_WITH_BOUND`, stop-before-J, all downstream candidate
gates NOT RUN, and continuing production HOLD. This verifier has no remaining
blocker to final package completion after the other independent terminal roles
confirm their corrected-cut freshness.
