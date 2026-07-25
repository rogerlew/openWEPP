# Tooling Defect 02: Repository-Relative Scratch Compatibility

Evidence class: Ran.

Attempt:
`/home/workdir/openWEPP-quality-history/20260724-order3-local-attempt2-NuumUF`.

Admission:
`8a7d0848eaaefc909a950f6a8dbfefb051854f33a8a907fce20f94e8f7c58f3e`.

Full-profile result:

- `2,279` run;
- `2,253` passed, including 8 slow;
- `26` failed;
- `31` skipped;
- elapsed `1,447.898s`;
- Nextest exit `100`; transition exit `2`.

Exact log:
`/home/workdir/openWEPP-quality-history/20260724-order3-local-attempt2-NuumUF/local/nextest-full.log`.

Observed failure: the first correction preserved writable fixture file modes but
still removed write permission from every directory in the execution checkout.
Valid tests require repository-relative, ignored scratch surfaces such as
`target/`, fixture-adjacent tamper files, checkpoint/history roots, diagnostic
outputs, and temporary Git probes. Those writes failed with `PermissionDenied`;
several downstream assertions then failed because their setup had not
completed.

Impact: attempt invalid. `science-manual`, LCOV derivation/merge, adjudicated
CRAP, publication, and terminal verification did not run. No
`quality_evidence_id` or metric-debt counts exist. The published directory is
empty.

Correction boundary: use a writable no-hardlink execution clone and preserve
exact admitted identity through direct index-listed/untracked byte hashing,
source-manifest checks, executable hashes, and checks after each profile and
before/after each LCOV derivation. Temporary writes are allowed only when they
leave the exact admitted checkout identity restored at every boundary.
Snapshot creation rejects any non-owner-writable directory so the same
compatibility defect fails before HEAVY.

Correction verification:

- reviewer A: `PASS`;
- reviewer B: `PASS`;
- Python compilation, collector self-test, focused Nextest `5/5`,
  warnings-denied focused Clippy, and diff hygiene: `PASS`;
- a disposable no-hardlink clone ran 18 affected root integration tests, three
  gate-planner scratch/history tests, and the snowbench fixture-creation test:
  `22/22 PASS`;
- after those tests, the source manifest was restored and the exact working
  identity
  `4f3b16046ba4834324584d6bec37140b79839a98e7d74f5ce3de59fd7bddabd1`
  matched a separately created fresh clone.

Retry policy disposition: tooling corrected and dual-rereviewed. One fresh
one-process `transition` attempt is authorized; neither invalid permission-mode
attempt may be resumed.
