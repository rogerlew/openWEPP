# PERFARCH03 Cache And Working Set

Evidence class: Ran.

Status: complete 2026-06-18.

## Dense working set

The prototype reports the dense state/flux slot storage used by the measured
array surface:

```text
working_set_bytes 18208
output_slot_count state=543 flux=8 total=551
```

That is `17.78 KiB` of dense slot storage for the measured flow's state and flux
surface.

## RSS measurement

Direct release-binary array-only RSS command:

```bash
/usr/bin/time -f 'elapsed=%e max_rss_kb=%M' \
  docs/work-packages/20260618-perfarch03-full-array-native-floor-prototype-001/artifacts/perfarch03-floor-prototype/target/release/perfarch03-floor-prototype \
  array-only 10000000
```

Result:

```text
array_only_iterations 10000000
array_only_checksum 5.25000000026889174e4
elapsed=11.51 max_rss_kb=3072
```

Measured RSS was `3072 KiB` (`3.0 MiB`), below the PERFIDX06 legacy no-UI RSS
reference of about `4.6 MiB`.

The larger RSS observed while using Cargo to build or run the binary is excluded
from this working-set result because it includes Cargo/rustc process memory and
does not measure the release binary's runtime footprint.

## Interpretation

The measured branch surface is cache-resident. The result supports the
PERFARCH03 hypothesis that the previous logical `BTreeMap`/symbol-authoritative
path was measuring machinery and cache behavior, not an irreducible physics
floor.
