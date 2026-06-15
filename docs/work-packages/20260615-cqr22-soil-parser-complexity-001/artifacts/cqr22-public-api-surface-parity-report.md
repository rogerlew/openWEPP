# CQR22 Public API Surface Parity Report

Status: complete.

Static: production edits are private helper extraction in soil parser paths and
a private sentinel constant. No public item was added, removed, renamed, or
retargeted.

Static: preserved public parser surface includes:

- `ParserMode`
- `SoilErrorCode`
- `SoilParserError`
- `SoilDatver`
- `DisturbedPolicy`
- `SoilLayer`
- `SoilOfes`
- `SoilDocument`
- `SoilParserOptions`
- `parse_soil`

Static: stable error IDs, fields, and messages exercised by CQR22
characterization are unchanged. Parser compatibility and warning behavior were
not modified.
