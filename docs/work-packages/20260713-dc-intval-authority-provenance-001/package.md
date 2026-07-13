# Close INTVAL Required-Authority Fixture Provenance Defect

Status: `HOLD-INTVAL-AUTH-PROV-001`

Package ID: `20260713-dc-intval-authority-provenance-001`

Defect ID: `INTVAL-AUTH-PROV-001`

Execution mode: `package-end-to-end`

## Purpose

Close the required-authority fixture-integrity failure that blocks the exact
INTVAL release command after workspace nextest and dependency policy pass.

## Progress

- [x] Reproduce the fail-closed provenance error in the exact release command.
- [x] Attribute it to missing schema/item metadata in one protected record and
  identify the authoritative fixture-producing commit.
- [x] Verify clean intake at `ed22f37b`, applicable fixture/test guidance, and
  equality of current fixture, lock, provenance, and Git-source hashes.
- [x] (2026-07-13 UTC) Add a narrow regression for schema, unique target item,
  exact source path/commit, and byte identity; static pre/post comparison proves
  its discrimination and focused interactive output is supporting only.
- [x] (2026-07-13 UTC) Correct only provenance metadata; supporting focused
  runs report fixture lock, anti-evasion, AUTH11, and AUTH06 passing.
- [x] (2026-07-13 UTC) Rerun the exact pinned-input release command: provenance
  integrity passed after 1,946/1,946 workspace tests and deny, then the required
  authority lane failed on a deleted active target (`INTVAL-AUTH-BIND-001`).
- [x] (2026-07-13 UTC) Complete dual independent review and fix every accepted
  evidence, guard, and successor-strategy finding.
- [x] (2026-07-13 UTC) Complete dual verification; both verifiers passed the
  corrected HOLD, guard, evidence hierarchy, and iterative finalizer.
- [x] (2026-07-13 UTC) Prepare the terminal HOLD and `INTVAL-FINAL-001` scaffold
  for commit.

## Correction Authority Envelope

Observed failure: `tools/release/run_release_candidate_gates.sh` rejects
`tests/fixtures/constitutive/cas_l4_subhyd_watyld_fcwp_consistency_001/fixtures.provenance.yaml`
because it lacks `schema_version` and per-fixture `source_repo` and
`source_commit` keys.

In-scope writes are that provenance file, a narrow test under
`tests/integration/`, this package, roadmap, and catalog. Allowed correction is
metadata-only: add `schema_version: 1` and bind the fixture item to canonical
source repo `/workdir/openWEPP` and source commit
`9aa4c3d61549ab30da665a4dc109bab811522fe9`. Git history proves that commit
contains the current fixture bytes with locked SHA-256
`a1c50a82cd1e497875cb034481c4b2ef710c319907480b0f584fde30f48fae5e`.
Remove the contradictory legacy top-level `source_repo` and
`source_commit: working-tree-hphys0256` fields when their authority is moved to
the fixture item; do not leave two commit claims.

The fixture JSON, fixture lock, suite document, registry lane/failure posture,
science contracts, production code, release integrity validator, thresholds,
and skip behavior are protected. A provenance correction may not invent an
unverified source or change fixture bytes.

## Surprises And Discoveries

The package-scaffold focused command names the registry-declared HPHYS0227 test,
but that target was deleted by commit `a381702b` while the active required-suite
registry binding remained. The focused command therefore fails before test
execution. Registry posture and deleted authority-test restoration are
protected here; the exact release command determines whether this independent
binding defect blocks terminal acceptance.

Static inventory after the release failure found seven active required suites
bound to five missing test targets. Commit `a381702b` deleted all five together
while their active required registry rows remained. This is one coherent
required-authority binding defect, not five sequential diagnostic relays.

## Decision Log

- Decision: transition the deleted-target family to `INTVAL-AUTH-BIND-001`
  without modifying registry or restoring obsolete symbol-map tests here.
  Rationale: registry/test authority is protected by this metadata-only
  envelope, and the deletion commit removed a runtime surface that must not be
  revived accidentally. Date/Author: 2026-07-13 / Codex.

## Outcomes And Retrospective

The provenance correction is complete: exact release accepted the corrected
schema, source commit, and byte identities after all 1,946 workspace tests and
dependency policy passed. Terminal result remains
`HOLD-INTVAL-AUTH-PROV-001` because required authority then invoked a deleted
target. Remaining authority, binaries, lint, stability, and separate final
gates are blocked; no release or integrated-validation PASS is claimed.

## Plan And Acceptance

Add a source guard requiring `schema_version: 1`, no contradictory top-level
commit claim, the canonical per-item repository, the exact 40-hex source
commit, and equality among the fixture bytes, locked SHA-256, per-item SHA-256,
and Git object bytes at that commit. Record its pre-fix failure, apply the
metadata-only correction, and pass:

    bash tools/release/check_authority_suite_antievasion.sh
    cargo nextest run --test auth06_fixture_provenance_hash_enforcement_contract
    cargo nextest run --test hphys0227_wb19_fcwp_coca_watyld_authority_contract

Then rerun the exact no-skip, pinned-input release command from
`20260713-dc-intval-release-nextest-isolation-001/package.md`. It must exit zero
through full nextest, dependency policy, required authority, binary/staging,
release lint, and both expected stability suites. Run formatting, all-target
Clippy, full nextest, deny, Markdown, and diff gates afterward.

Acceptance requires unchanged fixture/lock hashes, a truthful Git source
commit, all authority guards and the exact release command passing, no skipped
lane, and dual review/verification. The integrated-validation campaign then
restarts in full at the correction commit.

## Subagent Authorization

Subagent requirement: **REQUIRED**. This package explicitly authorizes and
requires a comparator-suite runner for authority, release, stability, and full
gates, plus two independent reviewers and two independent verifiers. Writes are
limited to named artifacts; source changes remain within the envelope above.

## Security Impact Gate

Preserve fail-closed provenance validation, hashes, authority posture, argument
arrays, and release/stability behavior. No network, credential, dependency,
fixture-value, or runtime change is authorized.
