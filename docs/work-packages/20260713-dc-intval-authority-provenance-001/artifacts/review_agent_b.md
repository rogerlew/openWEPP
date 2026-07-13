# Independent Review B

Evidence class: **Ran** for lightweight log replay, hashes, Git-object/history,
inventory, line-count, Markdown, and diff checks; **Static** for source-guard,
protected-boundary, HOLD, roadmap/catalog, and iterative-successor inspection.
No heavy gate was rerun.

Verdict: `HOLD-VALID-WITH-FINDINGS`

The `HOLD-INTVAL-AUTH-PROV-001` disposition is legitimate. The provenance
correction is bounded and accepted by the real release integrity consumer; the
exact release then fails on an independent protected authority-binding defect.
The replacement `INTVAL-FINAL-001` strategy is materially better than another
single-gate successor and has a coherent first batch. Three accepted-fix
evidence/guard/strategy findings prevent terminal review closure.

## Command and release-log audit

The archived command records reconcile as follows:

| ID | Result | Exit | Elapsed | Peak RSS (KiB) |
| --- | --- | ---: | ---: | ---: |
| 00 | pinned forest commit and cohort/watchlist hashes verified | 0 | 0:00.02 | 3,840 |
| 01 | exact release stopped on formatting of the new guard | 1 | 0:02.13 | 70,272 |
| 02 | exact release stopped on missing required AUTH05 target | 1 | 9:44.72 | 209,340 |

Both release time records contain the exact no-skip invocation with the pinned
cohort/watchlist paths and expectations `wb05b_1166=1166` and
`release_gate_watchlist=19`. ID 01 preserves the formatting diff. After the
in-envelope format correction, ID 02 restarts the command from the beginning,
runs 1,946 tests across 175 binaries, reports 1,946 passed and three skipped,
passes all four dependency-policy classes, and advances through fixture
integrity. It then invokes
`auth05_level4_constitutive_authority_hardening_contract`; Cargo reports that
the target does not exist, and the required hard-fail lane exits 1.

Remaining required authority, binary build/staging/sidecars, release lint,
stability, and separate final gates are correctly `BLOCKED`. Neither the
1,946/1,946 workspace result nor provenance acceptance is represented as a
release or integrated-validation PASS.

## Provenance correction and guard audit

Independent reconstruction gives one SHA-256 value for the current fixture,
lock entry, provenance `sha256`, provenance `source_sha256`, and the same path
at Git commit `9aa4c3d61549ab30da665a4dc109bab811522fe9`:

```text
a1c50a82cd1e497875cb034481c4b2ef710c319907480b0f584fde30f48fae5e
```

The metadata diff adds `schema_version: 1`, removes the contradictory legacy
top-level source fields, and adds the canonical repository plus exact 40-hex
commit to the fixture item. The fixture JSON and lock have no worktree diff.
The release integrity consumer proceeding past provenance in ID 02 is direct
acceptance evidence.

The new regression checks schema version, absence of top-level source claims,
the exact per-item repository/commit, fixture-to-lock equality, provenance hash
lines, Git object type, and Git-object bytes. The touched Rust test is 400
lines, below both governance thresholds. No production crate, science contract,
registry row/posture, release validator, fixture JSON/lock, threshold, skip
behavior, or dependency was edited.

## Required-binding inventory and deletion provenance

Static inventory finds nine active required suites total. Two resolve to
present AUTH07 and SOILAUTH03 targets. The other seven bind exactly five absent
targets:

| Active required suites | Missing target |
| --- | --- |
| FC, WP, WATBAL | `auth05_level4_constitutive_authority_hardening_contract` |
| withdrawal soil-water cap | `hphys0224_wb19_withdrawal_soilwater_cap_contract` |
| layer-pool withdrawal cap | `hphys0225_wb19_layer_pool_withdrawal_cap_contract` |
| saturated-thickness response | `hphys0226_wb19_lateral_saturated_thickness_response_contract` |
| FC/WP + COCA water yield | `hphys0227_wb19_fcwp_coca_watyld_authority_contract` |

Commit `a381702beca580fa10e71456a897f1a6a705a968` deletes all five files and their
Cargo registrations as part of symbol-map kernel-boundary removal. The active
registry bindings remain. ID 02 directly exposes the first missing target;
source inventory establishes the coherent seven-suite/five-target family.

## HOLD and iterative-finalization audit

Registry/test restoration is outside this metadata-only package and must not be
accomplished by deactivating suites, weakening required/hard-fail posture, or
reviving the deleted symbol-map runtime. `INTVAL-FINAL-001` properly begins
with all five targets as one batch, uses deleted tests only as assertion
provenance, and requires mapping to current public typed/kernel authority. Its
cumulative defect ledger, per-mechanism red/green rule, intended-write-set
revision, coherent-candidate heavy-gate cadence, protected boundaries,
candidate freeze, exact release restart, and full Phase 0 integrated restart
form a viable continuing campaign rather than a diagnostic relay.

The final package correctly permits HOLD only at a proven external/authority
boundary and forbids effort-based HOLD, one-gate successors, skips, tolerance
loosening, fixture-result edits, retry-until-green, compatibility wrappers,
surrogate physics, suite deactivation, and weaker failure posture. Roadmap and
catalog route the executed provenance HOLD into this iterative package and
retain exact release plus full integrated-campaign acceptance.

## Findings

### INTVAL-AUTH-PROV-B-01 — accepted fix required: command and focused evidence wording

`gate-results.md` says “No skip, retry, waiver, or source edit was used” next to
a table containing two release attempts. ID 02 is a legitimate full restart
after correcting ID 01's formatting defect, not a blind or unchanged retry, but
the absolute “no retry” statement is false. Replace it with “no unchanged
retry/retry-until-green” and explicitly bind the format correction between
attempts.

The package and `implementation-evidence.md` also state that the provenance
guard failed before correction and AUTH06 passed 5/5, while the artifact logs
contain only IDs 00 through 02. `intake.md` identifies interactive focused
output as summarized, but the terminal evidence distinction is not carried
through every claim. Qualify unarchived focused results as supporting only and
bind terminal provenance acceptance to ID 02's real fixture-integrity consumer.

### INTVAL-AUTH-PROV-B-02 — accepted fix required: guard ignores provenance `source_path`

The regression hashes a hard-coded Git path but never reads or asserts the
fixture item's YAML `source_path`. The release validator requires that field to
be nonempty but does not verify its value. Consequently, a wrong nonempty
`source_path` can pass both guards while the regression proves bytes from a
different hard-coded path. Parse or otherwise bind all provenance fields to the
target fixture item, require the exact canonical `source_path`, and use that
verified field for `git show`. Also reject duplicate target-item ambiguity.

### INTVAL-AUTH-PROV-B-03 — accepted fix required: finalization precedence and exact command must be autonomous

The original integrated package says semantic failure opens a separate
DC-ExecPlan, while `INTVAL-FINAL-001` says every correctable release/integrated
blocker stays in one package. The user-directed finalization strategy must state
explicitly that it supersedes only the original package's separate-successor
routing during this restart, while preserving all original scenario,
consumer/reconstruction, review, verification, and terminal gate obligations.
Without that precedence rule, an autonomous worker receives contradictory
instructions at the first new semantic defect.

For the same autonomy reason, copy the literal exact pinned-input release
command into `INTVAL-FINAL-001` rather than relying only on a pointer to
`INTVAL-REL-001`. Retain the cumulative command/candidate ledger and require
every in-repository blocker to be dispositioned there; no intermediate package
may be scaffolded.

## Restart disposition

No partial evidence carries forward. After these findings are corrected,
`INTVAL-FINAL-001` must restore the complete first batch, continue its ledger
through every in-envelope blocker, make the literal exact release command pass
from the beginning, freeze that candidate, and rerun the complete integrated
campaign from Phase 0. Terminal success requires one-source
`PASS-INTEGRATED-VALIDATION`; otherwise only a proven external/authority HOLD is
legitimate.
