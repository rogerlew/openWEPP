# Line-Count Governance

Status: `PASS with existing-file WARN rows`

Evidence mode: `Ran`

No changed Rust file reaches the nonexempt 3,000-line blocking threshold.
Changed existing files at or above the 2,000-line warning threshold are:

- `direct_runtime/00_core_frames.rs`: 2,707 lines.
- `direct_runtime/laned_active.rs`: 2,028 lines.
- `direct_runtime/runoff.rs`: 2,745 lines.
- `00_builders_and_authority.rs`: 2,760 lines.
- `00c_day_input_builder_impl.rs`: 2,981 lines.

The peak implementation removes the obsolete 541-line WB16 projection body
from its split file and does not create a new monolithic source file. These
WARN rows are retained architecture debt, not a package closure blocker.
