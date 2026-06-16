# CQR36 Behavior Equivalence

Status: complete.

Static: production changes are private extraction only. The original
`parse_impoundment` parse sequence was preserved:

1. three description lines;
2. `ids` and drop spillway branch;
3. culvert 1 and culvert 2;
4. `irf` rockfill branch;
5. `ies` emergency spillway branch;
6. `iff` filter barrier branch;
7. `ipr` perforated riser branch;
8. `misc` storage fields and guards;
9. `isize/ndiv`, `nalpts`, baseline curve, and vectors;
10. final `ImpoundmentRecord` assembly and `StructureFlags`.

Static: newly added private helpers keep the same context labels, field
ordering, branch comment pushes, and payload construction as the original
inline code.

Ran:
`cargo test --test infile_watershed_impoundment_parser_contract`

Result: `22 passed; 0 failed`.

Ran: final workspace LCOV invocation completed successfully and included the
parser contract tests, watershed CLI tests, and watershed runtime unit tests.

Conclusion: behavior equivalence is accepted for CQR36. No public API,
parser-compatibility, typed-error, output-shape, unit, alias, symbol, or runtime
semantics change was introduced.
