# Review Agent A

Status: `CHANGES REQUIRED — SANITY-FAIL SUPPORTED`

Evidence mode: `Static + Ran`

Static: reviewed root, crate, test, science-contract, and work-package
instructions; `package.md`; the complete changed-test diff; the generated HBP
and `chan.inp` builders; HBP and `chan.inp` parsers; hourly routing and
publication calculations; `sanity-results.md`; implementation evidence; and
release logs.

Ran: lightweight read-only checks only. `git diff --check` passed; `wc -l`
reported 1,269 lines for the changed Rust test; release-log recount found 35
`W11C_RESULT` rows, 12 `W11C_TIMESTEP` rows, eight material negative-storage
records, four legacy serial-throughflow records, and four legacy sediment
deltas. I did not rerun the matrix or any cargo gate. At this review snapshot,
the recorded erosion-profile run passed 313/313 and the delegated full-profile
run was still executing.

## Findings

### A-001 — High — `chan.inp` channel IDs do not close against the real topology

The fixture writes and locally validates `ichnum = [1, 2]`
(`mt3_hbp_hourly_consumer_contract.rs:603-625`). With two hillslope blocks,
however, the watershed-structure parser assigns channel element IDs as
`nhill + record_index`, so the two channel element IDs are 3 and 4
(`watershed_structure.rs:458`). The real CLI derives its valid `chan.inp` set
from those structure element IDs (`openwepp-cli-watershed.rs:286-308`).

The test masks the mismatch by supplying a fabricated `BTreeSet::from([1, 2])`
to its local parser. The real compatibility parser retains unknown IDs and
emits `CHN-W-005` (`chaninp.rs:488-512`), rather than proving a canonical,
warning-free sidecar. This does **not** invalidate the 3,600/600-second result:
the real parser remains on `ParsedBranch`, and routing globals consume the
normalized timestep independently of `ichnum`. It does invalidate the test
design's claim that all four records and channel IDs are canonical.

Required response: write `3 4`, validate against the topology-derived set (or
at least `{3, 4}`), assert `ParsedBranch`, assert no parser warnings, and rerun
both debug and exact-release matrices. Do not assume the outputs remain
identical without the rerun.

### A-002 — Medium — the water-closure evidence is algebraic, not anti-tautological

The test computes expected external input from the same in-memory scenario
arrays used to serialize HBP (`mt3_hbp_hourly_consumer_contract.rs:685-686`),
not from reparsed produced HBP bytes. More importantly, the production hourly
path defines each channel's published storage as integrated inflow minus
integrated outflow (`hourly.rs:100-108`), while publication sums that derived
storage over all channels and selects terminal-channel outflow for the EBE
surface (`network_frame.rs:638-670`). In this serial fixture, internal channel
flow cancels, making
`external input = terminal outflow + summed derived storage` an identity when
the ingress projection is intact. `chanwb.Balance` is derived from the same
publication operands.

The package correctly says this exact closure is not a physical pass when
storage is negative, and `sanity-results.md` now calls it "algebraic water
closure." Therefore the negative-storage result is not hidden. Nevertheless,
the test and implementation evidence must not call this an independently
closed conservation gate under the package anti-tautology rule. Reparse and
sum both HBP inputs to establish serialized external authority, and label the
current Parquet identity diagnostic unless an independently authoritative
storage/state operand is added.

### A-003 — Medium — final-gate and release-provenance evidence is incomplete

`gate-results.md` remains `QUEUED` with every gate `NOT RUN`, despite available
release, clippy, and erosion logs. The release artifacts record the binary path,
mtime/size, SHA-256, and a successful matrix log, but neither the build log nor
another current artifact records the exact build command and the exact
`OPENWEPP_W11C_WATERSHED_CLI=... cargo nextest ...` release-run command. A one-
line Cargo completion message is not command provenance. This fails the
package's explicit release-binary evidence rule until reconciled.

At the review snapshot, full nextest, deny, Markdown lint, dual review,
finding disposition, and dual verification had no completed gate-table
evidence. Package closure is therefore not legitimate yet, independently of
the scientific findings.

### A-004 — Low — line-count evidence is stale but the posture passes

The checklist records 1,265 lines; the reviewed file currently has 1,269.
The changed test remains below the 2,000-line WARN and 3,000-line blocker
thresholds, and no production Rust file changed. No line-count refactor is
required. Refresh the count before disposition. The existing 3,103-line
`direct_runtime.rs` test module is outside this package's diff and is not a
new W11C line-count regression.

## Fixture and calculation review

- The topology is a real serial two-channel graph: hillslope 1 feeds channel
  1; hillslope 2 plus upstream channel 1 feed channel 2. H2's all-zero EVENT on
  wave branches is structurally parser-valid and deliberately preserves paired
  hourly authority, but it is a synthetic diagnostic payload, not a canonical
  `NO_EVENT` representation. The CREAMS zero case correctly uses `NO_EVENT`.
- H1 EVENT payloads have 24 finite, nonnegative water and sediment bins. The
  sediment integral is 240 kg and closes to `total_detachment -
  total_deposition = 240 kg`; spike, spread, and uniform scalar durations are
  correctly 3,600, 14,400, and 86,400 seconds. The fixed concentration field
  is not used as an intake mass reconstruction, consistent with
  `SC-INFILE-HBP-001`.
- Input peaks are correctly computed as maximum hour-mean discharge. For this
  fixture they are 2.0, 0.5, and 0.0833333333 m3/s. Peak ratios and timestep
  deltas in `sanity-results.md` agree with the release log.
- The release log contains every declared matrix row: five CREAMS cases and
  five cases for each of six wave branch/timestep combinations. Zero cases are
  exact; nonzero hourly sediment egress is 240 kg to roundoff.
- The test-only release-binary override is scoped to the integration test,
  resolves relative paths under the repository, and uses explicit
  `std::process::Command` arguments. I found no network use, secret handling,
  shell interpolation, or production debug hook.

## SANITY-FAIL finding adjudication

- `W11C-F001`: **supported, High**. Release evidence gives minimum network
  storage `-210.4004750797003 m3` for static MC at 3,600 seconds and
  `-65.192020902048171 m3` for KW at 3,600 seconds, with terminal volume above
  the 7,200 m3 external input. These are material negative physical volumes,
  not tolerance noise, and violate the nonnegative-volume physical bound.
- `W11C-F002`: **supported as a High investigation finding, but not a standalone
  universal-threshold failure**. The maximum observed passive-route peak ratio
  is `1.549880088144078`; variable MC spike peak changes from
  `1.185838944220455` to `3.071519111681464 m3/s` between 3,600 and 600 seconds.
  This is material and timestep-sensitive. The package correctly avoids
  inventing a universal acceptance threshold; defect adjudication still needs
  authoritative numerical/physics follow-up.
- `W11C-F003`: **supported, High**. Static publication tracing confirms that
  the non-interval lane publishes all dispatched channel IDs, sums their
  runoff and sediment, and takes the first ID, whereas the interval lane uses
  outlet IDs (`network_frame.rs:638-707`). Thus the observed 14,400 m3, element
  1, and nonterminal sediment values are not a first-row Parquet-reader error;
  they are current event-scalar multi-channel publication semantics and fail
  the package's terminal-output checks.
- `W11C-F004`: **supported with qualification, High test-evidence defect**. The
  pre-W11C three-line `nchnum=0` sidecar necessarily entered compatibility
  defaulting and used 60 seconds, so the prior 600-second claim was unsupported.
  W11C does prove distinct 3,600/600-second execution, but A-001 must be fixed
  before the replacement sidecar is described as canonical.

The overall `SANITY-FAIL` classification is supported even without treating
peak amplification as a universal fail rule: F001 violates the nonnegative
physical-volume gate, and F003 fails the declared terminal water/sediment
publication gates. A passing test process is correctly not being used to turn
those emitted findings into a physical pass.

## Proposed disposition

Accept the four W11C findings, accept the package-level `SANITY-FAIL`, and hold
closure as `executed-hold`. Before final disposition, correct A-001 and rerun
debug/release evidence; correct the anti-tautology language/evidence for A-002;
record exact release commands; complete and reconcile every final gate; refresh
line count; and disposition both independent reviews. Defect-shaped follow-on
work should separately address negative interval storage/timestep behavior,
event-scalar terminal publication, and the superseded W11B timestep claim; this
characterization package must not imply those production defects were fixed.
