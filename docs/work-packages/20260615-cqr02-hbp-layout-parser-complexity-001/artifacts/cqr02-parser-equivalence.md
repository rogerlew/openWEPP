# Parser Equivalence

Status: complete
Evidence mode: Static + Ran

Equivalence checks performed:

- Public HBP parse entrypoints unchanged.
- `parse_layout` remains private to the HBP parser module.
- Header field order, dimension/metadata order, year table order, state registry order, day directory order, schema branch order, footer read order, and checksum windows were preserved.
- Registry duplicate ID is still checked before canonical schema mismatch; the focused suite caught and verified this branch order.
- Schema-1 directory CRC, file CRC, footer count, footer magic, payload offset, and payload truncation details remain typed.
- Schema-2 directory CRC, table CRC, file CRC, footer count/block count, stored/raw block CRC, raw slice overlap/gap/bounds, and day-index details remain typed.

Ran:

- `cargo test --test infile_hbp_parser_contract` -> exit 0, 21 passed.
- `cargo test --workspace` -> exit 0.
