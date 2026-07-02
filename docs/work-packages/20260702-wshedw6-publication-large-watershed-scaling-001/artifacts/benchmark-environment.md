# Benchmark Environment

Status: `passed`

Evidence mode: `Ran:` host and binary inventory commands.

## Host

- Logical CPUs: `48` (`nproc`).
- CPU model: `Intel(R) Xeon(R) CPU E5-2697 v2 @ 2.70GHz`.
- Sockets: `2`.
- Cores per socket: `12`.
- Threads per core: `2`.
- Architecture: `x86_64`.

## Binaries

Release binaries built with:

```sh
cargo build --release -p openwepp-runner --bins
```

Result: `PASS`, release build completed in `1:08.65`.

W6 scaling binaries:

- `target/release/openwepp-cli-watershed`
- `target/release/openwepp-cli-hill`

## Canonical Benchmark Surface

- Mode: strict committed fixture.
- Sidecar policy: `--policy compat`.
- Legacy sidecar discovery: disabled.
- Full large fixture:
  `tests/fixtures/watershed/onshore-xenophobia/runs/case.run`.
- Existing development fixture:
  `tests/fixtures/watershed/carnivorous-adobo/runs/case.run`.
- Output scope: public `openwepp-cli-watershed` full run, including generated
  hillslope HBP jobs, pass inventory, typed network routing, typed publication,
  and watershed parquet writer output.
- Timing command wrapper: `/usr/bin/time -v`.

## Single-Hillslope Onshore Smoke

Command:

```sh
target/release/openwepp-cli-hill \
  --run-dir tests/fixtures/watershed/onshore-xenophobia/runs \
  --run-file p1.source.run \
  --output-dir /tmp/wshedw6_onshore_p1_smoke \
  --policy compat \
  --manifest-path /tmp/wshedw6_onshore_p1_smoke/H1.manifest.json
```

Result: `PASS`.

- Wall time: `0:04.21`.
- User time: `4.18 s`.
- System time: `0.03 s`.
- Max RSS: `43744 KiB`.
