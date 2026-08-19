# Schema defect disposition

Status: `OPEN / replacement required before authority review`

| Defect | Disposition |
| --- | --- |
| opaque `sealed.payload` arrays | replace with named Rust-independent owner DTO fields |
| all-zero placeholder vector | replace with real boundary, interval-24, cross-midnight, and multi-destination vectors |
| top-level owner duplication | replace with `BetweenDays` / `InProgressDay` phase union |
| duplicated surface-liquid state | retain state only inside direct hydrology; bind outer configuration only |
| flat forcing array | use ordered destination records, each containing exactly 48 intervals |
| `usize` and native-layout assumptions | replace with bounded `u8`/`u32`/`u64`, fixed strings, `HexF64`, `HexU128`, and `Sha256` |
| canonical JSON asserted by schema alone | add strict typed parse, semantic validation, canonical serialization, and exact input-byte equality |
| incomplete frame classification | exhaustively classify every named nested state field and enforce source destructuring without `..` |
| text-only poison evidence | implement typed executable poisons against actual DTOs/vectors and prove live-owner no-op on failure |

No item is deferred to implementation. Authority release is blocked until all
rows have direct schema/vector/test evidence and independent review.
