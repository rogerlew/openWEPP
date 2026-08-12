# Model Definition Embedding Strategy

Status: `FROZEN`

The sole editable production registry is `crates/openwepp-vegetation/model-registry/openwepp_c3_woody_v1_definition.json`, initially byte-copied from the read-only authority artifact. Production uses `include_bytes!`; parsing verifies model version, SHA-256 `003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157`, and the six canonical section digests. Tests require byte equality with the authority-package copy. There is no runtime fallback definition.
