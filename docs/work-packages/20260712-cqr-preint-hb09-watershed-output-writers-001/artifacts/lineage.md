# Watershed Writer Lineage

The fourteen schema builders remain publication authority. A private
`write_schema_output` stage preserves the original sequential schema-build and
write order, so first-error and partial-publication behavior do not move.

Float64 lookup is partitioned into five ordered field families. The explicit
tri-state result distinguishes an unknown field from a recognized field whose
value is null; alias precedence, arithmetic grouping, units and Option
propagation remain unchanged. The P102 runner vector consumes the emitted
watershed Parquet path rather than a private batch.
