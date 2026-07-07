# Hold Legitimacy Audit

Status: EXECUTED-HOLD-ROUTE-COEFFICIENT-BRIDGE-AUTHORITY. Evidence mode: Static + Ran.

## Exact Hold Condition

The D16 hybrid route-coefficient authoring bridge cannot be lifted because
neither accepted authority path exists in the current repo/session:

- no source-authored native `ow-lanuse-1` managements with explicit
  `routing_coefficients`; and
- no ratified legacy-to-native bridge mapping named source fields to all five
  static route-coefficient operands.

## Evidence

- Static: `LANUSE-AUTH-3` forbids inferring new-physics operands from legacy
  cropland fields without a ratified bridge contract.
- Static: `SC-INFILE-MANAGEMENT-001` binds the current source-authorized static
  route-coefficient surface to native `ow-lanuse-1` forest/cropland
  `routing_coefficients`.
- Static: `SC-OFEROUTE-001` requires active/activation-candidate paths to
  consume source-authorized operands or fail closed and rejects missing-source
  all-lane defaults.
- Static: D11 evidence found no WEPP-runtime source/default mapping for
  `k_o`, form `C_d`, `D_r`, or `lambda`, and rejected residue/random
  roughness/Chapter-10 inference as surrogate physics.
- Ran: selected external roots currently contain `157` `.man` files and zero
  matches for native datver or `routing_coefficients`.
- Ran: selected external roots contain zero `*.run.toml` active inputs.
- Ran: `cargo test -q --test laned_shadow_h2637 h2637_active_fails_closed_without_routing_coefficients`
  passed, proving active mode still fails closed on missing coefficients.

## In-Envelope Correction Routes Considered

| Route | Decision | Reason |
|---|---|---|
| Import source-authored native `ow-lanuse-1` inputs from selected roots | Blocked | No such files are present in the selected roots. |
| Add route coefficients to legacy managements from row/ridge/random-roughness/residue fields | Rejected | Explicitly forbidden by `LANUSE-AUTH-3` without a bridge and rejected by D11 evidence as surrogate physics. |
| Author bridge from Chapter-10 hydraulics or D-val constants | Rejected | No canonical mapping to all five Papanicolaou operands; fixture constants are not runtime authority. |
| Build an owcmp executable suite with package-local placeholder coefficients | Rejected | Would hide missing input authority and produce a misleading D16 promotion surface. |
| Promote H2637-only coefficient recipe to cohort policy | Rejected | H2637 scratch inputs are timing/regression scaffolding, not selected-cohort input authority. |

## Why This Is Outside The Package Envelope

The package can execute the authority decision but cannot invent the authority.
Lifting the hold requires either external source data or a canonical
source-backed bridge design not present in the current repo/session. Creating
either without source values/provenance would change process input authority
and would violate the no-surrogate-physics rule.

## First Follow-On

Scaffold `D16-HYB-ROUTE-COEFF-SOURCE-ACQUISITION`.

First actionable item: obtain/import source-authored explicit
`ow-lanuse-1` management inputs for the selected D16 cohort with complete
`routing_coefficients` for every active Lane-D lane, or supply primary/source
authority that maps named legacy fields to all five static route coefficients.
The next package should not rerun generic scans first; it should start by
placing or receiving the coefficient source artifact, then parse/project and
active-preflight it.
