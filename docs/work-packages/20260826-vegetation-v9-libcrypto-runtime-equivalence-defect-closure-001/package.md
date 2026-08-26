# Close V9 libcrypto runtime-equivalence defect

This Defect-Closure ExecPlan is a living document maintained under
`docs/codex_exec_plans.md` and `docs/defect_closure_execplans.md`.

Status: `EXECUTING`

## Purpose / Big Picture

The protected `OPENWEPP_C3_WOODY_V9` oracle currently rejects the valid
openWEPP development host after an Ubuntu security update changed only the
patch-level bytes of `libcrypto.so.3`. After this package, the immutable V9
generation-host descriptor remains exact historical provenance, while active
verification may admit a different SHA-256 provider object only when the
provider passes independent known-answer tests, every other host binding is
exact, and the complete generated V9 vector is byte-identical to frozen
authority.

## Progress

- [x] (2026-08-26) Reproduced and identified defect `VEG-V9-CRYPTO-001`.
- [x] (2026-08-26) Scaffolded the authorized DC-ExecPlan and evidence set.
- [x] (2026-08-26) Amend `SC-VEGETATION-001` before executable verification changes.
- [x] (2026-08-26) Add contract-derived regression and poison coverage and record the
  expected pre-implementation failure.
- [x] (2026-08-26) Implement the bounded equivalence verifier and update the active guard.
- [ ] Run focused, authority, anti-evasion, and full-workspace validation.
- [ ] Complete dual independent reviews, finding disposition, dual terminal
  verification, closure, commit, and push.

## Correction Authority Envelope

Defect `VEG-V9-CRYPTO-001` is the ordinary, non-skipped failure of
`v9_oracle_successor_is_exactly_bound_and_v8_is_immutable` on a valid host whose
only descriptor mismatch is `/usr/lib/x86_64-linux-gnu/libcrypto.so.3` after
Ubuntu upgraded `libssl3t64` from `3.5.5-1ubuntu3.3` to
`3.5.5-1ubuntu3.4`. The old and new providers produce the same complete frozen
V9 output, but the exact-ELF guard rejects before that evidence can be observed.

The in-scope write set is:

- `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md`;
- `docs/specifications/science-contracts/index.md`;
- `tests/integration/vegetation_boundary_authority_contract.rs`;
- `tests/integration/snow_stage3_terminal_receiver_authority_contract.rs` only
  to reconcile the package-induced V28-to-V29 vegetation registry assertion;
- this package, including one package-local executable equivalence verifier;
- `docs/work-packages/README.md` and `docs/ROADMAP.md` only for lifecycle
  reconciliation if required.

Allowed edits are contract authority for exact-host provenance versus bounded
SHA-256 provider equivalence, contract-derived tests, a nonproduction verifier,
and package evidence. No production Rust, vegetation equation, forcing,
state, configuration, model selector, migration, output, V9 authority byte,
historical descriptor byte, anti-evasion guard, or other science contract may
change.

Acceptance requires all of the following:

1. the historical `.3` runtime still passes exact-host verification;
2. the current `.4` object passes only through the bounded equivalence route;
3. SHA-256 known-answer tests pass before any substitution;
4. all non-`libcrypto.so.3` descriptor bindings remain exact;
5. the exact frozen calculator, descriptor, definition, vectors, and imported
   V8 source retain their existing SHA-256 identities;
6. complete generated stdout equals the frozen V9 vector byte-for-byte;
7. poisons for wrong provider capability, mapped-provider identity mismatch,
   any second runtime mismatch, changed protected bytes, and changed generated
   output fail closed; and
8. the ordinary integration test passes without a host `/usr` mutation,
   package skip, or external-runtime overlay.

## Conversion Rule And Seven-Gate Bar

If the reproducible mechanism lies in this envelope and canonical authority
supports the expected behavior, this package must proceed through contract
amendment, contract-derived tests, a failing pre-implementation gate,
implementation, validation, review, and disposition. It may not stop at an
intermediate diagnosis. Reproduction, mechanism, ownership, authority, safety,
testability, and measurable validation are already satisfied by the August 26
diagnostic evidence and must be reconfirmed in package artifacts.

`HOLD` is exceptional and requires `artifacts/hold-legitimacy-audit.md` naming
an authority or envelope boundary, evidence, the considered in-envelope route,
and why that route cannot close now. Difficulty, edit size, or a partial wrapper
is not a boundary.

## Protected Boundaries

The following four V9 artifacts remain immutable at their existing hashes:

- `reference_calculator_v9.py` at `05cee908...ad5a`;
- `runtime_descriptor.json` at `e0d05e49...951f`;
- `openwepp_c3_woody_v9_definition.json` at `f388aa88...1bcd0`; and
- `openwepp_c3_woody_v9_vectors.json` at `f86770cc...90633`.

No accepted route may simply ignore the descriptor checksum. Equivalence is
limited to the one named SHA-256 provider object, must validate the actual
provider bytes as observed evidence, must prove the provider's SHA-256
capability independently, and must still execute the exact historical
calculator to complete byte equality. Exact-host execution remains valid and
preferred when all descriptor bytes are present.

## Phase Plan

Phase A amends the canonical V9 authority and registry. Phase B adds a
contract-derived active integration test and poison cases, then records the
expected missing-verifier failure before implementation. Phase C implements
the package-local verifier. Phase D runs focused validation, contract admission,
anti-evasion, formatting/lint as applicable, and because this changes a
protected authority lane, exact-clean full-workspace correctness. Phase E
completes dual reviews, finding disposition, dual verification, line-count
governance, final artifacts, commits, and push.

## Validation Intent

Classification: `critical`, because the change alters protected external-oracle
admission and an existing required test. Required execution includes the
focused vegetation integration binary, science-contract admission,
`check_authority_suite_antievasion.sh`,
`auth11_required_suite_obligation_guards_contract`, formatting, warnings-denied
Clippy for affected Rust surfaces, and exact-clean full-workspace Nextest. The
known historical workspace-eleven baseline remains visible and must not be
reclassified by this package.

Assurance impact is `none`: no assurance source, lock, fixture, contract set,
or publication changes. Calibration applicability is `NOT_APPLICABLE`; this is
runtime provenance/verification authority, not process science or calibration.

## Review And Delegation

Two independent authority/implementation reviews and two independent terminal
verifications are mandatory. Every finding is dispositioned as `accepted`,
`rejected`, `deferred`, or `follow-up`; closure is blocked while any finding is
undispositioned.

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two read-only independent reviewer agents, two read-only
terminal verifier agents, and one read-only `comparator_suite_runner` for the
heavy exact-workspace run. Expected outputs are compact findings, exact command
metrics and log paths, and PASS/FAIL verification. Only the primary agent may
edit the declared write set.

## Surprises & Discoveries

- The descriptor checksum maps exactly to Ubuntu
  `libssl3t64_3.5.5-1ubuntu3.3_amd64.deb`; unattended upgrade installed `.3`
  before V9 generation and `.4` on the day the failure appeared.
- CPython `hashlib.sha256` is supplied by `_hashlib`, whose only relevant
  transitive reason for loading `libcrypto.so.3` here is cryptographic hashing,
  not vegetation floating-point calculation.
- A diagnostic that substituted only the stale provider checksum produced the
  exact frozen V9 bytes under `.4`.
- The first exact-workspace candidate exposed one package-induced stale v28
  registry assertion in the terminal-receiver authority test; the historical
  eleven were otherwise unchanged. The assertion was reconciled to v29 without
  changing terminal-receiver behavior or authority.
- The pre-existing full-workspace Clippy surface has unrelated warnings-denied
  debt in biogeochemistry, coupled time, and land-surface energy. The selected
  affected-target Clippy command passes; the broad diagnostic failure remains
  recorded without expanding this package into those production modules.

## Decision Log

- Decision: preserve all historical V9 authority bytes and add a bounded
  verifier rather than rebasing the descriptor to current host bytes.
  Rationale: rebasing would erase provenance and make the next package update
  recreate the same failure. Date/Author: 2026-08-26 / Codex.
- Decision: require full output equality in addition to SHA-256 known answers.
  Rationale: capability evidence alone cannot establish unchanged oracle
  execution. Date/Author: 2026-08-26 / Codex.

## Outcomes & Retrospective

Pending execution.

## Idempotence And Recovery

All verifier operations are read-only and write generated bytes only to stdout
or caller-owned temporary directories. No host library is replaced. Repeating
the validation commands is safe. If execution is interrupted, resume from this
plan and the recorded artifacts; do not reset or rewrite history.
