# ASSURE-05 Terminal Verification B

Status: **PASS — INTERNAL TERMINAL VERIFIER B; PACKAGE REMAINS
HOLD-HUMAN-APPROVAL**

Evidence class: Static + Ran

Verified: `2026-07-16 UTC`

This is an independent coding-agent terminal verification of the final
working-tree source and package disposition. It is not human scientific
review, reproduction/publication approval, assurance-steward approval,
release-owner authorization, or external peer review. This verifier did not
rerun the expensive full-workspace or CRAP gates, did not edit report source or
tests, and did not create approval records. The only write is this artifact.

## Verdict

No closure-blocking technical or governance finding remains against the
bounded ASSURE-05 package. The final identity cascade validates; retained
procedures reproduce the report results; deterministic staging is current;
the publication-fixture compatibility correction preserves fail-closed
production behavior; complete heavy-gate and fresh adjudicated-CRAP evidence
is present; failed attempts remain visible; the protected public surface is
unchanged; and package, catalog, final disposition, and roadmaps agree.

Final internal-verifier verdict: `TERMINAL-INTERNAL-VERIFIER-B-PASS`.

The package's truthful terminal disposition remains
`HOLD-HUMAN-APPROVAL`. Technical verification cannot lift that hold.

## Independent Checks Run

| Check | Result |
| --- | --- |
| Named assurance source validation | PASS — one selected production-domain report, lifecycle `DRAFT`, `fixture_only=false`, zero public reports, report source root `84a8467ff818411a34c89bf825fc2e9280a7c37c50db9b38636fc831546f4d01` |
| Source H2637 procedure against retained manifest/HBP/Parquet | PASS — exact semantic JSON equality with the retained ledger after raw-object authentication |
| Two unrelated narrative-seeded builds and checks | PASS — complete roots `/tmp/assure05-terminal-b-a.p4yxpg` and `/tmp/assure05-terminal-b-b.T3LMGs` were byte-identical |
| Staged analytical procedure | PASS — exact semantic JSON equality with the staged two-day result |
| Staged H2637 procedure | PASS — exact semantic JSON equality with the staged ledger after authenticating staged raw inputs |
| Focused publication boundary checks | PASS — 2/2 in Nextest run `15cb4664-d6d1-403a-9c78-68c1927b6852` |
| Protected public inventory and four frozen hashes | PASS — one neutral public file and all intake hashes unchanged |
| CRAP artifact checksum manifest | PASS — 16/16 retained bundle entries matched |
| CRAP source snapshots | PASS — before, after, and final manifests were byte-identical |

## Identity And Research-Object Cascade

The current cascade is closed and resolves to present bytes:

- `assurance/v2/catalog.yaml` binds the report descriptor at SHA-256
  `47d1e0be95512c865172802eda34db7c3ec60112192abcaf2ad78c6808997a90`;
- the report and packet bind the archived exact prompt at
  `5a740e7f3fa2d4415cc4a82c1f42771a0aa3df6c7f10af09b534e2940fdcf9d8`;
- the packet hashes to
  `752b48c11a68703c8503ddba2db21ccfc79947fddfe885fa386582b1d14b58ac`
  and binds the current manuscript, supplement, protocol, freeze, analytical
  input, and production-evidence identities;
- the accepted H2637 manifest, HBP, and pass-Parquet objects hash to
  `756e324e…`, `378a8c1d…`, and `915f3b99…`, respectively; and
- both terminal-B builds produced build manifest
  `bb95a9c09fde56f141b250e89a9efd4d9328465062a096cf9feafaefcc6ce499`
  and staged the same prompt, packet, and raw-object bytes.

The active-prompt directory contains no execution prompt. The packet's
archived path exists and its digest matches both packet and report bindings.
Named validation independently checked all remaining descriptor dependencies,
results, and research objects.

## Reproduction And Deterministic Consumer

The report no longer depends on transient H2637 acquisition paths. Its staged
research-object set contains the exact accepted manifest, HBP, and
pass-Parquet files plus the standard-library reproduction procedure. The
procedure authenticated the raw file identities and reconstructed the
retained ledger from both source and staged copies.

Supplement S7 explicitly seeds the external staging root with
`usersum/hillslope-hydrology-and-sediment-physics.md` before build/check.
Terminal verification followed that procedure in two new unrelated roots.
Both named builds and checks passed, and `diff -qr` over the complete roots
produced no output. The staged analytical and H2637 procedures then reproduced
their corresponding staged results.

## Publication-Fixture Remediation

The amended publication contract now derives synthetic test-only fixtures from
the converted `1.0.0` production source instead of assuming the retired
`test_only`, `fixture_only: true`, and `0.1.0` literals. It explicitly changes
trust state only inside scratch fixtures, adds the test-only banner there,
uses current versioned paths, and copies the new protocol, freeze, and archived
prompt dependencies.

The terminal full suite passed every publication-contract case. As a light
independent confirmation, verifier B ran the current-source production/test
trust-boundary case and the `DRAFT` nonpublication case; both passed. The real
report remains production-domain `DRAFT`, and no synthetic principal,
approval, publication, snapshot, or public byte was transferred into source.

## Heavy Gates And Failed Attempts

The heavy-gate record is complete and does not rewrite history:

1. Attempt 1 stopped after strict Clippy rejected three exact floating-point
   assertions. Later gates are correctly marked `NOT RUN`.
2. Attempt 2 restarted from formatting, passed Clippy, and stopped after full
   Nextest run `12302dab-2d6d-49fb-bf78-6c641a386a02` reported 2,027 passed and
   22 failed publication-contract tests. Later gates are correctly marked
   `NOT RUN`.
3. Attempt 3 restarted the complete sequence after the package-authorized test
   correction. Formatting, strict Clippy, full Nextest, deny, fresh CRAP,
   assurance validation/plan/build/check, Markdown lint, and diff hygiene all
   passed. Full Nextest run `f7960089-7439-420e-aa3b-293c7fa5d773` passed
   2,049/2,049 tests, with 3 skipped and 4 slow.

The attempt-1 and attempt-2 logs were present at their recorded `/tmp` paths
during this verification, and `heavy-gate-runner.md` durably records their
failure classes, exits, timings, counts, and run identity. Neither failure is
waived or silently converted to a pass.

## Adjudicated CRAP Closure

The retained CRAP report is a fresh, current-source, closure-eligible run over
9,262 production entries at threshold 30. It reports:

- 2 raw rows above 30;
- 2 exact current adjudications;
- 0 actionable rows;
- 0 invalid or stale adjudications; and
- 0 touched production Rust files relative to frozen base `01ed7055`.

The two raw rows remain visible at CRAP 56 and 90. Both match exact registry
entries; neither is suppressed by a wildcard or package-local waiver. The
workspace CRAP and LCOV identities match the heavy-runner record. Source
manifest SHA-256
`5f0446b67c84ecc1606a8adc6527adf75734ab82bda0df7ee62265635f593fcd`
is identical before, after, and at finalization. Thus the test amendments did
not evade the workspace gate: fresh coverage was collected over the whole
workspace even though touched-production-file maximum CRAP is not applicable.

## Protected Public And Human-Authority Boundary

The protected public inventory remains exactly one file:
`usersum/assurance/README.md`. The four frozen identities still match intake:

| Path | SHA-256 |
| --- | --- |
| `usersum/assurance/README.md` | `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70` |
| `assurance/templates/catalog.md` | `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70` |
| `assurance/catalog.yaml` | `cb9cb601739b64f8cb497c0999ca268859946f2ce7e57532de8c08b9ed30801f` |
| `assurance/generated/wepppy-usersum.yaml` | `08b21cbe20ed059eb61f01f9965d8603779501bde90a728bc7a6c138a69258eb` |

The source principal registry contains only `codex-agent-assure05`, kind
`agent`, role `draft_author`. Human report lead and scientific approver are
null. Review is `DRAFT/not_started`, with no subject root, finding ledger,
approvals, or approval lock. Publication is `DRAFT`; release identity,
transfer, public path, and publication date are null; export and vendoring are
false. These records correctly fail closed rather than simulating authority.

## Package And Roadmap Consistency

The package, final disposition, work-package catalog, engine roadmap, and v2
implementation roadmap consistently describe ASSURE-05 as technically
review-ready and held for named human authority. They consistently leave
ASSURE-06 queued until the pilot is human-reviewed and the complete publication
lifecycle is accepted. Recorded final source root, descriptor hash,
build-manifest hash, full-suite count, and CRAP counts agree across the
terminal artifacts.

All observed source changes remain inside the amended declared write set. No
production Rust file differs from the frozen base. The touched publication
contract is 2,108 lines versus 2,109 at the base: this is a nonblocking
line-count `WARN`, remains below the 3,000-line mandatory-refactor threshold,
and did not grow in this package.

This verification is bound to the exact current working-tree bytes. The
report, retained research objects, package evidence, and untracked new test
must all be included unchanged in any eventual commit. Any byte change after
this verdict invalidates the affected identity and requires proportionate
reverification.

## Findings

Closure-blocking findings: **none**.

Nonblocking observations:

- the touched publication contract retains the expected 2,000-line `WARN`, as
  dispositioned above; and
- full heavy-gate logs remain scratch artifacts, while the package retains the
  compact durable attempt record and the complete fresh CRAP evidence bundle.

Terminal internal-verifier B result: **PASS**.

Package/publication result: **HOLD-HUMAN-APPROVAL**.

## TV-A-01 Remediation Renewal — 2026-07-16 UTC

Status: **PASS — VERIFIER-B VERDICT RENEWED**

Evidence class: Static + Ran

Verifier B renewed the exact-tree documentation and identity check after
TV-A-01 was accepted and corrected. The semantic-differences summary now:

- characterizes the positive transfer evidence as separate writer/parser and
  watershed-consumer interface conclusions;
- explicitly excludes continuous nonzero CLI-adapter traversal from the claim
  envelope; and
- records 15 public-safe research objects, matching the 15 declarations in
  the current report descriptor and the final staging evidence.

The finding ledger records TV-A-01 as accepted and resolved. Scoped Markdown
lint and validation passed for both the corrected summary and disposition.
Named assurance validation also passed with the unchanged report source root
`84a8467ff818411a34c89bf825fc2e9280a7c37c50db9b38636fc831546f4d01`,
one production-domain `DRAFT`, and zero public reports.

The documentation-only correction did not disturb the identity cascade. The
descriptor remains SHA-256 `47d1e0be…` and matches the catalog; the agent
packet remains `752b48c1…`; and the archived prompt remains `5a740e7f…`.
Human report lead and scientific approver remain null, review remains
`DRAFT/not_started` with no approvals, publication remains `DRAFT`, and export
and vendoring remain false.

Renewed finding result: **no closure-blocking findings**.

Renewed terminal internal-verifier B result:
`TERMINAL-INTERNAL-VERIFIER-B-PASS`.

Package/publication result remains: **HOLD-HUMAN-APPROVAL**.

## American-English Normalization Renewal — 2026-07-16 UTC

Status: **PASS — VERIFIER-B VERDICT RENEWED ON NORMALIZED SOURCE**

Evidence class: Static + Ran

The user-directed follow-on normalized the report's British unit-spelling
variants to American English, rebound the dependent identities, and updated
the corresponding source-contract mutation fixture. A current search finds no
remaining British unit-spelling variant in the manuscript, supplement, or
report descriptor. No production Rust, scientific formula, result object,
claim boundary, lifecycle state, or approval record changed.

### Renewed Identity And Build Evidence

- Named assurance validation passed with per-report source root
  `08e2b5e3b6444067db7204f790a6670af2d6f16bf1b733879cbc3e95d235dfa6`,
  one production-domain `DRAFT`, and zero public reports.
- The descriptor hashes to
  `64fbfa6756a86bc98a9656235e1c0df5cf06a414806d7291c7ee37fea69cf5d8`
  and matches the catalog binding.
- The manuscript and supplement hash to
  `cd23e31bf0e4c9ce121b18e5da8d072f16cb70c0a5cbf22911043c83091e6c90`
  and `47d1e86a990f636d2f9534fb3153f7c0bb470c0100dd9d0753cb4e20784ecd24`.
- The rebound agent packet hashes to
  `ef191305e56f817d90056091b3ecef7a3d15e4a4ad6ef52987f2529378acfa1f`;
  its descriptor bindings are current. The archived prompt remains unchanged
  at `5a740e7f3fa2d4415cc4a82c1f42771a0aa3df6c7f10af09b534e2940fdcf9d8`.
- Verifier B built and checked two new narrative-seeded roots,
  `/tmp/assure05-terminal-b-us-a.qVFaaS` and
  `/tmp/assure05-terminal-b-us-b.keEWcV`. The complete trees were
  byte-identical and both produced build manifest
  `072c260e71b835f8f2b5005dd0fe3e489171f82d444191407f9b4ba705af45f2`.
- The staged analytical and H2637 procedures reproduced their retained results
  with exact semantic JSON equality. The H2637 path continued to authenticate
  the retained raw objects before reconstruction.

### Proportional Test And Governance Renewal

The recorded affected-assurance sweep passed 59/59 in Nextest run
`9fb5644b…`. Verifier B independently reran the complete source-contract target
after its lexical mutation-fixture update: 12/12 passed in run
`24330e4b-475f-429a-95e0-f927bdc1ab59`.

The sole Rust-test follow-on changes only the literal used to mutate the
normalized unit definition; it preserves the same fail-closed contract path,
test selection, assertions, and production coverage intent. Production Rust
and the CRAP source manifest are unchanged. The prior full-workspace, strict
Clippy, deny, and fresh adjudicated-CRAP conclusions therefore remain
applicable to this bounded prose normalization; no broader heavy-gate claim is
introduced by this renewal.

### Authority And Public Boundary

The protected public inventory still contains only
`usersum/assurance/README.md`, and all four frozen public/template/catalog/
export hashes remain unchanged. Human report lead and scientific approver are
null; review is `DRAFT/not_started` with no approvals; publication is `DRAFT`;
release transfer remains null; and export and vendoring remain false.

Renewed finding result: **no closure-blocking findings**.

Renewed terminal internal-verifier B result:
`TERMINAL-INTERNAL-VERIFIER-B-PASS`.

Package/publication result remains: **HOLD-HUMAN-APPROVAL**.

## TV-A-02 Flaky-Test Remediation Renewal — 2026-07-16 UTC

Status: **PASS — VERIFIER-B VERDICT RENEWED ON FINAL TEST BYTES**

Evidence class: Static + Ran

Verifier B renewed the exact-tree audit after TV-A-02 exposed and corrected a
test-harness observation-window weakness in
`source_drift_after_install_rolls_back_prior_selected_bytes`.

### Scope And Invariant Review

The correction is confined to
`tests/integration/assurance_v2_assembly_contract.rs`, currently SHA-256
`e2006e9039bfca62ea5c09c37180544a7b8dd846518ccc23c98080ff66865eaa`.
Inside a disposable copied source fixture, the test now:

- appends an 8 MiB payload to the scratch implementation dependency and
  refreshes only that fixture's dependent identities, extending the
  post-install rehash interval;
- observes the transient installed-backup marker for up to 60 seconds; and
- retains the original acceptance assertions: the second build must fail with
  `changed during assembly`, and the selected staged tree after rollback must
  equal the prior selected bytes exactly.

The test does not modify the repository implementation dependency or weaken
production assembly behavior. No production Rust file differs from frozen
base `01ed7055`. Package amendment, finding disposition, verifier A, and final
disposition consistently record the correction as test-only and TV-A-02 as
accepted and resolved.

### Renewed Execution Evidence

The contradictory pre-fix observations remain explicit in terminal
verification A under runs `7c7f5666…`, `95f885c8…`, and `6a1a44bf…`; they are
not waived or relabeled. On the corrected bytes:

- the primary proportional rerun recorded consecutive isolated passes
  `0b802e0c…` and `8477b419…`;
- verifier A passed the complete assembly target 9/9 in run
  `28ebb607-16fd-4a91-9178-b3ca61cbe985`;
- strict workspace Clippy and the final five-suite affected set passed 59/59
  in run `c9072e19-566f-489a-96e4-ca65e4262b47`; and
- verifier B independently invoked the exact rollback case twice
  consecutively on the current tree; both commands exited zero. The first
  invocation reported run `730be0d1-5f3f-4879-b363-c82f2c4927bb`.

The correction changes test observability and timing only. It exercises the
same production paths and assertions, removes no coverage obligation, and
introduces no production CRAP scope. The earlier full-workspace pass remains
recorded as the broad closure run; this proportional renewal does not claim a
new full-workspace run after TV-A-02.

### Exact Identity And Governance Boundary

Named source validation passes on the unchanged normalized report root
`08e2b5e3b6444067db7204f790a6670af2d6f16bf1b733879cbc3e95d235dfa6`.
The descriptor remains
`64fbfa6756a86bc98a9656235e1c0df5cf06a414806d7291c7ee37fea69cf5d8`,
matching the catalog, and report/staging identities are unaffected by the
test-only repair. Markdown lint passes for the package amendment, TV-A-02
disposition, final disposition, and verifier-A renewal.

The protected public inventory remains exactly
`usersum/assurance/README.md`, with all four frozen public/template/catalog/
export hashes unchanged. The report and publication records remain `DRAFT`;
human report lead and scientific approver remain null; approvals remain empty;
release transfer remains null; and export and vendoring remain false.

Renewed finding result: **no closure-blocking findings**.

Renewed terminal internal-verifier B result:
`TERMINAL-INTERNAL-VERIFIER-B-PASS`.

Package/publication result remains: **HOLD-HUMAN-APPROVAL**.
