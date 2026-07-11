# Review Agent B

Status: `EXECUTED-FAIL-WITH-FINDINGS`

Evidence mode: `Static + Ran`

## Verdict

`FAIL` for complete disposition. The 35-case branch/timestep matrix is real,
deterministic, and correctly classified `SANITY-FAIL`: wave closure is
algebraically exact but materially negative first-day storage/output magnitude,
passive-route peak overshoot, and non-terminal CREAMS publication remain.
However, one High test-design defect makes the known-bad CREAMS metadata a
required passing assertion. Two Medium evidence issues also require artifact
correction before the release/authority claims are accepted.

Proposed overall disposition: accept the physical findings and close W11C as a
failed characterization/hold with defect-shaped follow-up, not
`EXECUTED-COMPLETE`. Fix the High harness issue in this package first; it is in
the declared test write set and does not require production changes.

## Findings

### B-H1 — High — known-bad legacy publication is locked in as a pass condition

Static: `mt3_hbp_hourly_consumer_contract.rs:338-345` computes
`expected_element_id = 1` for `ipeak=2`, asserts that value, and only then emits
`legacy_event_publication_uses_first_channel`. The package simultaneously
classifies first-channel metadata as High finding W11C-F003. A future production
fix that publishes terminal channel 2 would therefore fail this test.

This is not merely characterization logging; it converts the defect into a
regression expectation. It conflicts with the package's stated separation of
process success from physical/publication acceptance and with correctness over
completion.

Proposed disposition: `accepted`. Remove the `ipeak=2 -> element 1` required
assertion. Keep the wave-branch terminal-ID assertion. For CREAMS, record the
observed ID and emit the finding when it is not terminal channel 2, or assert
terminal 2 only after the follow-up correction lands. Rerun debug/release
matrices and protected tests.

### B-M1 — Medium — negative-storage authority cites an out-of-scope tolerance

Static: `finding-mechanisms.md` invokes `TOL-ROUTE-003` for the negative
`Storage (m^3)` result. In `SC-ROUTE-001:302`, that tolerance is explicitly for
runon/runoff/loss volumes, not routed storage. The Rust field is computed as a
daily flux difference (`hourly.rs:108`), while pinned `wshchr.for:614-655`
distinguishes absolute final storage `sfnl` from outflow volume `chvol` and
prevents material negative outflow volume.

The finding itself remains well supported: every fixture run is fresh, the
early scenarios have zero forcing before the pulse, baseflow/loss are zero, and
terminal outflow exceeds the only 7,200 m3 external source by 65.19 to 210.40
m3. Thus this is not roundoff or an unbooked initial source. The authority
argument should rest on that independently reconstructed first-day magnitude
and the pinned storage/outflow lineage, not TOL-ROUTE-003.

Proposed disposition: `accepted`, artifact-only correction. Clarify that the
published Rust operand is net routed storage change, explain why its first-day
initial value is zero in this fixture, and cite the pinned `sinit/sfnl/chvol`
sequence. Do not claim TOL-ROUTE-003 directly governs storage.

### B-M2 — Medium — release evidence omits the exact evidence-run command

Static: `release-binary-provenance.md` records the build command, exact binary
path/hash/size/mtime, selector name, run ID, and log, but not the exact command
that set `OPENWEPP_W11C_WATERSHED_CLI` and invoked nextest. The release log
begins at Cargo compilation and does not itself identify the selected child
binary. Root release-evidence governance requires the evidence run command.

The unchanged production hash and test-only absolute-path selector make the
result credible, but the record is not independently reproducible as written.

Proposed disposition: `accepted`, artifact-only correction. Record the exact
environment assignment plus nextest selector/`--no-capture` command. Actual
temporary output paths are needed only if output files/hashes are accepted;
for this package, the structured release log is the accepted result surface.

### B-L1 — Low — line-count evidence is stale by four lines

Ran: `wc -l crates/openwepp-runner/tests/mt3_hbp_hourly_consumer_contract.rs`
reports 1,269 lines, while `line-count-governance-checklist.md` records 1,265.
Both are below the 2,000-line WARN threshold, so this is not a structural
blocker.

Proposed disposition: `accepted`, refresh the count before disposition.

## Branch and timestep coverage

Static PASS. The loops at test lines 164-183 cover five scenarios for each of
KW (`ipeak=3`), static MC (`4`), variable MC (`5`) at 3,600 and 600 seconds,
plus CREAMS (`2`) once: `3 * 2 * 5 + 1 * 5 = 35` cases. The generated wave
sidecar is parsed before execution and asserts 24 versus 144 intervals,
`nchnum=2`, and channel IDs `[1,2]` (`:603-626`), closing the old compatibility
alias exposed by W11C-F004.

Ran: release-log counts independently match 35 `W11C_RESULT`, 33
`W11C_FINDING`, and 12 `W11C_TIMESTEP` rows. `git diff --check` passed.

Scenario independence is adequate: zero, early spike, equal-total early
spread, uniform, and late spike have distinct arrays; only the intended
7,200 m3/240 kg totals alias. Early/late pulses share scalar descriptors by
design for the event-scalar control.

## Physical classification and legacy interpretation

Static + Ran PASS for classification truthfulness:

- W11C-F001 is material, not tautological acceptance. The balance identity
  closes because negative storage absorbs excess output; the independent
  magnitude audit shows output greater than the sole external input.
- W11C-F002 is correctly labeled defect-shaped rather than contract-adjudicated.
  With hourly-uniform boundary forcing, zero baseflow/loss, and no initial
  pulse storage, peak ratios up to 1.549880 and large 600/3,600-second deltas
  warrant High follow-up. The package does not invent a universal numeric
  threshold or claim observed-data validation.
- W11C-F003 correctly interprets the current CREAMS surface. Static source
  inspection shows the non-interval publication aggregates all dispatched
  serial channels but pairs the result with the first channel ID. The 14,400
  m3 water and rate-like sediment values therefore cannot be represented as a
  terminal 7,200 m3/240 kg outlet result. This also conflicts with
  `SC-ROUTE-001#INV-ROUTE-005(d)` hourly sediment mass authority.
- W11C-F004 is a valid High evidence defect: the corrected four-record sidecar
  and parser assertions prove that the earlier three-record `nchnum=0` fixture
  did not execute its written timestep.

Exact zero behavior, finite outputs, wave sediment mass, shape distinction,
and later-versus-earlier storage relationships all pass. Those successes do
not offset the required physical-sanity failures.

## Gate legitimacy and truthfulness

Current `gate-results.md` is truthful: focused/release/protected tests and
formatting/focused clippy pass; physical sanity is explicitly `FAIL`; workspace
clippy, erosion, full, deny, Markdown lint, review, and verification remain
`NOT RUN`/pending. Under the package non-deferral rule, the physical `FAIL`
alone prevents complete disposition even if every regression gate later
passes. Heavy regression gates remain useful to validate that the
characterization harness did not damage the protected tree.

The test process's `PASS` is not presented as a physical pass in
`implementation-test-evidence.md` or `sanity-results.md`. That separation is
good. The package may not use successful Cargo exit status to downgrade
`SANITY-FAIL` to `SANITY-PASS-WITH-FINDING`.

## Security and scope

Static PASS. Only the declared runner test and package/catalog/roadmap docs are
changed; no production Rust is modified. Fixture writes are confined to unique
temporary directories. Child execution uses `Command::new` with explicit
arguments; there is no shell interpolation, network access, secret handling,
or production debug hook. `OPENWEPP_W11C_WATERSHED_CLI` is test-only and
resolves relative paths against the repository root.

Line-count governance has no threshold blocker after B-L1 is refreshed.
