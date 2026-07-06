# HOLD Legitimacy Audit

Status: executed
Evidence mode: Static

## Held Operands

`k_o`, `C_d`, `D_r`, `lambda`, and `h_c` source/timing binding. `I` and `LAI`
have source candidates but remain unwired because the builder cannot close
while required peer operands are unsupported.

## Boundary Named

Source-authority boundary: no canonical `SC-*` text, WEPP input projection, or
operator-approved bounded default maps current runtime inputs to the complete
Papanicolaou friction operand set.

## Evidence

- `laned_shadow.rs` still uses `LANED_SHADOW_KO=500`, bare `CellParameters`,
  and `I=0`.
- `SC-OFEROUTE-001` rev 19 records source candidates and missing operand
  mappings.
- Explorer audits independently recommended HOLD and found no current builder.

## Considered In-Envelope Routes

| Route | Why rejected |
|---|---|
| Ratify all-lane bare `k_o=500`, zero form/wave/vegetation, and source `I` | Would silently model vegetated/rough hillslopes as bare soil and promote a D-val fixture constant to runtime default. |
| Infer `D_r`/`lambda`/`C_d` from residue depth, random roughness, or Chapter-10 hydraulics | No contract maps those symbols to Papanicolaou roughness-element operands; this would be surrogate physics. |
| Wire partial `I`/`LAI` builder and leave missing operands zero | Would create an incomplete active friction surface while pretending every operand was sourced/defaulted/fail-closed. |

## Why D11 Cannot Close Now

D11 can identify candidates, but cannot fabricate the missing authority. The
contract-first correction route requires new source/default reconciliation
before implementation.

First actionable follow-on:

Close `SC-OFEROUTE-001#GAP-OFEROUTE-007` by ratifying a complete per-operand
source/default/fail-closed policy for `k_o`, `C_d`, `D_r`, `lambda`, `LAI`,
`h_c`, and `I`, then implement and test the real active/shadow friction operand
builder.
