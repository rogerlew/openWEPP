# Frozen Evaluator Obligation Key

Evidence class: Static.

This key was frozen before participant trials. Reviewers may clarify wording
but may not add or remove scored obligations after arm labels are unblinded.
An obligation is covered only when the submitted plan names the action,
applicability, and governing package/repository source closely enough for an
agent to execute it without guessing.

## Mode Keys

Every package/mode includes these critical keys.

### Pre-edit

- `PRE-AUTH`: confirm package authorization, intent, and current status.
- `PRE-INSTR`: resolve applicable `AGENTS.md` instructions for intended paths.
- `PRE-SCOPE`: reconcile included/excluded scope and declared write set.
- `PRE-VALID`: select direct canonical validation from declared intent and
  likely affected surfaces; do not treat a linter finding as authority.
- `PRE-BOUNDARY`: preserve named security, science, protected-data, and
  no-external-action boundaries.

### Working-tree

- all `PRE-*` keys;
- `WT-DIFF`: inspect index, tracked worktree, and untracked paths separately;
- `WT-WRITESET`: identify every changed path outside the declared write set;
- `WT-GATES`: revise focused requirements from the actual affected surfaces;
  and
- `WT-NOEXEC`: propose commands only; do not execute package/model/CAL work.

### Terminal

- all `PRE-*` keys;
- `TERM-IDENTITY`: resolve declared base and exact head; report missing or
  ambiguous identity rather than inventing it;
- `TERM-DIFF`: reconcile the exact base-to-head diff plus remaining dirty paths
  against the write set;
- `TERM-EVIDENCE`: require direct evidence for every current-scope acceptance
  criterion, including review/verification/prompt state when declared;
- `TERM-STATUS`: match closure/status verbs to evidence and keep any failed
  current requirement open; and
- `TERM-NOEXEC`: produce the plan without executing suggested commands.

## Package Keys

### DOC-1 — Advisory-Linter Roadmap

- `DOC1-ONLY`: documentation-only; no executable, workflow, schema, fixture,
  contract, calibration, or protected-data change.
- `DOC1-AUTH`: ADR-0043 must make the linter advisory, read-only, non-CI,
  non-lifecycle, and nonblocking while preserving underlying obligations.
- `DOC1-DELIVER`: require the conflict matrix, interface/finding contract,
  capability and migration maps, manual route, friction/stop-loss metrics, and
  downstream decomposition.
- `DOC1-REVIEW`: require three independent scoped reviews and complete finding
  disposition.
- `DOC1-GATES`: scoped Markdown/path/reference/diff/write-set checks only; no
  TESTGATE, full workspace, CAL, comparator, or Harvard command.
- `DOC1-NOCHILD`: no downstream implementation package may be scaffolded.

### DOC-2 — Governance Authority Alignment

- `DOC2-REMOVE`: remove prospective planner/TESTGATE admission, receipt,
  pre-heavy, CI-lane, repair-prerequisite, and lifecycle authority.
- `DOC2-PRESERVE`: preserve direct requirements, conservative unknown-impact
  handling, exact-diff evidence, assurance transfer, optional quality, and
  campaign/release correctness.
- `DOC2-HISTORY`: preserve immutable generation-17 identity independently and
  freeze the five named historical package statuses.
- `DOC2-GUARDS`: migrate/delete the exact source-coupled guard registrations
  without weakening independent anti-evasion.
- `DOC2-BOUNDARY`: no linter implementation, TESTGATE run, CAL/Harvard action,
  kernel/model/science change, or Order-2 scaffold.

### NKR-1 — OWCMP01 Comparator CLI

- `NKR1-PARALLEL`: implement `tools/owcmp` in parallel; do not delete the legacy
  comparison suite or perform cutover.
- `NKR1-PRESERVE`: preserve PL14S WAT semantic comparison and replay-suite
  behavior.
- `NKR1-SCOPE`: keep changes within comparator tooling, tests, and package
  documentation; no kernel/science behavior change.
- `NKR1-PROOF`: require focused comparator CLI/replay tests, deterministic
  output/error behavior, formatting/lint, and exact diff/write-set checks.
- `NKR1-HANDOFF`: leave canonical-reference retargeting and legacy deletion to
  the separately authorized cutover.

### NKR-2 — Assurance Amendment Clippy Disposition

- `NKR2-ONEALLOW`: permit exactly one function-scoped
  `#[allow(clippy::too_many_lines)]` with adjacent cohesion rationale.
- `NKR2-NOBEHAVIOR`: do not change assurance behavior, fixtures, assertions,
  identities, lifecycle semantics, or science.
- `NKR2-NOBROAD`: reject file/crate/workspace lint allowances and test
  weakening/splitting.
- `NKR2-GATES`: require the exact focused assurance test, strict workspace
  Clippy, full regression, diff/write-set checks, dual reviews, and dual
  verification as declared.
- `NKR2-BOUNDARY`: no gate inventory/executor/ledger, CAL, or Harvard change.

### KER-1 — Native GSI Canopy-Height Coherence

- `KER1-AUTH`: adjudicate/amend the canonical current-day generalized-GSI
  canopy-height/state-ordering law before production correction.
- `KER1-FIX`: correct the centralized production post-phenology projection;
  do not evade the guard through fixture/selector/wrapper changes.
- `KER1-TESTS`: require zero-to-positive transition, negative guard, Lane-D
  active routing, and every named downstream canopy consumer.
- `KER1-NATIVE`: replay the complete frozen 12-case native-proof plan.
- `KER1-CRITICAL`: run contract/source guards and campaign-strength full
  correctness with dual review/verification.
- `KER1-BOUNDARY`: no calibration population, design change, Harvard action, or
  surrogate/proxy physics.

### CAL-1 — CAL-04B

- `CAL1-DESIGN`: preserve the frozen 9,261-vector domain, enumeration,
  objective, accepted-ensemble rule, failure behavior, and no-refinement rule.
- `CAL1-NATIVE`: prove declared parameters reach the real production consumer;
  reconstruction alone is not consumer evidence.
- `CAL1-SYNTH`: synthetic recovery and independent objective reconstruction
  must pass before Hubbard population execution.
- `CAL1-SEPARATE`: keep Hubbard calibration and Harvard independent validation
  separate; Harvard cannot influence selection/refitting.
- `CAL1-CUSTODY`: Harvard remains sealed until a nonempty freeze and two
  independent verifier passes; opening is durable and one-time.
- `CAL1-READINESS`: report implementation, calibration-evidence, and
  identifiability status for every stage; assumed execution axes are not
  observations or calibration.
- `CAL1-HOLD`: the observed hidden-truth crossing failure is a current
  science-design hold; do not plan population execution until it is corrected
  under separate authority.
- `CAL1-NOEXEC`: no candidate population, model, synthetic, freeze/open, or
  Harvard command is executed during planning qualification.
