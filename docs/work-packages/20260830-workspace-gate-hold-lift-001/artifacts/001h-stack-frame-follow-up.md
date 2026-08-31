# WGHL-FULL-001H adaptive-parent stack-frame follow-up

## Mechanical change

Static: `execute_covered_real_v11_parent_with_evidence` retains its signature,
branch order, error order, interruption sites, receipt mutation order, adaptive
support chronology, and final publication path. The mutually exclusive covered
and terminal candidate-selection blocks now execute in private `#[inline(never)]`
helpers and return boxed accepted trials. The repeated interruption macro still
occupies its original call sites, but delegates the exact checkpoint clone,
restart `take`, and paused-outcome construction to one private helper returning
a boxed outcome.

Static: the helper enum and private helpers are textually included immediately
before the parent function from
`snow_stage3_v11_adaptive_execution_stack_helpers.rs`. No public item, schema,
physics expression, tolerance, 60-second floor, acceptance criterion, event,
owner/custody rule, receipt, restart field, rollback boundary, or production
diagnostic changed.

Static: the second follow-up stages initialization, active-loop execution, and
final publication through a private boxed execution-state carrier. The active
loop remains source-ordered and the snow-free successor call alone is isolated
behind a private `#[inline(never)]` boxed result. This changes only where debug
stack storage is live: the same initialization checks, interruption sites,
candidate selection, terminal receiver, support chronology, predecessor joins,
failure injections, receipt appends, and finalization execute in the same order.

## Debug frame evidence

Ran: an exact-head debug `cargo build -p openwepp-hillslope-orchestrator
--lib` was inspected with `nm -A -S -C`, archive-member extraction, and
`objdump -d -C`. The x86-64 stack-probe prologue for
`execute_covered_real_v11_parent_with_evidence` changed as follows:

| Source | Probe allocation | Tail allocation | Total | KiB |
|---|---:|---:|---:|---:|
| pre-change | `0xd6000` | `0x8e8` | 878,824 B | 858.2 |
| first follow-up | `0x60000` | `0x5e8` | 394,728 B | 385.5 |
| second follow-up | `4 * 0x1000` | `0xc40` | 19,520 B | 19.1 |

The parent frame decreased by 484,096 bytes (472.8 KiB, 55.1%). The new
private frames are mutually exclusive: covered selection is about 48.8 KiB,
terminal selection about 49.6 KiB, and interruption construction about 34.9
KiB. They do not recreate the prior monolithic 858.2 KiB frame.

Ran: the second exact-head archive/object inspection measured the parent at
19,520 bytes, a further 375,208-byte (366.4-KiB, 95.1%) reduction from the first
follow-up and a 97.8% reduction from the original frame. The active-loop closure
is 121,400 bytes (`0x1d000 + 0xa38`, 118.6 KiB), below the requested 128-KiB
target. Its snow-free successor call frame is 54,904 bytes (`0xd000 + 0x678`,
53.6 KiB) and is live only on that mutually exclusive branch. Exact symbol and
prologue logs are `/tmp/wghl-001h-stack2-symbols.log`,
`/tmp/wghl-001h-stack2-parent-frame.log`,
`/tmp/wghl-001h-stack2-loop-frame.log`, and
`/tmp/wghl-001h-stack2-successor-frame.log`, with SHA-256 values
`41b1f6e6b08ae98fd776087a66e00b558339031d4f5668c59c75619c1fb290ae`,
`04791e83ba177b37fd6c9bfe065073791e4881198a48c00e2551fd69cdce7f27`,
`6cd495efb61e1d14a1ea99358a2645fe4bf13a9d27bc2703f19a55f0f6931bd6`,
and `1e457131fe25aa218df691c72260c708f8ff852d3bde99e6171c6e88e0d94800`.

## Validation

Ran:

```text
nix develop -c cargo check -p openwepp-hillslope-orchestrator --lib
```

- result: `PASS`
- log: `/tmp/wghl-001h-stack-cargo-check-final.log`
- SHA-256: `ebd84b4d90dace1e78f761b08566bd329bac8da93f6ff0bf43e2bd1ca4c1c6d1`
- warnings are confined to concurrent V33/fixed-point work

Ran against the terminal include source:

```text
nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator \
  -E 'test(/terminal_successor_partition_tests/) | \
      test(qualification_terminal_child_requires_exact_physical_and_successor_partition) | \
      test(qualification_rejects_successor_crossing_the_sealed_terminal_child)' \
  --no-capture --no-fail-fast
```

- run: `17e7f073-ca8a-4c79-9466-5dcd9c396ca4`
- result: `PASS`, 4/4; 1,181 skipped
- covers sealed child-end derivation, restart equivalence, receipt/authority
  poison, exact terminal-child partition, and crossing-successor refusal
- log: `/tmp/wghl-001h-stack-focused-partition-final.log`
- SHA-256: `0d6a18cbdbccf84a688e1eff159935f2c89c58f6b395949d4537d9e7bdba29da`

Ran against the terminal include source:

```text
env RUST_MIN_STACK=67108864 nix develop -c cargo nextest run \
  -p openwepp-hillslope-orchestrator \
  -E 'test(interior_terminal_event_runs_covered_event_and_snow_free_remainder) | \
      test(interior_terminal_event_capture_reproduces_below_carrier_domain)' \
  --no-capture --no-fail-fast
```

- run: `489d3cda-bb4d-4f63-8319-a5dbe4025fd2`
- result: `PASS`, 2/2; 1,183 skipped; 190.242 s
- capture consumer: `PASS`, 123.058 s
- ordinary consumer: `PASS`, 67.181 s
- log: `/tmp/wghl-001h-stack-real-consumers-final.log`
- SHA-256: `883a73c7e446f694b6e1186faae9cba38a43ddd9dc5bac62acfb1f84daf05ab7`

Ran against the second-follow-up exact source:

```text
nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator \
  -E 'test(/terminal_successor_partition_tests/) | \
      test(qualification_terminal_child_requires_exact_physical_and_successor_partition) | \
      test(qualification_rejects_successor_crossing_the_sealed_terminal_child)' \
  --no-capture --no-fail-fast
```

- run: `2fee6699-78dc-451e-b3cb-8f2577e54a66`
- result: `PASS`, 4/4; 1,182 skipped
- log: `/tmp/wghl-001h-stack2-focused.log`
- SHA-256: `43aed22e5adb64720e76e55d0c41c4b27bdaa87679fd4a1a81a658387f69356a`

Ran the two unchanged real interior-terminal consumers against that same
source:

```text
env RUST_MIN_STACK=67108864 nix develop -c cargo nextest run \
  -p openwepp-hillslope-orchestrator \
  -E 'test(interior_terminal_event_runs_covered_event_and_snow_free_remainder) | \
      test(interior_terminal_event_capture_reproduces_below_carrier_domain)' \
  --no-capture --no-fail-fast
```

- run: `f6d526d0-4476-45c7-a2e2-c1dfd772be29`
- result: `PASS`, 2/2; 1,184 skipped; 190.643 s
- capture consumer: `PASS`, 122.921 s
- ordinary consumer: `PASS`, 67.718 s
- log: `/tmp/wghl-001h-stack2-real-consumers.log`
- SHA-256: `e05fef06291be65d7a99e816b12442da2901e9ab3630a7a6e64a5f13cb7c0dd6`

Ran the final exact-source library check: `PASS`; log
`/tmp/wghl-001h-stack2-cargo-check.log`, SHA-256
`eaa25e6e6152997395c2c69bb0e711850612c9ac84ea3503abab07866a337baf`.

Ran: individual `rustfmt --edition 2024` on both assigned Rust paths and
`git diff --check`; both passed. Static scans found no new public item or
production diagnostic hook.

## Line-count disposition

| File | Lines | Disposition |
|---|---:|---|
| `snow_stage3_v11_adaptive_execution.rs` | 2,327 | warn, below hard limit |
| `snow_stage3_v11_adaptive_execution_stack_helpers.rs` | 1,092 | pass |

The private include was prospectively authorized after the initial exact-file
split reached 3,169 lines. It contains only private mechanical state, enums,
and helpers moved from the same parent function; no public API was added.
