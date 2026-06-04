# Review Agent B

Status: completed/HOLD
Evidence mode: Static + Ran

## Findings

1. High - raw conversion guard can be bypassed by equivalent Rust literal spellings.

   Static: `tools/release/check_raw_unit_conversions.py:29-35` matches only a
   small set of exact decimal spellings (`1000.0`, `1_000.0`, `0.001`,
   `3600.0`, `3_600.0`, `3.6e6`, `39.37`, `0.0254`, `1609.0`,
   `1_609.0`, `100.0`, `0.04184`). Equivalent valid Rust spellings such as
   `1e3`, `1.0e3`, `1000.`, `1000_f64`, `3600f64`, `4.184e-2`, or
   `0.041_84` would not be flagged. The guard contract says it must fail on
   unauthorized raw conversion literals in enforced production files
   (`docs/specifications/unit-governance.md:151-162`), and the current test
   only proves rejection for one canonical spelling
   (`tests/integration/hphys0276_raw_unit_conversion_guard_contract.rs:27-51`).

   Required disposition: accepted before package closure, or explicitly
   deferred with a follow-up package if HPHYS0276 is intentionally limited to
   literal-shape inventory. Preferred fix is to parse/tokenize Rust numeric
   literals or broaden the matcher and add rejection fixtures for alternate
   spellings in each first-wave literal class.

2. Medium - allow markers are window-scoped and not bound to a specific literal or class.

   Static: `line_is_allowed` accepts any `UNIT-CONVERSION-ALLOW:` marker in
   the previous three lines or current line
   (`tools/release/check_raw_unit_conversions.py:96-99`), and `scan_file`
   skips all matching literals on that line when the window matches
   (`tools/release/check_raw_unit_conversions.py:134-138`). This can
   unintentionally authorize an unrelated conversion literal near a legitimate
   exception, and the marker is not required to be a comment or to name the
   literal class. The current positive test only verifies that one documented
   exception is accepted
   (`tests/integration/hphys0276_raw_unit_conversion_guard_contract.rs:79-100`);
   it does not prove that adjacent unauthorized literals are rejected.

   Required disposition: accepted before expanding enforcement, or follow-up if
   first-wave closure only requires current local exceptions. Add negative
   fixtures showing that one allow marker does not suppress unrelated literal
   classes or additional literals, and bind allows to a single line/literal
   class where practical.

3. Medium - contract-derived tests do not cover the full first-wave helper surface or failure behavior.

   Static: the contract declares canonical helpers for length, time/rate,
   radiation, snow density/depth, and Celsius-delta conversions
   (`docs/specifications/science-contracts/unit-safe-boundary-types-contract.md:89-114`).
   The added crate tests cover several happy-path directions
   (`crates/openwepp-unit-boundary/src/lib.rs:1117-1168`), but they do not
   directly exercise all new helper names (`meters_to_millimeters`,
   `millimeters_to_meters`, `meters_to_centimeters`,
   `centimeters_to_meters`, `hours_to_seconds`,
   `meters_per_second_to_centimeters_per_hour`,
   `celsius_delta_to_fahrenheit_delta`) or representative non-finite,
   negative, divisor, and overflow rejection paths. This is a test robustness
   gap for the stated invariant that every helper validates finite
   inputs/results and domain failures.

   Required disposition: accepted or follow-up with explicit rationale. Before
   promoting beyond completed/HOLD, add parameterized helper tests or a compact
   table-driven test suite that verifies both direction and typed error
   behavior for every first-wave helper class.

## Non-Blocking Debt / Follow-Ups

- Low: `docs/architecture/unit-safe-boundary-types.md:60-71` lists only a
  subset of the helpers implemented in
  `crates/openwepp-unit-boundary/src/lib.rs:90-229` and declared in the
  contract. Add the omitted helper names so the architecture page and contract
  remain aligned.
- Low: `tools/release/check_raw_unit_conversions.py:102-127` uses a simple
  brace counter for `#[cfg(test)] mod tests` skipping. This is acceptable for
  the current enforced files, but it is brittle against strings/comments with
  braces and external test module declarations. Revisit if the guard expands
  to broader production roots.
- Low: I saw concurrent artifact updates during review. The latest
  `gate-results.md` records targeted gates as pass and workspace tests as
  HOLD for known SIMIMPL18 failures; no stale placeholder finding remains in
  the current artifact state.

## Review Evidence

Static: reviewed current git status/diff and the requested files:

- `crates/openwepp-unit-boundary/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `tools/release/check_raw_unit_conversions.py`
- `tools/release/check_raw_unit_conversions.sh`
- `tests/integration/hphys0276_raw_unit_conversion_guard_contract.rs`
- touched unit-governance and unit-safe-boundary docs
- HPHYS0276 package/evidence artifacts

Ran during initial review, before accepted follow-up fixes:

- `cargo fmt --check`: pass.
- `tools/release/check_raw_unit_conversions.sh`: pass.
- `cargo test -p openwepp-unit-boundary`: pass, 22 tests.
- `cargo test --test hphys0276_raw_unit_conversion_guard_contract`: pass, 3 tests.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo deny check`: pass with existing duplicate crate and unmatched-license warnings.
- `tools/release/check_raw_unit_conversions.py --inventory-all-production`: pass/exit 0 inventory mode, with remaining all-production findings printed for HOLD follow-up.

## Initial QA Pass Statement

Superseded by the disposition update below. Initial QA status was HOLD pending
disposition of the guard robustness and helper-test coverage findings. The
focused implementation gates passed, and the first-wave production replacements
were maintainable in shape, but the guard was not yet strong enough to support
the package's anti-evasion claims without accepted fixes or an explicit
deferred/follow-up disposition.

## Disposition Update

Static:
- B1 accepted/fixed: guard matching now classifies normalized Rust numeric
  literals instead of exact decimal strings, and fixtures reject alternate
  spellings such as `0.041_84`, `1e3`, `3600f64`, and `1_609_f64`.
- B2 accepted/fixed: allow markers are class-bound and limited to the current
  line or immediately preceding comment line; fixtures prove a marker for one
  class does not suppress another class.
- B3 accepted/fixed for first-wave closure: helper tests now cover the
  previously untested length/time/rate directions and representative invalid
  input domains. A fully exhaustive every-helper error matrix is non-blocking
  follow-up.
- Low architecture-list finding accepted/fixed: architecture docs now list the
  full canonical first-wave helper surface.
- Low test-skip brittleness accepted/follow-up: current skip logic remains
  sufficient for first-wave enforced files; revisit when guard default coverage
  expands.

Ran:
- `tools/release/check_raw_unit_conversions.sh`: pass.
- `cargo test -p openwepp-unit-boundary`: pass, 24 tests.
- `cargo test --test hphys0276_raw_unit_conversion_guard_contract`: pass, 5 tests.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.

Disposition: accepted/fixed or follow-up as stated; no undispositioned B
findings remain.
