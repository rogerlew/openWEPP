# HOLD Legitimacy Audit

Status: `PASS`

Evidence class: **Ran + Static**.

The release-harness correction is implemented and its workspace acceptance
passed. The exact no-skip command then exited 1 because required suite
`cas_l4_subhyd_watyld_fcwp_consistency_001` has an invalid
`fixtures.provenance.yaml`: the record lacks `schema_version` and the fixture
item lacks validator-required `source_repo` and `source_commit` keys.

That provenance file, its fixture, lock, suite authority, registry posture,
and thresholds are explicit protected boundaries in this package. The release
script's fail-closed integrity check is correct and must not be weakened or
skipped. Continuing in-envelope cannot make the protected evidence valid;
serial execution, retry, or bypass would only evade required authority.

The bounded successor `20260713-dc-intval-authority-provenance-001` owns the
provenance-only correction. Git history identifies commit
`9aa4c3d61549ab30da665a4dc109bab811522fe9` as the source of the current fixture
bytes, whose SHA-256 exactly matches the locked value. This is a finite,
authority-owned correction boundary, not diagnostic deferral.
