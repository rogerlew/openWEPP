# Disposition

Status: `EXECUTED-COMPLETE-DX5-PRODUCTION-MESH-POLICY`
Evidence mode: Static + Ran.

## Package Decision

Complete. The package promoted the active production Lane D mesh policy to
target `dx = 5.0 m` under `SC-OFEROUTE-001` rev 45 and proved the runtime
default/no-env path consumes that production default.

Shadow mesh remains unchanged and separate. The diagnostic
`OPENWEPP_LANED_ACTIVE_MESH_TARGET_DX_M` override remains explicit and
fail-closed.

## Finding Disposition

### `review-codex.md`

`HIGH - Canonical contract metadata still advertises rev 43`

Accepted and fixed. `SC-OFEROUTE-001` front matter now records
`contract_version: 45`, matching the rev 45 body text, invariant updates, BEI
surface, and revision-history row. Contract/profile checks were rerun and
recorded in `artifacts/gate-results.md`.

`HIGH - Completion is claimed while required closure gates remain open`

Accepted and fixed. `cargo fmt` repaired the formatting failure, `cargo
fmt --check` passed, the full required closure artifact set is now present,
and `package.md` now records
`EXECUTED-COMPLETE-DX5-PRODUCTION-MESH-POLICY` only after the package gates
closed.

`LOW - Required-reading/provenance artifact still names rev 44`

Accepted and fixed. `artifacts/required-reading-map.md` now identifies
`SC-OFEROUTE-001` as rev 45 active mesh-policy authority while retaining the
rev 44 annual sediment replay as an evidence input.

`LOW - Package-local trailing whitespace is present in an untracked artifact`

Accepted and fixed. The trailing whitespace in
`artifacts/verification-comparator.md` was removed, and package-local
whitespace/doc lint gates were rerun.

Residual note: `DirectLanedActiveMeshPolicy::FixedCells` remains as an
orchestrator API variant, but no current runner selector constructs it. This
package does not retire internal diagnostic/API variants.

### `verification-codex.md`

`BLOCKING - Contract metadata still advertises rev 43`

Accepted and fixed by the same contract metadata correction described above.

`BLOCKING - Closure artifacts and gate record are incomplete`

Accepted and fixed. This package now includes `gate-results.md`,
`disposition.md`, `final-disposition.md`, and `worker-handoff.md`. Full
workspace gates, focused Lane D tests, contract checks, runtime evidence, and
review/verification evidence are recorded.

`LOW - Required-reading artifact is stale for rev 45`

Accepted and fixed.

`LOW - Package-local whitespace scan found one trailing-space line`

Accepted and fixed.

### `verification-comparator.md`

Comparator verification passed with no findings requiring disposition. It
confirmed the compact evidence artifacts for active default dx5, active
default-versus-explicit dx5 identity, protected off identity, closure
residuals, and promotion-matrix blockers.

## Authority Changes

`SC-OFEROUTE-001` rev 45 now authorizes:

- active production default `target_dx_m = 5.0`;
- retained `min_cells = 10`;
- retained `max_cells = 4096`;
- retained `LANED_ACTIVE_SAMPLE_DT_S = 900`;
- retained `LANED_ACTIVE_MAX_DT_S = 300`;
- explicit diagnostic target-`dx` override;
- unchanged shadow mesh policy.

## Implementation Changes

The production default in the active router now returns:

```text
TargetDx { target_dx_m: 5.0, min_cells: 10, max_cells: 4096 }
```

Runner default/no-env active configuration consumes that default. Focused
tests pin the 300 m OFE to `60` cells under the production default and retain
the 26 m floor at `10` cells.

## Final Gate State

All required package gates passed. The full workspace gate passed with
`1424` tests passed, `3` skipped.

Cost remains a recorded operational cost, not a promotion blocker:
`dx5/fixed10 = 4.877887788778878` aggregate selected real-cohort user-time
ratio.
