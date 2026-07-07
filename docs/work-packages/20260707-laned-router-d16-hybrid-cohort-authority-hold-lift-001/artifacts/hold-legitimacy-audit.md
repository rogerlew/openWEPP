# Hold Legitimacy Audit

Status: EXECUTED-HOLD-ROUTE-COEFFICIENT-AUTHORITY. Evidence mode: Static + Ran.

## Exact Hold Condition

The D16 hybrid cohort-authority hold cannot be lifted because the selected
promotion cohort lacks source-authorized Lane-D route coefficients and no
approved authoring or legacy-field bridge exists in this package.

## Evidence

- Static: `LANUSE-AUTH` and `SC-INFILE-MANAGEMENT-001` make
  `routing_coefficients` explicit native management input, available only under
  `ow-lanuse-1` native forest/cropland records.
- Static: `SC-OFEROUTE-001` requires active or activation-candidate paths to
  consume source-authorized operands or fail closed, and rejects all-lane
  missing-source defaults.
- Ran: selected external roots contain `44 + 40 + 36 = 120` management files,
  with `0` `routing_coefficients` matches and `0` native `ow-lanuse-1` matches.
- Ran: selected external roots contain `0` `*.run.toml` files.
- Ran: all three owcmp manifests pass env checks but `manifest run` exits `1`
  because they are `cohort-inventory` declarations.
- Ran: `cargo test -q --test laned_shadow_h2637 h2637_active_fails_closed_without_routing_coefficients`
  passes, proving the active missing-coefficients guard remains live.

## In-Envelope Correction Routes Considered

1. Patch the external legacy cropland managements with `routing_coefficients`.
   Rejected: no source values exist in the selected roots, and current authority
   forbids inferring them from legacy fields.
2. Reuse the H2637 `500.0 0.0 0.0 0.0 0.0` patch recipe across the cohort.
   Rejected: H2637 timing scaffolding is not broad production-cohort authority,
   and `SC-OFEROUTE-001` explicitly rejects missing-source all-lane defaults for
   activation-candidate claims.
3. Add an owcmp executable suite around the inventory roots anyway. Rejected:
   without active-runnable inputs, the suite would be either non-executable or
   would hide surrogate input authority.
4. Amend `SC-OFEROUTE-002` tolerance/default-promotion posture from current
   H2637 evidence. Rejected: the prior D16 package already proved H2637-only
   evidence is insufficient.

## Why This Is Outside The Package Envelope

Lifting the hold requires one of two authority-producing actions that this
package did not own:

- operator/source-authored native `ow-lanuse-1` management sidecars for the
  selected cohort, including explicit route coefficients; or
- a new bridge contract and implementation that authorizes a particular legacy
  field to native route-coefficient mapping with tests, provenance, and review.

Both actions change input authority, not merely package evidence. They require
a dedicated contract/input-authoring package before comparator suite work.

## First Follow-On

Scaffold `D16-HYB-ROUTE-COEFF-AUTHORING-BRIDGE`.

First actionable item: land source-authorized native route-coefficient inputs
for the selected cohort, either by importing operator-authored `ow-lanuse-1`
managements or by contract-first authorizing a legacy-to-native bridge. Only
after that should an executable active plain-vs-hybrid owcmp suite be added.
