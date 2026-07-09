# Coverage Before

Evidence label: Static/Ran.

Source artifact: `/tmp/openwepp-cqr-nightly.lcov`

Filter:

```sh
grep -E '^(SF:|LF:|LH:)' /tmp/openwepp-cqr-nightly.lcov | awk 'BEGIN{target="/home/workdir/openWEPP/crates/openwepp-runner/src/bin/openwepp-snowbench.rs"} /^SF:/{infile=($0=="SF:"target)} infile && /^LF:/{lf=$0} infile && /^LH:/{lh=$0} END{print lf; print lh}'
```

Ran: command above, exit `0`.

## Target LCOV

- `LF:217`
- `LH:0`
- Line coverage: `0 / 217 = 0.0%`

Region coverage is not present in LCOV. If characterization tests are added or
materially changed, final closure must record ADR-0021 coverage status using the
available llvm-cov report surface or a justified LCOV-derived surrogate.
