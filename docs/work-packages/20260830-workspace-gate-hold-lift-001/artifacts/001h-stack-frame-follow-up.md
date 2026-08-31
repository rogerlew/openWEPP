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

Static: the helper enum and three helpers are textually included immediately
before the unchanged parent function from
`snow_stage3_v11_adaptive_execution_stack_helpers.rs`. No public item, schema,
physics expression, tolerance, 60-second floor, acceptance criterion, event,
owner/custody rule, receipt, restart field, rollback boundary, or production
diagnostic changed.

## Debug frame evidence

Ran: an exact-head debug `cargo build -p openwepp-hillslope-orchestrator
--lib` was inspected with `nm -A -S -C`, archive-member extraction, and
`objdump -d -C`. The x86-64 stack-probe prologue for
`execute_covered_real_v11_parent_with_evidence` changed as follows:

| Source | Probe allocation | Tail allocation | Total | KiB |
|---|---:|---:|---:|---:|
| pre-change | `0xd6000` | `0x8e8` | 878,824 B | 858.2 |
| terminal | `0x60000` | `0x5e8` | 394,728 B | 385.5 |

The parent frame decreased by 484,096 bytes (472.8 KiB, 55.1%). The new
private frames are mutually exclusive: covered selection is about 48.8 KiB,
terminal selection about 49.6 KiB, and interruption construction about 34.9
KiB. They do not recreate the prior monolithic 858.2 KiB frame.

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

Ran: individual `rustfmt --edition 2024` on both assigned Rust paths and
`git diff --check`; both passed. Static scans found no new public item or
production diagnostic hook.

## Line-count disposition

| File | Lines | Disposition |
|---|---:|---|
| `snow_stage3_v11_adaptive_execution.rs` | 2,563 | warn, below hard limit |
| `snow_stage3_v11_adaptive_execution_stack_helpers.rs` | 607 | pass |

The private include was prospectively authorized after the initial exact-file
split reached 3,169 lines. It contains only the new mechanical enum and three
helpers; no pre-existing implementation was moved into it.
