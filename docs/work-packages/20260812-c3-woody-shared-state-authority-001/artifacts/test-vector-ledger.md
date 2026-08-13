# V4 Test-Vector Ledger

Status: `selected / remediation candidate frozen`

Evidence mode: `Static + Ran`

| Family | Positive authority | Rejected poison |
|---|---|---|
| displayed leaf | exact display C times SLA | total/display+storage/display+transfer C |
| displayed leaf N | exact display N divided by positive displayed LAI | total/display+storage/display+transfer N capacity or Rd |
| zero display | exact zero LAI with nonzero storage/transfer | positive LAI or photosynthetic area |
| derived caches | exact LAI/SAI/RAI equations | independent/mismatched leaf, stem, or root area |
| tissue set | exact six typed identities and three subpools | missing, extra, duplicate, or adjacent tissue |
| schema | exact retained shared fields | either removed offset field or any unknown field |
| serialization | exact typed-line encoding is byte stable for arbitrary UTF-8 IDs and binary64 values | JSON decimal/exponent or insertion-order-dependent digest |
| occupancy identity | structural identity/state array, typed-pair UTF-8 sort, independently length-framed identity components | `stratum@tile` flattening, delimiter split, duplicate structural pair, occupancy swap |
| whole-state preimage | committed production-consumable preimage bytes and SHA-256 | omitted identity component or noncanonical lane order |
| digest | every retained top-level/nested operand changes digest on mutation | excluded shared operand |
| migration | remove exactly two fields, preserve all retained data | synthesis, remapping, or cache repair |

The Python generator uses only the standard library and does not call Rust.
Fixture booleans must all be true and regeneration must be byte-identical. The
whole-state family executes 155 scalar mutations, including both identity
components for both occupancy lanes. The collision fixture proves that two
typed pairs with the same rejected `@` rendering have different canonical
preimages and digests; a separate Greek/CJK/control-character vector proves
arbitrary admitted UTF-8 is length-framed rather than parsed as syntax.
