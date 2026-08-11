# Five Minute Water Closure

Status: `PASS — post-review real consumer and independent reconstruction`

Evidence mode: `Ran`

The exact rebuilt release runner executed the p61 soil/management/slope
configuration with a two-day source-complete warm-rain climate and explicit
`outputs.wat_subhourly`. Evidence is under
`/home/workdir/openwepp-wat5-terminal/on`.

The real streaming consumer wrote a 26-column Parquet dataset with 24 rows,
global day bins `0..23`, and hours `0..1`:

- rainfall: `79.56180923624731 mm`;
- raw infiltration: `63.502721051072044 mm`;
- raw post-depression generation: `16.059088185175263 mm`;
- raw event residual: `7.105427357601002e-15 mm`;
- closed/closing generation: `16.059088185175263 mm`;
- independently reconstructed hour residuals: exact zero and exact zero;
- maximum producer-recorded hourly residual: `3.469446951953614e-15 mm`;
- null exponent/power-rate/power-duration values: `24/24/24`.

The WAT5 SHA-256 is
`71f943f9ff30f74846f74d521c66ecee8dce64f7ddcd5fe2c64e4d12008ed938`.
Completed-file row count and schema metadata were validated before atomic
no-replace publication, then independently read from Parquet.

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

The writer intentionally refuses an existing WAT5 target, so use a fresh
output path rather than overwriting a prior result. The retained evidence used
this exact argument shape with run directory
`/home/workdir/openwepp-wat5-terminal/on`, runfile `p61.run`, and target
`output/H61.wat-subhourly.parquet`. Its manifest records source commit
`c9f28a7dbe7adf69d8e6d54ebd8da57568af5552`, rebuilt release binary
`/home/workdir/openWEPP/target/release/openwepp-cli-hill`, and binary SHA-256
`f264661135cde810ff4914df80f5aba1e176349af89537794f18187e49bbc85a`.
