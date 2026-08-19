# Authority stage matrix

| Stage | Status | Release condition |
|---|---|---|
| Exact intake | PASS | clean required local checkpoint and instruction discovery recorded |
| Single executable authority source | IN PROGRESS | package-local Rust reference now owns strict parse/serialization and the first exhaustive real-runtime mapping; remaining DTOs, artifacts, and poisons must migrate |
| Primitive wire types | PASS | strict `HexF64`, `HexU128`, `Sha256Hex`, `WireDayIndex`, `WireIntervalIndex`, `WireLaneIndex`, and `WireCount` tests pass 6/6 with canonical parser and runtime mapping tests |
| Exhaustive hydrology mapping | NOT RUN | every classified field is compile-time mapped and runtime round-tripped |
| Full poison matrix | NOT RUN | every typed category rejects without live-byte mutation |
| Independent authority reviews | BLOCKED | exact-current hydrology, serialization, and GSI/forcing PASS |
| Dual terminal verification | BLOCKED | both exact-current terminal verifiers PASS |
| Production restart implementation | FORBIDDEN | authority package released first |
