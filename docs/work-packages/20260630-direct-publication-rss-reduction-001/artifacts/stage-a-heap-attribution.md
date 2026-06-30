# Stage A Heap Attribution

Evidence class: Ran + Static

## Tooling

`valgrind`, `heaptrack`, and `jeprof` were not available in the worktree
environment. Stage A therefore used:

- `/usr/bin/time -v` peak RSS.
- jemalloc exit statistics through
  `LD_PRELOAD=/lib/x86_64-linux-gnu/libjemalloc.so.2
  MALLOC_CONF='stats_print:true,prof:false'`.
- Static type-size accounting from the existing direct-size probe and source
  inspection.

This is not a full call-site heap profile. It is sufficient to identify the
dominant previously-missed allocation and to predict the measured RSS drop, but
it does not give exact parquet/Arrow call-site attribution.

## Baseline RSS

H2637 direct full-output run before this package:

- Prior package baseline: `1:09.18`, `1159672 KiB` max RSS.
- Stage A jemalloc rerun: `1:14.32`, `1153536 KiB` max RSS.
- jemalloc exit stats include `retained: 1511903232`, showing that most peak
  memory was transient/retained by the allocator rather than live at process
  exit.

The prior size probe accounted for only about `428 MiB`:

- `DirectPublicationDayRow`: `544 B` x `235961` rows = `122.4 MiB`.
- `HillslopeWatRow`: `312 B` x `235961` rows = `70.2 MiB`.
- `HillslopePassRow`: `96 B` x `235961` rows = `21.6 MiB`.
- The retained `DirectPublicationExecution` clone doubled the publication frame
  lower bound, yielding about `214 MiB` extra in the full-output build path.

That left about `700 MiB` unattributed.

## Corrected Dominant Allocation

Static source inspection found the missed allocation in typed direct setup, not
symbol-map setup: each lane constructor preallocated
`DirectLaneConstructorInputs.day_inputs` for every day, and each entry is a
`DirectDayConstructorInputs`.

Measured type size from the direct runtime size probe:

- `DirectDayConstructorInputs = 4040 B`.

H2637 direct production rows:

- `12419` climate days x `19` OFEs = `235961` day/OFE rows.

Estimated allocation:

- `235961 x 4040 B = 953282440 B = 909.1 MiB`.

The production direct executor does not need those stored constructor inputs:
it constructs the per-day inputs dynamically in the execution loop. This makes
the vector a run-length-scaling setup allocation with no output authority.

## Remaining Scaling Buckets

After removing the constructor-input vector, H2637 minimized-output RSS remains
`184644 KiB`, while `cli01` remains `19584 KiB`. The remaining run-length
scaling buckets are:

- `DirectRunPublicationFrame.rows`: whole-run retained
  `DirectPublicationDayRow` values, about `122.4 MiB` lower bound on H2637.
- Full-output-only WAT/PASS projection vectors: about `91.8 MiB` lower bound
  when WAT/PASS are requested.
- Parquet/Arrow writer column buffers and allocator overhead. The full-output
  post-fix RSS is `316212 KiB`, about `131568 KiB` above the HBP/loss-only
  post-fix RSS, which is consistent with requested-output projection and writer
  buffering.

## Stage A Conclusion

The setup-time symbol-map dual representation remains architectural debt, but it
is not the dominant direct endpoint RSS bucket. The largest measured target for
this package was a typed direct constructor-input vector. Removing it is
justified before symbol-map carrier deletion because it is dead weight on the
production direct path and preserves output identity.
