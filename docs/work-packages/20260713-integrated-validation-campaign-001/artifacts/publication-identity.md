# Publication Identity

Status: `PASS`

Evidence class: **Ran** at frozen source
`de520f1ff867ca5c65b1f82dfe32a19c213ae18c`.

The final W7R production CLI test ran the committed p102 watershed with
`--jobs 1` and `--jobs 4`. All 14 Parquet products have identical decoded row
counts, ordered values, null posture, field types/nullability, field metadata,
and schema metadata. HBP and pass files are byte-identical.

Parquet container hashes differ because metadata serialization order is not
byte-canonical; the campaign records that fact and does not substitute a false
byte-identity requirement for semantic publication identity. Per-product
hashes and equality checks are in `logs/final-reconstruction-arithmetic.log`.
