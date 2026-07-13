# Close INTVAL Required-Authority Fixture Provenance Defect

Status: `QUEUED`

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
- [ ] Add a narrow regression for the required provenance schema.
- [ ] Correct only the provenance metadata and run authority guards.
- [ ] Rerun the exact pinned-input release command through stability.
- [ ] Complete final gates, dual review, dual verification, and disposition.

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
