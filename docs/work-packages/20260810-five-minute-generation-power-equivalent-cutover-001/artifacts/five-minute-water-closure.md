# Five Minute Water Closure

Status: `PASS — schema v2 and independent storage-aware reconstruction`

Evidence mode: `Ran`

The reopened contract publishes schema
`openwepp-hillslope-wat-subhourly-v2.0` with 27 columns. It adds
`depression_storage_retention_depth_mm` and renames the ambiguous raw excess
field to `raw_wb14_post_depression_generation_depth_mm`.

The real streaming consumer wrote a 27-column Parquet dataset with 24 rows,
global day bins `0..23`, and hours `0..1`:

- rainfall: `79.56180923624731 mm`;
- raw infiltration: `63.502721051072044 mm`;
- raw post-depression generation: `16.059088185175263 mm`;
- depression-storage retention: `0 mm` for this p61 event;
- storage-aware raw event residual: `7.105427357601002e-15 mm`;
- closed/closing generation: `16.059088185175263 mm`;
- independently reconstructed hour residuals: exact zero and exact zero;
- maximum producer-recorded hourly residual: `3.469446951953614e-15 mm`;
- null exponent/power-rate/power-duration values: `24/24/24`.

The positive-storage Parquet regression independently reads the emitted depth
columns and verifies:

`sum(rainfall) = sum(infiltration) + sum(depression retention) + sum(raw WB14 post-depression generation)`.

Its storage operand is positive, so the test cannot pass by relying on the
zero-storage p61 special case. The writer boundary also rejects nonfinite,
negative, clock-inconsistent, raw-closure-inconsistent, and hourly-closure-
inconsistent public rows before publication.

## User command

Add a fresh, non-existing WAT5 target to the hillslope runfile. The required
output entry is:

```toml
[outputs]
wat_subhourly = "output/H61.wat-subhourly.parquet"
```

With the ordinary required hillslope inputs and outputs present in that same
runfile, one command produces and manifests the WAT5 dataset:

```bash
target/release/openwepp-cli-hill --run-dir /absolute/run-dir --run-file /absolute/run-dir/hillslope.run --output-dir /absolute/run-dir/output --manifest-path /absolute/run-dir/output/manifest.json --direct-production-executor
```

The manifest records that the output was selected by
`run_file.outputs.wat_subhourly_presence`, the requested final path, and the
v2 dataset identity. The writer intentionally refuses an existing WAT5
target, so use a fresh output path rather than overwriting a prior result.
