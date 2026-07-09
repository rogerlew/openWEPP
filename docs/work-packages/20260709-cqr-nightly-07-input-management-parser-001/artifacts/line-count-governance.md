# Line Count Governance

Evidence label: Static/Ran.

Status: `SCAFFOLDED-WARN`

Target file:
`crates/openwepp-input-contract/src/parsers/management.rs`

Baseline line count:

- `2851` lines.

Disposition:

- Target starts above the 2000-line WARN threshold and below the 3000-line
  blocker.
- Closure must keep the target below `3000` lines or include a valid production
  split/refactor before completion.
- Characterization tests should be added to integration tests or a dedicated
  test include, not appended inside the target file.
