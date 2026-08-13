# Gate Results

Status: `complete`

Evidence mode: `Ran`

## Intake

- `git rev-parse HEAD` -> `9a154596d258e43aab7ec51dd7aa6b27f22ac1f3`.
- `git status --short --branch` -> `main...origin/main [ahead 11]`, clean at intake.
- instruction discovery identified root, work-package, and science-contract
  `AGENTS.md` chains.
- protected definition SHA-256/byte counts:
  - V1 `003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157`, 2,860 bytes;
  - V2 `38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3`, 5,067 bytes;
  - V3 `7768657ca3d03603b66f5cd6677f032ee630fdd46d6ffadf214c713065f73852`, 8,205 bytes.

Subsequent failed and successful commands are appended below; no failed
evidence is replaced.

## Authority and independent vectors

- `.venv/bin/python .../reference_calculator.py` -> PASS; all 21 fixture
  checks true, 59 shared-state scalar leaves and 151 whole-state scalar leaves
  independently affect their canonical digests.
- Fixture SHA-256 ->
  `6862b507cf54b57606304d4a7b01cffe55dd3f90b2a2b0d44601fe103e2841a7`.
- Generator SHA-256 ->
  `5ac8dfea31270a7cd7e213e29ffff9efc7cde8bb5e9333aa69add5100b0872c3`.
- Canonical V4 definition SHA-256 ->
  `571bac78b6f116078b463021ec0a36a5206cbe14a94d9fdc76bc32c0a7cde327`.

## Focused authority gates

- `markdown-doc lint --path <package>` -> PASS, 16 files, 0 errors/warnings.
- `check_sc_unit_compliance.sh --path SC-VEGETATION-001.md` -> PASS.
- `check_science_contract_admission.sh --base-ref 9a154596... --worktree`
  -> PASS, `A0_ADMITTED contracts=45 science_surfaces=0`, authority SHA-256
  `bc04dd6ebf51f17f03e78067e1cf5512724d4e065b11907a3a62b6d97e9e878c`.
- `check_authority_suite_antievasion.sh` -> PASS.
- scoped `git diff --check` -> PASS.

## Preserved review failure

Reviewer B's first exact-byte review returned HOLD for canonical-digest/oracle
disagreement, ambiguous displayed leaf N, non-fail-closed fixture checks,
tautological migration/digest evidence, and insufficient multi-owner rejection.
Reviewer A additionally found an undefined cache tolerance, ambiguous encoding,
incomplete predecessor identity validation, and zero-C/positive-displayed-N
fixture conflict. All are accepted. Remediation is recorded in
`review-finding-disposition.md`; final rereviews are pending.

## Focused final reruns

- V4 generator regeneration -> PASS, byte-identical, all 21 checks true.
- `cargo nextest run --test vegetation_boundary_authority_contract --profile quick`
  -> initial post-V8 attempts failed because the historical V3 test bound live
  section digests and the registry string; failures preserved above in command
  output history. Contract placement/registry text were corrected without
  editing the protected V3 definition or tests. Final rerun -> PASS, 17/17.
- `cargo nextest run --test auth11_required_suite_obligation_guards_contract`
  -> PASS, 3/3.
- final `check_sc_unit_compliance.sh` -> PASS.
- final science-contract admission -> PASS, `A0_ADMITTED contracts=45`,
  authority SHA-256
  `305eb77d689f6b6872a3e6f88b158ed7d8551b1651c97582d1a7368ed9e77e3d`.
- final package Markdown lint -> PASS, 18 files, 0 errors/warnings.
- both independent science rereviews -> GO with no unresolved material finding.

## Heavy comparator attempt

Comparator artifacts:
`artifacts/v4-closure-20260812-231331/`.

- `cargo clippy --workspace --all-targets -- -D warnings` -> PASS.
- `cargo nextest run --workspace --profile full` -> FAIL before execution with
  Rust `E0063` in concurrently developed implementation code:
  `occupancy_solver/potential.rs` initializers lacked `active_water_caps`.
- doctests, deny, fmt, and diff check -> NOT RUN because the comparator stopped
  at the first hard failure.

The failing Rust files are explicitly outside this authority package's write
set and were concurrently modified by the parent implementation campaign. Under
the non-deferral rule, the package cannot claim terminal completion until a
fresh exact-byte heavy run passes after that implementation compile defect is
corrected by its owner.

## Final-definition regeneration correction and focused rerun

- A terminal verifier correctly rejected the first generator because it
  canonicalized an existing definition without independently guarding all
  static definition content or refreshing all adjacent contract bindings.
- The generator now rejects any static semantic drift by a fixed canonical
  static-definition digest, recomputes all six live vegetation section
  bindings and all four adjacent contract file bindings, and deterministically
  rewrites both the fixture and definition. Two consecutive regenerations were
  byte-identical.
- Final frozen identities are definition
  `c11f5406290b7c99db3858bfe858d996ab9de24bcdb5f2bbbe3f8af19610f830`,
  fixture `a815e018b7aec05c9fa46ce62ed713cb809088852feb1ecf29b33f2b6759676a`,
  and generator
  `0e918763e274cae8317425cfdf03c7dcc838e85c537f08fbf7fa0e7fcc0b4ad6`.
- `check_science_contract_admission.sh --base-ref 4f5bb1c5... --worktree`
  -> PASS, `A0_ADMITTED contracts=45 science_surfaces=0`, authority SHA-256
  `24a14c6651f22b8eac384425572d7cd3ab9aee0ffb224e64a35b001b0413cbc3`.
- `check_sc_unit_compliance.sh --path SC-VEGETATION-001.md` -> PASS.
- `check_authority_suite_antievasion.sh` -> PASS.
- `cargo nextest run --test vegetation_boundary_authority_contract --profile quick`
  -> PASS, 17/17.
- `cargo nextest run --test auth11_required_suite_obligation_guards_contract`
  -> PASS, 3/3.
- `git diff --check` -> PASS.

## Accepted typed pending-transfer rereview finding

- Reviewer B returned HOLD after finding that the positive fixture's
  `litter_metabolic` receiver was outside the imported typed receiver enum and
  that the oracle accepted arbitrary donor/receiver strings plus zero
  transaction/proposal identities.
- The finding was accepted. The fixture now uses `metabolic`; validation binds
  the exact six donor and four receiver serde identities and positive nonzero
  `u128` transaction/proposal IDs. Unsupported donor, unsupported receiver,
  zero transaction, and zero proposal poisons all return
  `VEG-E-087 invalid_pending_transfer`.
- Two consecutive regenerations produced byte-identical final candidates:
  definition
  `c11f5406290b7c99db3858bfe858d996ab9de24bcdb5f2bbbe3f8af19610f830`,
  fixture `a815e018b7aec05c9fa46ce62ed713cb809088852feb1ecf29b33f2b6759676a`,
  and generator
  `0e918763e274cae8317425cfdf03c7dcc838e85c537f08fbf7fa0e7fcc0b4ad6`.
- Final focused rerun: science-contract admission PASS with authority SHA-256
  `93bcf9d0f7367e4d8736234821972ab31d02a7249329e13a454d1c8260c6875b`;
  SC unit compliance PASS; anti-evasion PASS; A0 authority contract 17/17
  PASS; AUTH11 obligation guard 3/3 PASS; diff hygiene PASS.

## Canonical typed pending-transfer authority correction

- Reviewer A correctly returned HOLD because the first pending-transfer fix
  lived only in package evidence. The V8 amendment now canonically enumerates
  the six donor and four receiver serde identities, requires positive nonzero
  `transaction_id: u128` and `proposal_id: u64`, and binds owner and amount
  domains. The V4 definition carries the same exact sub-schema.
- The oracle now validates proposal IDs against the authoritative positive
  `u64` domain rather than inventing `u128` authority.
- Two consecutive regenerations are byte-identical at definition
  `8e83d202ec2a0d98e25ffde27397b90a5d6fff190ee91128db1a396c7b1fa1ac`,
  fixture `a815e018b7aec05c9fa46ce62ed713cb809088852feb1ecf29b33f2b6759676a`,
  and generator
  `a2305bd148ea65d8994721f4284f47d71661510cd1d9cbbf6d93e3b9aec65083`.
- Post-correction focused rerun: admission PASS with authority SHA-256
  `8a95dd6f6fe0ef2f7179253bec4dcef774e948e15114198b0ba4fb9f54030528`;
  unit compliance PASS; anti-evasion PASS; A0 17/17 PASS; AUTH11 3/3 PASS.

## Exact-width poison correction

- Reviewer B found that the exact integer-width validators lacked upper-bound
  fixture poisons. The accepted correction adds `transaction_id = 2^128` and
  `proposal_id = 2^64` cases; both reject with
  `VEG-E-087 invalid_pending_transfer` and enter the all-poisons check.
- Two consecutive final regenerations are byte-identical at definition
  `571bac78b6f116078b463021ec0a36a5206cbe14a94d9fdc76bc32c0a7cde327`,
  fixture `6862b507cf54b57606304d4a7b01cffe55dd3f90b2a2b0d44601fe103e2841a7`,
  and generator
  `5ac8dfea31270a7cd7e213e29ffff9efc7cde8bb5e9333aa69add5100b0872c3`.
- Final focused rerun: admission PASS with authority SHA-256
  `796cdfa8672b44d60dd1e1f237ceeeb8dda0de201311a23c502ab6f32b247e36`;
  unit compliance PASS; anti-evasion PASS; A0 17/17 PASS; AUTH11 3/3 PASS.
- Final independent science rereviews A and B -> GO, no unresolved material
  findings, against definition `571bac78...`, fixture `6862b507...`, and
  generator `5ac8dfea...`.

## Final-heavy interrupted attempt

Comparator artifacts:
`artifacts/v4-closure-final-stable-20260812-233641/`.

- Workspace Clippy, doctests, deny, formatting, and diff hygiene all passed.
- Full nextest was externally interrupted after 1,496.718 seconds: 227 tests
  passed, two still-running tests received `SIGINT`, 33 were skipped, and 2,328
  were not run. The two rc=100 entries are interrupt results, not assertion
  failures. The exact log and summary are preserved.
- Because an interrupted campaign cannot close the full-workspace gate, a fresh
  uninterrupted retry is active; no authority or science bytes changed.

## Final-heavy ENOSPC attempt

Comparator artifacts:
`artifacts/v4-closure-final-stable-20260813-000317/`.

- Workspace Clippy passed.
- Full nextest encountered `No space left on device (os error 28)` while nested
  assurance quality-verification builds wrote under the comparator-selected
  `/tmp/openwepp-v4-shared-state-final-2`; the wrapper then aborted before the
  remaining commands. This is infrastructure failure, not an assertion or
  source failure. Exact logs are preserved.
- After the process terminated, only the two exact comparator-owned ordinary
  directories `/tmp/openwepp-v4-shared-state-final` and
  `/tmp/openwepp-v4-shared-state-final-2` were validated and deleted. They were
  disposable campaign temp output and are not recoverable. `/home` retained
  24 TiB free.
- Exactly one final retry is running with a unique absolute `TMPDIR` created
  under `/home/workdir`; authority bytes remain unchanged.

## Final capacity-correct heavy PASS

Comparator artifacts:
`artifacts/v4-closure-final-stable-20260813-004136/`.

The retry used
`/home/workdir/openwepp-v4-shared-state-tmp.fML7Mp`, created by `mktemp`, with
24 TiB available on `/home`. All six commands completed in one uninterrupted
campaign:

- workspace warnings-denied Clippy -> PASS, 3 seconds;
- full-workspace nextest -> PASS, 3,217 seconds;
- workspace doctests -> PASS, 8 seconds;
- `cargo deny check` -> PASS (one non-fatal unmatched `MIT-0` allowance
  warning), under 1 second;
- formatting -> PASS, 3 seconds;
- diff hygiene -> PASS, 1 second.

`results.csv`, per-command logs, fingerprint, command log, and JSON/Markdown
summaries are preserved in the run directory. Final authority identities remain
definition `571bac78...`, fixture `6862b507...`, generator `5ac8dfea...`.

## Terminal verification and archive

- Independent terminal verifier A -> PASS, no unresolved finding.
- Independent terminal verifier B -> PASS, no unresolved finding.
- Both verified exact definition `571bac78...`, fixture `6862b507...`,
  generator `5ac8dfea...`, protected predecessor identities, dual review GO,
  final heavy 6/6 PASS, bounded claims, and honest GAP-027 fail-closed posture.
- After both PASS results, the active kickoff prompt was archived byte-for-byte;
  source and archived SHA-256 are
  `7f31e3a82634aaab31aa9de2d4bf5ac9bfd34c11241671fb3a80685b6839df25`.
