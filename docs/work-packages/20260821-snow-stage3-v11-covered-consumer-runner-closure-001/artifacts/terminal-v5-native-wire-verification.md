# Terminal V5 native-wire verification report

Declared native wires: **0**.

No source byte field or file-level occurrence is treated as native replay. In particular, `wb14_child_replay_bytes()` is an accessor, while `validate_wb14_child_replay_binding` is a separate multi-argument binding validator; there is no exact type-associated serializer/preimage+validator pair satisfying the V5 native-wire contract. The bytes are therefore an explicit projection, not a native wire.
