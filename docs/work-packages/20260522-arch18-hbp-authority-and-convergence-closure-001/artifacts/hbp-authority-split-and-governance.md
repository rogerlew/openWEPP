# HBP Authority Split and Governance

Static: authority inventory and boundary mapping complete.
Ran: convergence tests executed across parser and bridge surfaces.
Status: complete.

## Authority Model

### Parser authority (`openwepp-input-contract`)

Canonical HBP file-family authority for `H<hillslope_id>.hbp` is owned by the
parser contract/implementation surface:

- `docs/specifications/science-contracts/contracts/SC-INFILE-HBP-001.md`
- `crates/openwepp-input-contract/src/parsers/hbp.rs`

Owned concerns:

- naming policy (`H*.hbp`, compat-only `.pass.dat` derivation, forbidden suffix rejection)
- schema major/minor acceptance (`1.x`, `2.x`)
- binary layout validation and closure checks (header/year-table/registry/directory/payload/footer)
- typed parser error/warning IDs (`HBP-E-*`, `HBP-W-001`) for parser-surface semantics

Evidence anchors:

- `crates/openwepp-input-contract/src/parsers/hbp.rs:1846`
- `crates/openwepp-input-contract/src/parsers/hbp.rs:1896`

### Bridge authority (`openwepp-legacy-bridge`)

Legacy bridge HBP authority is adapter-local and intentionally narrower than
full parser authority:

- `docs/specifications/science-contracts/legacy-sidecar-bridge-contract.md`
- `docs/architecture/legacy-sidecar-bridge-boundary.md`
- `crates/openwepp-legacy-bridge/src/hbp.rs`

Owned concerns:

- strict/compat gating for configured 4-byte legacy/canonical magic aliases
- deterministic typed compatibility warnings/errors for edge adaptation
- no parsing of full HBP layout or payload semantics

Evidence anchors:

- `crates/openwepp-legacy-bridge/src/hbp.rs:30`
- `crates/openwepp-legacy-bridge/src/hbp.rs:182`

## Split Responsibility Matrix

| Concern | Parser owner | Bridge owner | Rule |
|---|---|---|---|
| `H*.hbp` naming + `.pass.dat` compat derivation | yes | no | parser is source-of-truth |
| header/year-table/payload/footer closure | yes | no | parser-only |
| legacy 4-byte magic alias gate | no | yes | bridge-only |
| strict/compat policy semantics | yes (path family) | yes (magic alias) | each on own boundary |
| `HBP-W-001` compat warning identity | yes | yes | shared ID required |

## Convergence Constraints

1. `HBP-W-001` is a shared, stable compatibility warning ID across parser and bridge.
2. Strict mode rejects legacy forms in both surfaces.
3. Compatibility mode accepts constrained legacy forms with explicit warning emission.
4. No silent fallback to legacy text pass files is allowed.
5. Parser and bridge are not merged authorities; divergence is controlled by explicit tests and contract docs, not by implicit behavioral coupling.

## Drift-Prevention Gate

Convergence tests are required whenever parser or bridge HBP compatibility code
changes:

- `tests/integration/infile_hbp_parser_contract.rs`
  - `parser_and_bridge_share_hbp_w_001_warning_id`
  - `strict_policy_rejects_legacy_forms_across_parser_and_bridge`
  - `compatibility_policy_accepts_legacy_forms_with_hbp_w_001`

Evidence anchors:

- `tests/integration/infile_hbp_parser_contract.rs:643`
- `tests/integration/infile_hbp_parser_contract.rs:652`
- `tests/integration/infile_hbp_parser_contract.rs:681`
