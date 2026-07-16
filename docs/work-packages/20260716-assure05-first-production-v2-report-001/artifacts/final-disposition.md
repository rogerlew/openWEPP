# ASSURE-05 Final Disposition

Status: HOLD-HUMAN-APPROVAL

Evidence class: Static + Ran

## Outcome

The first production-domain v2 report is technically review-ready, but it is
not scientifically approved or published. The truthful terminal package
disposition is `HOLD-HUMAN-APPROVAL`.

The bounded report source remains version `1.0.0`, lifecycle `DRAFT`, with
`fixture_only=false`. Assurance validation reports source root
`08e2b5e3b6444067db7204f790a6670af2d6f16bf1b733879cbc3e95d235dfa6`.
Its descriptor SHA-256 is
`64fbfa6756a86bc98a9656235e1c0df5cf06a414806d7291c7ee37fea69cf5d8`.
The deterministic staging build-manifest SHA-256 is
`072c260e71b835f8f2b5005dd0fe3e489171f82d444191407f9b4ba705af45f2`.

## Technical And Internal Review Closure

- The analytical procedure and Rust assertion evidence are separately and
  truthfully characterized.
- The exact accepted H2637 manifest, HBP, and pass-Parquet files are durable
  research objects and reproduce the retained ledger semantically.
- Writer/parser and watershed-consumer evidence are bounded as separate
  interface tests. A continuous nonzero CLI-adapter traversal remains explicit
  future work and is not claimed.
- Internal domain-science findings DS-01 through DS-05 and independent
  reproduction/publication findings F1 through F3 are accepted and closed.
- Two unrelated narrative-seeded staging roots build and check byte-identically.
- Terminal full Nextest passed 2,049/2,049 under run
  `f7960089-7439-420e-aa3b-293c7fa5d773`; strict Clippy and deny passed.
- Fresh adjudicated CRAP reported 2 raw, 2 adjudicated, and 0 actionable rows.
  No production Rust file was touched relative to frozen base `01ed7055`.
- The earlier Clippy and full-suite publication-fixture failures are preserved
  in `artifacts/heavy-gate-runner.md`; no failed attempt is hidden or waived.
- Two independent terminal coding-agent verifiers passed the final technical,
  identity, reproduction, protected-public, and governance audit. Their sole
  low-severity archival-summary finding was accepted, corrected, and reverified.
- The user-directed American-English normalization changed only unit spelling
  and dependent identities. The normalized report root is the one recorded
  above; strict Clippy and the final affected assurance sweep passed 59/59 in
  run `c9072e19-566f-489a-96e4-ca65e4262b47`. A timing weakness exposed in the
  rollback test harness was corrected without production-code changes and
  independently reverified.

## Unsatisfied Human Authority

The exact report root has no authenticated, competent, and accountable human:

- report lead;
- independent scientific reviewer;
- independent reproduction/publication reviewer;
- assurance steward; or
- release owner.

Therefore there is no human review subject/finding/approval lock, release
transfer root, publication date, or public path. Agents cannot populate or
infer these records. Internal coding-agent review does not substitute for any
of them.

## Protected Public State

No tracked public report, snapshot, export, or vendor copy was created. The
terminal protected state remains:

| Path | SHA-256 |
| --- | --- |
| `usersum/assurance/README.md` | `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70` |
| `assurance/templates/catalog.md` | `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70` |
| `assurance/catalog.yaml` | `cb9cb601739b64f8cb497c0999ca268859946f2ce7e57532de8c08b9ed30801f` |
| `assurance/generated/wepppy-usersum.yaml` | `08b21cbe20ed059eb61f01f9965d8603779501bde90a728bc7a6c138a69258eb` |

`usersum/assurance` still contains only its neutral `README.md`.

## Human Handoff

Accountable reviewers should evaluate the staged manuscript, supplement,
figures, exact raw H2637 objects, analytical inputs/results, independent
procedure, evidence classification, claim limits, and the declared missing CLI-
adapter traversal. If they approve a later exact source root, the assurance
steward and release owner must complete the schema-governed approval locks,
release transfer, public build/check, immutable snapshot, receipt, catalog, and
narrative cross-link. Until then, publication and ASSURE-06 advancement remain
held.
