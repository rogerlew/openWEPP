# Correction And Validation

Evidence class: `Ran` and `Static`

Direct runner SHA-256 is
`b75a06fae6899a05aabb77805933b4466b072a71a58e815430eefcffa0db1a85`.
Both `affected-adjudicated-crap-v1` and `adjudicated-crap-v1` now bind that
value. The JSON diff changes exactly those two strings; commands, risk classes,
prerequisites, outputs, and all other fields are unchanged.

Focused evidence:

- direct SHA/JQ equality: PASS;
- `cargo nextest run --test testgate_align_authority_contract`: 10/10 PASS in
  0.209 seconds; and
- `git diff --check`: PASS.

The mechanical critical terminal plan remains pending.
