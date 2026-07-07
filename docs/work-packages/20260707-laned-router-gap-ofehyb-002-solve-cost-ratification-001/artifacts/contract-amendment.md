# Contract Amendment

Status: EXECUTED. Evidence mode: Static.

## Amended Contracts

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`
  revision `4`.
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
  revision `35` parent-pointer synchronization.

## Binding Change

`SC-OFEROUTE-002` now authorizes the exact bare-skin branch evaluator under
`INV-OFEHYB-003`:

- exact zero component-absence guards for inactive form/wave/vegetation addends,
- ordinary finite/non-negative `CellParameters` validation before direct
  evaluation,
- unchanged LOW-to-HIGH outer preference and deterministic seed-side branch
  selection,
- no tolerance weakening, no fallback wrapper, no publication ownership/schema
  change, and no selector/default promotion.

The contract explicitly says the optimization is not an active-output
byte-identity promise. The H2637 output delta audit is recorded as sparse
branch-equilibrium numeric dust:

- `H2637.loss.json`: byte-identical.
- `H2637.hbp`: 54 differing bytes.
- `H2637.pass.parquet`: same shape, columns, and index; one `tdet` row changes
  by `3.48e-9` absolute / `1.54e-10` relative; `sedcon_1..5` change on three
  rows each with max relative `3.84e-10`.
- Manifest closure remains machine-precision (`max_day_identity_residual_rel`
  `<= 4.5e-13`; WB13 identity maxima `0.0`).

## Parent Synchronization

`SC-OFEROUTE-001` rev 35 keeps the hybrid rows as child-contract pointers and
now states that parent routing surfaces make no byte-identity claim for active
hybrid outputs. Default/off and routing-surface ownership are unchanged.
