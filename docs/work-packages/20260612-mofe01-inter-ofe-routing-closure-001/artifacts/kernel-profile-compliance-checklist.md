# kernel profile compliance checklist

Status: checked through M-D

Evidence mode: Ran + Static

## M-D checklist

- Production edits: none.
- Typed errors: unchanged.
- `unwrap`/`expect` in production: none introduced.
- Unsafe: none introduced.
- Bounded canonicalization: none introduced.
- Kernel math: unchanged.
- Runtime state/publication paths: unchanged in code; M-D only declares the
  architecture needed before M-E production edits.

Validation:
- Full Rust closure loop was not rerun for M-D because no production Rust,
  science-contract, dependency, or test files were edited.
- Package docs lint is recorded in `gate-results.md`.

## M-C2 checklist

- Production edits: none.
- Typed errors: unchanged.
- `unwrap`/`expect` in production: none introduced.
- Unsafe: none introduced.
- Bounded canonicalization: none introduced.
- Kernel math: unchanged.
- Runtime state/publication paths: unchanged because current architecture has
  no real per-OFE daily WB state surface to retain or publish.

Validation:
- Full Rust closure loop was not rerun for M-C2 because no production Rust,
  science-contract, dependency, or test files were edited.
- Focused existing M-B carry tests passed.
- M-C2 output comparison and publication audit were run separately and are
  recorded in `m-c2-per-ofe-daily-state-scope-evidence.md`.

## M-C checklist

- Production edits: none.
- Typed errors: unchanged.
- `unwrap`/`expect` in production: none introduced.
- Unsafe: none introduced.
- Bounded canonicalization: none introduced.
- Kernel math: unchanged.
- Runtime publication paths: unchanged because current aggregate-only WB13/WAT
  surface cannot support real per-OFE publication semantics.

Validation:
- Full Rust closure loop was not rerun for M-C because no production Rust,
  contract, test, or dependency files were edited.
- M-C output comparison and publication audit were run separately and are
  recorded in `m-c-wat-publication-closure-evidence.md`.

## M-B checklist

- Typed errors: preserved. Missing active frost topology and invalid aggregate-vs-array carry remain typed failures.
- `unwrap`/`expect` in production: none introduced.
- Unsafe: none introduced.
- Bounded canonicalization: top-layer saturation excess is relocated to the explicit current saturation carry under M-B contract authority; no silent default or mask was added.
- Kernel math: limited to contract-pinned inter-OFE routing/carry plumbing and saturation excess conservation.
- Runtime carry paths: updated to purge stale aggregate carry before MOFE
  hourly-array execution and preserve separated `UpStrmQ`/`SubRIn` lineage in
  runtime state. WAT publication remains aggregate-only and is held in M-C.

Validation:
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo test --workspace`: PASS.

## M-A

M-A made no production kernel/runtime edits.
