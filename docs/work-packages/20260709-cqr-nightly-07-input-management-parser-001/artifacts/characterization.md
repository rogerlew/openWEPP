# Characterization

Evidence label: Static/Ran.

Status: `EXECUTED`

## Behavior Oracle

Static:

- Target module:
  `crates/openwepp-input-contract/src/parsers/management.rs`.
- Characterization lives in:
  `tests/integration/infile_management_parser_contract.rs`.
- Native YAML characterization lives in:
  `tests/integration/infile_management_yaml_contract.rs`.
- The package preserves public parser APIs and existing output model shape:
  `parse_management_from_str`, `parse_management_from_path`, and
  `parse_management_document_from_path` remain the parser entry points.

## Added Characterization

Static:

- Error display coverage for every `ManagementParseError` variant verifies that
  display strings still begin with stable contract error IDs.
- Operation parsing coverage now exercises:
  - `pcode=3` with valid `cltpos`;
  - missing and invalid `cltpos` failures;
  - extension-line capture for extension-reading operation codes.
- Contour coverage now exercises:
  - legacy `98.4` four-value contour records;
  - modern `2016.3` five-value `contours_perm` records;
  - legacy rejection of `contours_perm`.
- Initial-condition coverage now exercises:
  - cropland terminal line with optional understory values;
  - invalid `imngmt`;
  - invalid `rtyp`.
- Drain coverage now exercises declared drainage scenarios and yearly `drset`
  references.
- Native YAML coverage now exercises all annual residue-management extension
  variants: herbicide, burn, silage, cut, and remove. The same fixture also
  asserts one native cropland operation, surface, contour, and drain scenario.

## Focused Runs

Ran:

| Command | Result | Evidence |
|---|---|---|
| `cargo nextest run --test infile_management_parser_contract` | PASS, exit `0`; `45` tests passed | parent shell, 2026-07-09 |
| `cargo nextest run --test infile_management_yaml_contract` | PASS, exit `0`; `2` tests passed | parent shell, 2026-07-09 |

Disposition:

- Characterization was added before and alongside decomposition, matching the
  CQR cover-then-decompose rule.
- The new tests bind parser behavior and existing authority guards only; they
  do not introduce new parser authority, formats, tolerances, or public output
  semantics.
