# Review Agent A — Terminal Rust Correctness Re-review

Evidence class: `Static exact-commit + Ran exact-commit`

Reviewed commit: `4f50e494cf7757309c94cd1b5cd62bb7cd9c0782`

Verdict: `HOLD / NO-GO`.

Static inspection and the recorded commands used an isolated `git archive` of
the exact reviewed commit. Later shared-worktree commits and in-progress
remediation bytes are excluded.

## Findings

### High — The pre-callback thermal-expectation guard still substitutes the first LSE receiver for a later soil-thermal offender

`validate_receiver_expectations()` combines beginning-LSE digest, LSE-owner,
and ordered thermal-topology failures in one predicate at
`crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/mod.rs:916-920`.
Every failure is then rebuilt from `expectations.lse_owner_id` and
`configuration.records.first()` at lines 921-934. A wrong second or later
`ordered_thermal_layers` row therefore returns `SURFACELIQUID-E-011` with the
LSE owner and the first valid OFE/tile rather than the soil-thermal owner and
the first mismatching row.

This path is public and reachable before the fixed-cap callback:
`UnifiedReceiverExpectations::try_new()` at lines 267-302 requires typed,
nonempty, unique rows but intentionally cannot compare them with the runtime
configuration. The finalization and post-callback receiver validators now use
typed `ReceiverEnvelopeViolation` propagation, but this earlier parallel
topology validator does not. Retaining three separately implemented topology
checks has already produced contract-significant drift in E011 attribution.

Required correction: use one ordered first-mismatch helper at this boundary as
well. A present bad row must retain its actual soil-thermal owner/OFE/tile; a
missing row must retain the exact expected soil-thermal identity. Add a
two-row public-bridge poison whose second independent thermal expectation is
wrong and assert the exact owner, OFE, tile, callback non-execution, and
rollback hashes.

### High — Removing a non-terminal rollback row reports the shifted following owner instead of the missing expected owner

The exact three-owner expectation is correctly ordered
`LandSurfaceEnergy`, `Hydrology`, `SoilThermal` at
`crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/mod.rs:1349-1365`.
However, `validate_rollback_joins()` compares actual and expected arrays only
by the same index at lines 1366-1388. If the first LSE row is removed, the
hydrology row shifts to index zero and is reported by `rollback_violation()` as
the offender; the missing-row branch at lines 1381-1385 is reached only for a
missing suffix. Removing the middle hydrology row similarly reports the
shifted soil-thermal row rather than the missing hydrology owner.

Wrong, reordered, and extra rows correctly retain the first actual typed
owner, and a missing terminal row correctly retains the expected owner.
Missing non-terminal rows do not satisfy the retained disposition claim that
missing rows use the exact expected identity, nor the requested exact
first-offender propagation. This remains fail-closed but makes the canonical
E011 owner context materially false.

Required correction: distinguish deletion from replacement/reordering against
the fixed three-owner sequence, or validate keyed exact membership before
ordered comparison. Add removal poisons for each of the three positions and
assert the expected missing owner; owner-wide rollback rows should continue to
carry typed absence for OFE/tile.

### Low — The retained line-count artifact is twelve lines stale

`artifacts/line-count-governance.md` records
`land_surface_energy_shadow/mod.rs` as 2,521 lines, while the exact reviewed
file is 2,533 lines. The classification and rationale remain valid: the file
is still a dispositioned `WARN` below the mandatory 3,000-line threshold.
Correct the count before terminal disposition so the artifact matches the
exact bytes it claims to describe.

## Confirmed prior-finding disposition

Static exact-commit review confirms that the prior accepted findings outside
the E011 contexts above remain materially closed:

- configuration and state persistence are configuration-bound, canonical,
  digest-sensitive, strict on restart combinations, and unavailable through
  raw root serde; canonical state emission validates before serialization;
- arbitration, resource, ingress, LSE finalization, and unified candidates are
  externally sealed; proportional authorization is re-derived from immutable
  `W0 + D`, and the resource validator retains exact D/A identities and checks
  `0 <= F <= A <= D` before debiting finalized use only;
- signed condensation credits precede capacity overflow, and current ingress
  is unavailable to same-interval authorization;
- ingress uses the one shared production WB14 interval transition, persistent
  continuation, exact tile/source custody, once-only area conversion, and
  independent mass/enthalpy/routing reconstruction;
- actual infiltration mutates only a cloned exact production lane through the
  shared same-pass transition; retained LSE and soil-thermal enthalpy endings
  are independently reconstructed;
- production aggregate soil water includes
  `theta_m + residual_theta * max(depth_m-frozen_depth_m,0)` at
  `land_surface_energy_shadow/mod.rs:1954-1968`, and the focused nonzero-
  residual vector remains present;
- the Child-3 bridge carries exactly the three owners it constructs—LSE,
  hydrology, and soil thermal—and no unverifiable vegetation/BGC rollback
  placeholders; and
- both normal `DirectRunFrame` constructors set `surface_liquid_shadow=None`.
  Repository search found no runner selector, scheduler dispatch, production
  default, publication, or production-state mutation consumer for the unified
  bridge.

No new duplicated constitutive or WB14 arithmetic was found. The remaining
duplication concern is the parallel receiver-topology validation called out in
the first finding because it has caused observable typed-error drift.

## Ran at the exact reviewed commit

Working directory: isolated archive
`/tmp/openwepp-custody-terminal.j9AJKD`; build artifacts used
`CARGO_TARGET_DIR=/home/workdir/openWEPP/target`.

```text
cargo nextest run --profile quick \
  --test surface_liquid_hydrology_custody_authority_contract \
  --test land_surface_energy_real_hydrology_shadow_contract
26 passed / 0 skipped

cargo nextest run -p openwepp-hillslope-orchestrator \
  surface_liquid --profile quick
30 passed / 507 skipped

cargo clippy -p openwepp-hillslope-orchestrator \
  --all-targets -- -D warnings
PASS
```

The passing focused suites do not contain a malformed later independent
thermal-expectation poison or per-position missing-rollback context assertions,
so they cannot close the findings above.

## Residual risk and missing tests

- Add exact-context E011 poisons for a wrong second/later independent thermal
  expectation and for missing rollback rows at positions zero, one, and two.
- Retain the existing second/later finalization-candidate, wrong-owner,
  extra-row, reordered-row, nonfinite receiver, nonzero-residual aggregate,
  byte-identical rollback, restart, D/A/F, serialization, ingress, and
  non-activation vectors after remediation.
- No full-workspace nextest, workspace doctest, or dependency-policy run was
  executed in this bounded terminal review. The package retains a failed
  historical workspace-nextest attempt and therefore still needs applicable
  exact-head critical-boundary evidence or a prospectively legitimate named
  campaign deferral before closure.

## Approval statement

`NO-GO`: exact commit `4f50e494c` is not acceptable for dependency-package
closure. Numerical custody, D/A/F, ingress/restart, serialization, sealing,
receiver reconstruction, the three-owner bridge, nonzero-residual aggregation,
rollback isolation, non-activation, and mandatory line thresholds are otherwise
materially closed, but canonical E011 exact-offender propagation remains wrong
on two reachable envelope shapes and requires correction plus fresh exact-byte
review.
