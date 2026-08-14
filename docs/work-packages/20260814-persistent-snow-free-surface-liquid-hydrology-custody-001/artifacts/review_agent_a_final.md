# Review Agent A — Final Rust Correctness Re-review

Evidence class: `Static exact-commit + Ran exact-commit`  
Reviewed commit: `c0d5da743099a6dc760d5a231236543d0354d967`  
Verdict: `HOLD / NO-GO`.

The worktree advanced during this review. All findings and commands below were
verified against a separate `git archive c0d5da743` export; later remediation
bytes are excluded.

## Findings

### High — The receiver envelope still accepts extra/nonfinite thermal state and self-authored rollback identities

`UnifiedLseFinalization::try_new` checks only tile-pair agreement, nonempty and
unique thermal layer IDs, unique rollback `(kind,owner)` pairs, and unchanged
rollback hashes in
`crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/mod.rs:218-278`.
The later receiver-set check at lines 887-935 requires only that the configured
infiltration layer is first and that layer IDs are unique. It does not require
the exact authoritative layer count/order or validate every layer's numeric
fields. Both closure freezing and unified-candidate validation find and check
only the named infiltration layer at lines 1287-1385 and 450-470. A caller can
therefore append a uniquely named extra layer, including one with nonfinite or
otherwise invalid values, without rejection.

The rollback join at lines 937-980 requires one row for each of five owner
kinds, but only Hydrology is compared with an independently known owner ID and
beginning digest. SoilThermal is compared only with the callback-supplied
candidate, so both can be forged consistently; LSE, Vegetation, and
Biogeochemistry owner IDs/digests are not compared with any independent
beginning envelope. Exact cardinality does not make self-authored identity
authoritative. This violates the exact receiving-layer and complete-owner join
requirements in `SC-SURFACELIQUID-001:132-138`, `442-459`, and `475-481`.

Preferred minimal correction: narrow this Child3 ingress bridge to the owner
set it actually returns or mutates—Hydrology, LSE, and SoilThermal—and supply an
immutable expected beginning-owner envelope for those exact three owners.
Require exact kind/order/cardinality, owner ID, digest, tile, and complete
thermal layer order/numerics against that independent input. Vegetation/BGC
hashes should not be carried as unverifiable evidence when their candidates
are absent. If five-owner LSE atomicity remains in scope, the alternative is to
carry the actual Vegetation/BGC candidates and validate all five against an
independent expected envelope; callback self-consistency is insufficient.

### High — Independent soil closure rejects valid nonzero residual-water production states

`freeze_production_soil_receivers` stores only `theta_m` as beginning/ending
layer liquid plus layer depth at
`crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/mod.rs:1220-1285`.
`validate_real_receiver_closure` then requires the production aggregate to
equal `sum(theta_m)` at lines 1422-1437. The actual production aggregation is
different: `aggregate_direct_soil_water` includes
`residual_theta * max(depth_m - frozen_depth_m, 0)` for every layer in
`crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs:406-416`.

Thus a valid snow-free production lane with nonzero `residual_theta` fails the
new E011 receiver closure even when the ordered infiltration deltas are exactly
correct. The focused fixture masks the defect by hardcoding
`residual_theta: 0.0` at
`tests/integration/land_surface_energy_real_hydrology_shadow_contract.rs:31-50`.
This is contract-significant production-domain drift, not a tolerance issue.

Required correction: freeze residual and frozen-depth operands and reconstruct
the exact production aggregate formula, or validate the aggregate delta using
the unchanged residual/frozen storage while retaining the independent ordered
`theta_m` infiltration reconstruction. Add nonzero-residual, multi-layer
snow-free vectors.

### High — Applicable E004/E007/E011 failure identity remains absent

The new runtime guards make E004, E007, and E011 genuinely reachable, closing
the prior unreachable-variant defect, but their canonical context is still
incomplete. `validate_native_shadow_domain` uses whole-frame `any` predicates
and constructs E004/E007 with only the transaction ID at
`crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/mod.rs:628-675`,
even though the offending lane/OFE is known. `UnifiedLseFinalization::try_new`
returns E011 with transaction only at lines 257-268 even when an offending
tile/thermal/rollback row is available. The receiver wrapper at lines 839-858
discards the validator's offending identity and again emits transaction-only
E011.

`SC-SURFACELIQUID-001:477-481` requires transaction, OFE, tile,
surface/source, and parcel whenever applicable, plus beginning/attempted owner
hashes. Required correction: preserve the first failing row under contract
precedence and construct contextual E004/E007/E011 at that site; do not collapse
it through an identity-free `any` or replace it with a generic envelope error.

## Confirmed closures at `c0d5da743`

Static review confirms these prior findings are closed:

- `DirectSurfaceLiquidArbitration` is externally immutable, its authorization
  batch is exactly re-derived from beginning supply and demand before apply,
  and D/A operands are retained and revalidated in the resource candidate;
- state `canonical_bytes` now requires the configuration and validates the
  complete state/self-digest before serialization;
- ingress, LSE finalization, and unified candidate fields are private with
  read-only accessors; ingress reconstructs from immutable inputs;
- unified validation explicitly checks equality between
  `ending_frame.surface_liquid_shadow` and the ingress ending state;
- ordered production-layer infiltration and named LSE/thermal energy equations
  now have independent frozen-operand validators, subject to the aggregate and
  complete-envelope defects above;
- cadence and WB14 producer failures use E008, candidate failures use E009,
  independent closure uses E010, and actual snow/duplicate-custody/envelope
  guards reach E004/E007/E011;
- candidate work remains clone-only and focused rollback poisons preserve the
  production frame;
- no production selector/default activation or runner consumer was added; and
- all touched Rust files remain below the mandatory 3000-line threshold with
  recorded WARN dispositions for files above 2000 lines.

## Residual risk and missing tests

The focused suites lack:

- extra, reordered, and nonfinite thermal-layer poisons against an independent
  authoritative thermal snapshot;
- forged but internally consistent LSE/SoilThermal/Vegetation/BGC rollback
  owner IDs and digests;
- nonzero `residual_theta`, partial frozen-depth accounting where applicable,
  and multi-layer production aggregates; and
- exact OFE/tile/source context assertions for actual E004/E007/E011 runtime
  triggers.

Ran against the isolated exact-commit export:

- `cargo nextest run --profile quick --test surface_liquid_hydrology_custody_authority_contract --test land_surface_energy_real_hydrology_shadow_contract` — 24 passed;
- `cargo nextest run --profile quick -p openwepp-hillslope-orchestrator -E 'test(/surface_liquid/)'` — 30 passed; and
- `cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings` — PASS.

These passing checks do not contain the poisons above. No full-workspace or
campaign comparator was run for this bounded final review.

## Approval statement

`NO-GO`: exact commit `c0d5da743` retains three high-severity correctness and
contract-envelope findings. It is not acceptable for package closure until
they are remediated and independently re-reviewed at new exact bytes.
