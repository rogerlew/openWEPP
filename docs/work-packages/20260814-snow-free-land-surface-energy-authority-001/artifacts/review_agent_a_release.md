# Release Land-Surface Science And Numerics Review

Evidence class: `Static + Ran`; fresh independent review of the exact current
worktree. Prior review conclusions were not reused as acceptance evidence.

Verdict: **NO-GO / FAIL**.

## Exact reviewed bytes

- LSE definition:
  `5f7ff9640d67ec7c3b747a8c81332a5906920d49cde58432b6f9de709201d8a5`;
- joint canopy-ground core:
  `0e5a7b0e93cd434463c2b4d32e53de762ea5c78026ff47db28ab8d10eca6591e`;
- top-level generator:
  `f08d010e6984c47a64bc51b457b21bea95de0a119d5e11ff414f8815ea45b589`;
- committed vectors:
  `68ebdb09e9344a18fc71c3a284d4f72b345c79e55a4b7b489ee51994eace2744`;
- `SC-LANDSURFACEENERGY-001.md`:
  `321ec01145042a65c0797b75d1ce7a12007abab57f95d5a313cc8e8c41a3578c`;
- `SC-VEGETATION-001.md`:
  `1d7ec3699085fdf5d2f29e01b3c1d76b8a2a5ad8ce22340df2e066cb39f1fb1a`;
- `SC-VEGETATIONTRANSACTION-001.md`:
  `44b77e5d9854ce9c4b64214f2a669e1b09e2fdc5bdb6d1391ef25a909f186e73`.

The reviewed V8 definition digest is
`622bc900a08bd4c70e67c09e1fa113a9de24c48afce3b145a494bb76f6dcbe9b`.
The six schema digests match the LSE definition and fixture manifest.

## Commands run

Ran:

```text
.venv/bin/python .../reference_calculator.py \
  --write /tmp/lse-child1-science-release-review.json
sha256sum /tmp/lse-child1-science-release-review.json \
  .../openwepp_snow_free_lse_v1_vectors.json
cmp -s /tmp/lse-child1-science-release-review.json \
  .../openwepp_snow_free_lse_v1_vectors.json
cargo nextest run --test land_surface_energy_balance_authority_contract \
  --profile quick
```

Regeneration completed in 123 seconds. Both JSON files had SHA-256
`68ebdb09...` and compared byte-identically. The focused Rust gate passed 7/7.
I also executed an independent equality-cap centered-perturbation probe against
the two-rank core and validated each natural numerical failure diagnostic
directly against `lse_v1_diagnostics_schema.json`.

## Confirmed corrections

The exact lower-boundary two-stream solve is now part of the covered coupled
core. Changing VIS/NIR ground albedo changes top reflection, ground absorption,
canopy absorption, surface temperature, and water demand in the same executed
scenario. For the VIS-direct check, canopy absorption changed from
`345.0022270180164` to `373.71832421185104 W m^-2 tile`, top reflection from
`14.46372433588948` to `15.6979770326543 W m^-2 tile`, and ground absorption
from `50.534048646094426` to `20.58369875549496 W m^-2 tile`. This closes the
shortwave defect in `A3-CRITICAL-002`.

The post-ingress operator now closes the complete dry-plus-liquid surface node.
The reviewed vector has pre-ingress surface enthalpy
`465727.3870099428 J m^-2 tile`, ingress `82377.54000000017`, outgoing
infiltration/runoff enthalpy `107922.76055795869`, ending dry-body enthalpy
`66231.10818059284`, ending liquid enthalpy `373951.0582713919`, and residual
`-4.656612873077393e-10 J m^-2 tile`. Ending surface enthalpy equals the dry
plus liquid operands. This closes `A3-CRITICAL-001`.

The mandatory matrix executes the required open, litter, covered, heterogeneous,
storage, advection, authorization, feedback, alternate-start, and rejected-domain
families. The component poison validators now reconstruct radiation, energy,
condensation, ingress, and owner joins rather than using digest inequality alone;
the emitted failures match the attempted semantic counterfactuals. Source rights
and custody also pass: the four vendored GMD PDFs match their declared CC-BY
rights and SHA-256 values, while CLM5 remains in the gitignored restricted area
with checksum `9ca0f0e5...`.

## Material findings

### `A4-HIGH-001` -- The multirank ground-cap active branch is still not frozen

The canonical centered derivative freezes the current water-cap branch, with
the cap value and zero generalized derivative on an active/tie branch.
`_raw_residual()` does set the label from `frozen_branches`, but for a frozen
ground cap it calls `_cap_value()` again (`reference_joint_canopy_core.py:479-483`).
That function reselects the constitutive-law value whenever a perturbation makes
`q_law < cap`. The returned detail can therefore say
`authorization_active_or_tie` while its residual uses the constitutive law.

The multirank wrapper compounds the defect: its surface/soil reconstruction at
lines 870--873 calls `_raw_residual()` without passing any frozen branches.

Ran: using the accepted two-rank full-cap solution, the central ground law was
`0.00015581562600571627` and the cap was
`0.00015581562596770875 kg m^-2 tile s^-1`. A canonical centered temperature
perturbation of `-4.4045415711362716e-6 K` changed the unfrozen branch to
`constitutive_law`. Supplying the frozen map changed the reported branch label
to `authorization_active_or_tie`, but `q_final` remained the law value
`0.00015581538850274556`, not the fixed cap. Thus the purported frozen
evaluation retains a nonzero constitutive derivative exactly where authority
requires the active-cap derivative.

This keeps the active-set portion of `A3-HIGH-003`, `A-CRITICAL-007`, and
`A2-CRITICAL-001` open. The focused accepted cap/tie vectors do not establish
the admitted multirank Newton method until frozen branches determine both the
branch label and value throughout every local and surface/soil residual.

### `A4-HIGH-002` -- Natural failures do not implement the normative diagnostic DTO

The natural singular, backtracking-limit, and iteration-limit trajectories are
genuine solver failures with null candidates and full owner-envelope rollback.
Their diagnostic payloads, however, do not instantiate the checksum-bound
`lse_v1_diagnostics_schema.json` and therefore do not prove the contract's typed
failure surface.

Ran: Draft 2020-12 validation of each of the three `diagnostics` objects produced
72 errors. Each is missing required fields including `model_version`,
`canonical_contract`, `beginning_state_sha256`, `accepted`, `failure_code`,
`failure_kind`, `step_norms`, and `owner_rollback_hashes`. Each also supplies
non-schema aliases such as `state_sha256`, `matrix_norm`, `ci_brackets_pa`, and
`normalized_residuals`. The top-level `typed_failure` is merely `singular`,
`backtracking_limit`, or `iteration_limit`, rather than the schema's typed
`LSEB-E-034` code plus failure kind.

The eight domain/validation failures are still less complete: they contain no
model/configuration/state/transaction/OFE/tile/pass/solve diagnostic object at
all. Equal before/after hash maps prove rollback but do not replace the required
typed diagnostic record. The Rust authority test checks only rejection, null
candidate, and rollback, so it does not detect this mismatch.

This keeps the diagnostic portion of `A3-HIGH-004`, `A2-HIGH-003`, and
`A-CRITICAL-007` open. Every frozen failure must use the normative DTO, preserve
the applicable complete identity and ordered evidence, and itself pass schema
validation.

## Finding reassessment

| Finding family | Release assessment |
|---|---|
| `A-CRITICAL-001`, `A3-CRITICAL-002` | lower-boundary shortwave corrected |
| `A-CRITICAL-002..006`, `A-HIGH-008..011`, `A-MEDIUM-012` | corrected for this science review |
| `A3-CRITICAL-001`, `A2-CRITICAL-002` | complete post-ingress enthalpy corrected |
| `A2-HIGH-004..005` | equilibrium-zero and strict positive schemas corrected |
| `A2-CRITICAL-001`, `A3-HIGH-003` | open because active-cap derivatives remain noncanonical |
| `A-CRITICAL-007`, `A2-HIGH-003`, `A3-HIGH-004` | open because failure diagnostics are not the normative typed DTO |

Both new findings are material and in scope. Neither is deferred or suitable
for follow-up. Child 1 is not eligible for `COMPLETE / snow-free land-surface-
energy implementation authority released` until both are corrected, the exact
fixture is regenerated, and a fresh independent review passes the new bytes.
