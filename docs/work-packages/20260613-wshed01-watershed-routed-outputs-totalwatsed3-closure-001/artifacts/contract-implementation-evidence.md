# Contract Implementation Evidence

Status: W-A executed

Evidence mode: Static

W-A did not modify canonical contracts or production code.

Contract implications captured for W-B/W-C:

- The current impoundment parser has an implicit `jpond >= 1` contract at
  `watershed_impoundment.rs:581-588`; legacy authority shows this is too narrow
  for no-impoundment watersheds.
- W-B must amend parser behavior and tests so `jpond=0` is a typed empty set
  only when structure expects zero impoundments.
- W-C must preserve the watershed runfile output contract:
  `openwepp-watershed-runfile-contract.md:141-163` requires all 14 parquet
  outputs, including `totalwatsed3`.
- The totalwatsed3 schema in `openwepp-watershed-output/src/writers.rs` already
  mirrors the wepppy schema names, but current publication defaults most
  unmapped fields to `0.0`; W-C must replace placeholder publication with
  real routed operands.
