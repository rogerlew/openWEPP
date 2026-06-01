# SOILAUTH01 Contract-Test Implementation Evidence

Status: complete  
Evidence mode: Static + Ran

## Scope
SOILAUTH01 does not author new tests. It validates existing contract tests cover
the compatibility seams being audited.

Static:
- `tests/integration/infile_soil_parser_contract.rs` coverage confirms:
  - strict reject + compatibility accept for policy-first `9002`,
  - strict reject + compatibility accept for quoted headers with omitted `avke`,
  - strict reject + compatibility accept for quoted policy rows,
  - strict reject + compatibility accept for per-OFE restrictive compatibility
    placement.

Ran:
- `cargo test --test infile_soil_parser_contract` -> pass (14/14).
