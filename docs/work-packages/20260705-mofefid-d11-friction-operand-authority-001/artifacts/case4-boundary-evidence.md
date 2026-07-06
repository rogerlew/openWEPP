# Case 4 Boundary Evidence

Status: executed
Evidence mode: Static

D11 preserved the D10 boundary.

Boundary checks:

- No Case-4 metric was rerun or tuned by D11.
- No `k_o` scan was accepted as authority.
- No Manning-`n` to `k_o` mapping was introduced.
- `SC-OFEROUTE-001#GAP-OFEROUTE-005` remains the D10 source-authority hold.
- `SC-OFEROUTE-001#GAP-OFEROUTE-007` is held separately on friction operand
  source/default authority.

D10 handoff retained:

The first actionable D10 follow-on remains a source-authority reconciliation
package that binds limiter/CFL/dissipation, lateral-source/boundary handoff,
and Iwagaki friction mapping before production solver/cascade correction.

D11 does not close `INV-OFEROUTE-011`, does not accept Iwagaki Case 4, and does
not authorize activation.
