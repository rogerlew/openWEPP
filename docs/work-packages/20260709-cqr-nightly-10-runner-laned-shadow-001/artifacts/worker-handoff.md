# Worker Handoff

Evidence label: Static.

Status: `SCAFFOLDED`

Current state:

- Package scaffolded for CQR nightly target #10.
- No production or test implementation edits have been made.
- Required scaffold commit is the next boundary before implementation.

Next step:

1. Commit the scaffold.
2. Read the target source and relevant tests fully.
3. Add characterization for the zero-covered high-CRAP collector helpers.
4. Decompose behavior-preservingly until target CRAP rows are `<= 30`.
