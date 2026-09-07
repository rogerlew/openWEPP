Static: independent authority assessment A, before the corrected full run.
Read canonical testing strategy, local-CI command selection, current full
Nextest configuration, Cargo release profile and owner kickoff. No build or
test run by this reviewer; review B's assessment was not consulted.

## Decision: GO for optimized full-workspace correctness

The canonical requirement is immediate campaign-strength **full-workspace
correctness**, not a specifically mandated default-debug compilation:
`docs/standards/testing-and-gate-strategy.md:174` and `:242`.
Its lines 46–50 permit documented command-plan revision without narrowing or
waiving the underlying requirement. `local-ci-gate-selection.md:27` names the
full command as a typical command, and its reporting rules forbid replacing
full with quick/domain coverage. Adding Cargo's `--release` does not select a
narrower Nextest profile.

Admissible prospectively recorded command:

```text
nix develop --offline -c env CARGO_PROFILE_RELEASE_LTO=false RUST_MIN_STACK=67108864 cargo nextest run --release --workspace --profile full
```

Keep `.config/nextest.toml` unchanged: full's canonical filters, groups,
fail-fast=false, retries and per-test timeouts remain in force. Remove only the
unrelated external 1,200-second process kill; this is not removal of canonical
test timeouts. Preserve exact default features, test inventory and required
separately selected authority/consumer cases. Optimized full is not a claim
that ignored tests, doctests or profile-excluded authority suites ran; their
independent applicable commands remain required.

The release/LTO=false posture matches the frozen experimental build posture
and is a legitimate correctness target for the executed consumer. Retain the
focused default-debug tests on affected LSE/orchestrator/runner surfaces,
including their debug-only guards where applicable. Record compilation and
execution configuration separately: Cargo release and Nextest full are
different axes. Release disables ordinary debug assertions/overflow checking;
do not label this default-debug-full equivalence. Source inspection finds
existing debug assertions, so that distinction is material even though no
governing standard mandates three default-debug full-suite repetitions.

## Failure/evidence boundary

The interrupted initial default-debug run is not a PASS. Preserve its actual
4,202-started/4,021-pass/181-fail/75-skip report and external-kill/stack posture
as historical evidence of that attempted run, subject to reconciliation of
those recorded totals. A missing stack setting does not automatically classify
all 181 failures as infrastructure. Diagnose/disposition the actual failures;
a deterministic semantic/debug-only failure cannot be erased by a later
optimized pass (`testing-and-gate-strategy.md`, section 16).

Freeze source/toolchain/flags/environment and retain exact inventory/results
for each A/F/R cut. Identical-root evidence reuse is permitted only under the
canonical reuse rules. Separate formatting, warnings-denied lint, doctests,
dependency policy, authority/anti-evasion, science/output/closure and terminal
verification obligations remain unchanged. This approval changes execution
posture, not acceptance strength, failure taxonomy, scope or test filters.
