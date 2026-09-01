# WGHL-FULL-001H implementation and validation

## Correction

Static: the producer now reads the terminal boundary from the last sealed
accepted-microstep receipt. The receipt must retain its canonical digest,
parent support, and terminal event posture. The existing one-quantum receiver
remains exact. The following snow-free successor retains cadence only up to the
sealed child end; at or beyond that end, ordinary cadence resumes to the parent
end.

For the exposed partition, the retained authority and tests are:

```text
parent          [0,1800)
accepted child  [0,420), [420,900)
physical event  540
receiver        [540,600)
child successor [600,900)
ordinary tail   [900,1800)
```

The unchanged qualification validator remains fail-closed for a successor
crossing from `600` to `1800` seconds. No adaptive floor, solve support,
tolerance, mass/energy ledger, event, topology, custody, receipt, restart,
publication, or rollback rule changed.

## Focused evidence

Ran:

```text
nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator \
  terminal_successor_partition_tests --no-capture
```

- final focused run: `fe90ab02-fcd3-49e5-9506-cc2389f8d35f`
- result: `PASS`, 2/2; 1143 skipped
- covers sealed child-end derivation, restart serialization equivalence,
  receipt-seal poison, missing-authority poison, and parent-support poison
- log: `/tmp/wghl_001h/focused-terminal-successor-final.log`
- log SHA-256:
  `9c4467bdeb33622c607a5d8717a02eb4d6615fa238f9d563e06aeb54857009a3`

Ran:

```text
nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator \
  -E 'test(qualification_terminal_child_requires_exact_physical_and_successor_partition) | test(qualification_rejects_successor_crossing_the_sealed_terminal_child)' \
  --no-capture
```

- run: `852aa974-d347-41aa-bea7-1673507371cb`
- result: `PASS`, 2/2; 1138 skipped
- covers the complete parent/child/receiver/successor union and explicit
  crossing-successor rejection
- log: `/tmp/wghl_001h/focused-qualification-partition.log`
- log SHA-256:
  `d53f7717b997186225c42c1cfe4f5ea27e0ae48fef76c55aa8f8b715ca1de354`

Ran:

```text
env RUST_MIN_STACK=67108864 nix develop -c cargo nextest run \
  -p openwepp-hillslope-orchestrator \
  -E 'test(interior_terminal_event_runs_covered_event_and_snow_free_remainder) | test(interior_terminal_event_capture_reproduces_below_carrier_domain)' \
  --no-capture
```

- exact-source replacement run: `f11ae697-6486-4482-a585-1111c9a29c8d`
- result: `PASS`, 2/2; 1146 skipped; 198.540 s
- capture consumer: `PASS`, 129.117 s
- ordinary interior-terminal consumer: `PASS`, 69.420 s
- unchanged consumers retain exact event tick `540 s`, exactly-once consumed
  parcel custody, accepted owner/replay receipt validation, contiguous supports,
  parent-end acceptance, and the internal physical closure/qualification gates
- log: `/tmp/wghl_001h/interior-terminal-real-consumers-final.log`
- log SHA-256:
  `ffa51a35b94b72032f7ef13d540a1b78ebf6fa30a12679a43977889d7bca5f94`

This replacement ran after the concurrent v31 owner explicitly returned the
Rust paths formatted, stable, and frozen. It supersedes preliminary run
`fa1cfe7e-9ff3-4a15-bf3c-ecd191c4d26c` as terminal 001H real-consumer
evidence.

## Source quality and exposure

Ran: individual `rustfmt` on all four owned Rust files and `git diff --check`
on the owned Rust/evidence paths; both passed. A package-wide format and
warnings-denied gate remains parent-package scope after concurrent increments
stabilize.

Static: no public item, serialized schema field, feature, or exported API was
added or changed by 001H. Production diagnostics were not added.

Final line counts at this evidence point:

| File | Lines | Disposition |
|---|---:|---|
| `snow_stage3_v11_adaptive_execution.rs` | 2,998 | `WARN`; below 3,000 hard limit |
| `snow_stage3_v11_adaptive_execution_tests.rs` | 597 | pass |
| `snow_stage3_v11_qualification_crossjoin_tests.rs` | 220 | pass |
| `snow_stage3_v11_qualification_crossjoin_child_tests.rs` | 108 | pass |

Decomposition rationale: the production file is an existing included execution
shard whose receipt, restart, event, and rollback flow is tightly ordered;
mixing a broad mechanical split into this chronology correction would enlarge
the semantic diff. Follow-on intent: before further behavior is added here,
extract adaptive receipt-boundary and successor-partition helpers into a
dedicated included implementation shard under the owning package's mechanical
refactor plan.
