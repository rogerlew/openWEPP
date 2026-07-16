# ASSURE-04D Independent Review A

Status: HOLD — findings dispositioned and remediated before heavy closure

Evidence class: Static + bounded read-only execution

The reviewer independently checked protected hashes, aggregate `usersum/**`,
write-set status, line counts, implementation, schemas, tests, release scripts,
and retained evidence. No scientific-approval judgment was made and no files
were edited.

Findings:

1. Critical: production release verification checked content-address syntax but
   did not reconstruct authorization roots, source lifecycle, principals, or
   the public catalog; empty/forged production containers could pass.
2. High: dropping complete review/publication subtrees made the subject-root
   grammar anti-omission claim false; state/decision/authorization leaves and
   stable finding order were incompletely bound.
3. High: each approval's declared ledger digest was not required to equal the
   calculated finding-ledger root.
4. High: source/staging capabilities were not retained, root ancestry was
   asymmetric, and complete identity rechecks were missing before commit.
5. High: a filename mention passed as the report/model cross-link and the
   frozen reciprocal-link requirement was impossible while tracked usersum
   remained zero-public.
6. High: draft/in-review, forged production, multi-report named/all,
   special-file, fault-boundary, reader, and root/role negatives were
   materially incomplete.
7. Major: the release driver verified but did not retain the v2 snapshot,
   receipt, or a discovery sidecar in release evidence.
8. Closure: `publication.rs` exceeded 2,000 lines and required WARN, while
   `v2.rs` remained too close to the 3,000-line block.

Verdict at review: HOLD.

## Remediation Review

Status: HOLD — second review findings accepted for remediation

The reviewer accepted the authority replay, approval-ledger equality,
field-by-field normalization, corrected backlink contract, release artifact
implementation, and line WARN disposition. The second review retained HOLD
for three closure gaps:

1. lexical ancestry could not detect a bind-mounted descendant, and all four
   held roots were not pairwise rechecked immediately before exchange;
2. the required principal/lifecycle/bound-byte/per-leaf mutation proof matrix
   remained incomplete; and
3. release persistence was source-inspected rather than executed, while test
   release identity was tied to the frozen pre-commit `HEAD`.

The reviewer independently passed protected hashes, aggregate `usersum/**`,
`git diff --check`, and release-script syntax. No files were edited.

## Final Remediation Audit — First Pass

Status: HOLD — semantic-negative specificity incomplete

The reviewer cleared descriptor-recursive mount ancestry, both final root-guard
checks, exact catalog-byte binding, HEAD-derived release identity, executed
materialization, protected/write-set checks, and current focused evidence. HOLD
remained because competence, independence, withdrawal, and supersession tests
asserted only generic failure and could pass from stale-root rejection; an
explicit missing release-transfer negative was absent. No files were edited.

## Final Remediation Audit — PASS

Status: PASS

The reviewer executed the repaired semantic matrix (1/1 PASS; nextest run
`169af646-fa2b-403c-809a-a6128ad3117f`) and confirmed that competence,
independence, withdrawal, supersession, and missing release transfer require
their intended diagnostics without public mutation. The amended parser
dependency/write set, line governance, protected hashes, release scripts,
focused 67/67, and quick 1,956/1,956 evidence also passed. No files were edited
and no scientific-approval judgment was made.

## Post-Heavy-HOLD Bounded Renewal — 2026-07-16 UTC

Status: PASS — Phase 4 review renewed for the strict-Clippy-only remediation

The first Phase 5 heavy result remains the preserved HOLD in
`heavy-gate-runner.md`; this bounded renewal does not relabel it or infer the
unrun full, deny, or adjudicated-CRAP gates. The post-HOLD implementation delta
is confined to `tests/integration/assurance_v2_publication_contract.rs`.
Production Rust line/nonblank/byte tuples remain exact against the heavy
freeze, and every non-document implementation/dependency file predates that
freeze.

The test-only remediation maps directly to the eight Clippy diagnostics:
helpers were extracted, two negative matrices were split into four tests, three
unneeded raw-string hash delimiters were removed, and catalog text now uses
`write!` without an intermediate formatted allocation. No lint suppression,
ignored test, production edit, or semantic relaxation was found. Principal,
lifecycle, release-transfer, and bound-byte negatives retain their required
failure diagnostics and no-public-mutation assertions.

Bounded execution passed:

- `cargo fmt --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- all four split negative tests, 4/4, nextest run
  `427b7d72-7109-45b3-b2f1-756a9d7440ec`; and
- test discovery of 69 focused contracts and 1,958 quick-profile tests.

The package records renewed focused 69/69 run
`84232ff0-2f48-45cb-b647-ab1aa5d49659` and quick 1,958/1,958 run
`d99c842d-8397-45bc-85d9-1d316ff0b4c3`. Protected hashes, aggregate
`usersum/**`, and `git diff --check` also pass. This is engineering review
evidence only; it makes no scientific or human-approval judgment.

## Post-CRAP-HOLD Production Renewal — 2026-07-16 UTC

Status: PASS — bounded production-remediation review renewed

The second Phase 5 heavy result remains the preserved HOLD in
`heavy-gate-runner.md`: full 2,043/2,043 and deny passed, but the adjudicated
CRAP gate reported seven actionable production rows. This review does not
relabel that result or substitute focused evidence for the required fresh
heavy-gate restart.

All seven findings were remediated by bounded decomposition: CLI publication
dispatch and option parsing; report-section and review-lifecycle validation;
ambient path opening; receipt installation; and snapshot-content verification.
Static review found the original ordering, required bindings, duplicate and
unknown-option rejection, state/root fail-closed rules, no-replace receipt
semantics, exact manifest verification, and public `TEST_ONLY` rejection
preserved. No CRAP adjudication, exception, lint/coverage suppression, ignored
test, gate change, or out-of-write-set production edit was found.

Independent execution passed:

- `cargo fmt --check`;
- workspace strict Clippy;
- assurance library tests, 17/17, nextest run
  `a5168606-ca33-4736-a3ca-75a40db1bc4a`;
- the five focused suites, 69/69, nextest run
  `1e0dcd9d-26d4-47c1-9e09-e2fb696cab15`;
- exact CLI duplicate, unknown, selection, publication, release-identity,
  snapshot-directory, and receipt error probes; and
- a fresh retained-publication run, 1/1, nextest run
  `ec2f1ce8-5854-42ac-b501-101033b99dae`.

An independently reproduced workspace LCOV/CRAP diagnostic reported zero rows
above 30 in every touched production file. The largest changed helper was
exactly 30.0 (`publish_selected`); other touched-file maxima were 30.0 or
lower. This is a focused estimate only, not heavy closure. All production files
remain below 3,000 lines; `publication.rs` remains WARN at 2,982. Protected
hashes and aggregate `usersum/**` pass, the adjudication file and CRAP
configuration match the second-HOLD freeze, and fresh retained output is
byte-identical to current output (complete-tree manifest
`a5a4559cea43a55848ddb6ffa7127b840a29de53ff44a1556e492f85a59333f2`).
No scientific-approval judgment was made.
