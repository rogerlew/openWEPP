# Independent Verification B

Evidence class: **Ran** for focused AUTH06, hashes, Git-object, line-count,
Markdown, and diff checks; **Static** for finding disposition, finalizer,
protected-boundary, HOLD, and restart inspection. No heavy gate was rerun.

Status: `PASS-HOLD-VERIFICATION`

Verdict: `PASS`

The corrections for INTVAL-AUTH-PROV-B-01, B-02, B-03, and the verification
line-count finding are satisfied. The current `HOLD-INTVAL-AUTH-PROV-001`
result remains legitimate, and no false release or integrated-validation PASS
is present.

## B-01 verification: verified

`gate-results.md` now distinguishes ID 02's full restart after the in-envelope
format correction from an unchanged retry or retry-until-green. It records both
attempts, their nonzero exits, and the intervening correction. `intake.md` and
`implementation-evidence.md` explicitly label the interactive red/green,
fixture-lock, anti-evasion, AUTH11, and focused AUTH06 reports as unarchived
supporting evidence. They bind terminal provenance-consumer evidence to ID 02,
which ran 1,946/1,946 full-profile tests and advanced through fixture integrity
before the independent missing-target failure.

## B-02 verification: verified

The revised guard identifies exactly one target fixture item, requires exactly
one value for each bound field, rejects legacy top-level source claims, and
asserts the exact repository, lowercase 40-hex commit, and canonical YAML
`source_path`. It uses that extracted `source_path` in `git show`, rather than a
separate hard-coded lookup path, and compares the target item's two digests
against fixture, lock, and Git-object bytes.

Independent reconstruction still gives the same SHA-256 for fixture, lock,
item `sha256`, item `source_sha256`, and Git object:

```text
a1c50a82cd1e497875cb034481c4b2ef710c319907480b0f584fde30f48fae5e
```

Ran:

```text
cargo nextest run --test auth06_fixture_provenance_hash_enforcement_contract
```

Result: 5 passed, 0 skipped, exit 0. This focused verification directly covers
the post-review guard; it does not rerun a heavy workspace or release gate.

## B-03 verification: verified

`INTVAL-FINAL-001` now explicitly supersedes only the original campaign's
separate-successor routing cadence. It preserves every original scenario,
consumer/conservation obligation, fixed-source restart, gate, review, and
verification requirement. The finalizer embeds the literal exact no-skip
release command, pinned forest commit, and both input hashes; requires
verification before every release candidate; preserves each nonzero log; and
restarts the command from the beginning after a correction.

The iterative package retains the cumulative defect/write-set/candidate
ledgers, coherent-batch heavy-gate policy, complete first missing-binding batch,
original full scenario matrix and reconstruction evidence, and Phase 0 restart
at a frozen release-passing candidate. It forbids piecemeal successors and all
named skip, threshold, fixture-result, authority-posture, compatibility,
surrogate-physics, fallback, and retry evasions. Terminal PASS requires exact
release and every integrated row/gate at one source; HOLD remains limited to a
proven external or authority boundary.

## Line-count verification

The final AUTH06 Rust test and `implementation-evidence.md` both report 442
lines. `review-disposition.md` records the corrected count. It remains below
the 2,000-line warning and 3,000-line refactor thresholds.

## HOLD and restart

The exact release remains exit 1 on the protected required-authority binding
defect after accepting the provenance correction. Remaining authority,
binaries, release lint, stability, and separate final gates remain blocked.
Execute `INTVAL-FINAL-001` continuously: close the complete first batch,
continue the same ledger through each in-envelope blocker, pass exact release
from the beginning, freeze the candidate, and rerun integrated validation from
Phase 0. No result from the current failed release may carry terminal
acceptance.
