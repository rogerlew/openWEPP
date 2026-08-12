# Contract Test Evidence

Status: `PASS / focused contract and full inventory accepted`

Evidence mode: `Static + Ran`

`tests/integration/vegetation_boundary_authority_contract.rs` now binds the
model version, admitted invariant range, BGC boundary, explicit LUNA rejection,
the digest and every bound canonical section, BGC boundary, explicit LUNA
rejection, and the independently executable reference calculator. The
calculator itself ran successfully on 2026-08-11 and reported all named checks
true. `cargo nextest run --test vegetation_boundary_authority_contract` passed
12/12 after the final fourth-review remediation, including the coupled
gas/hydraulic fixed point, integrated wet/dry canopy ledger, interval-amount
hydraulic caps, and independently parsed turnover residuals. Broader results
are recorded in `gate-results.md`: one clean full-workspace run passed all
2,398 selected tests using compliant external scratch.
