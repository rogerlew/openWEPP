# Review Agent A — Primary Rust Correctness

Evidence class: `Static + Ran` against committed producer head `19bd7aa8`.

Disposition: `HOLD` before remediation.

The primary reviewer found six material defects: malformed full schema-v5 JSON;
evaluation primitives executing from authoritative and filtered-out calls; no
typed pre-clone tag; incomplete shared-input fingerprints; double-scaled
partial-hour turbulent energy; and selected empty/unresolved requests silently
falling back to schema v4. It also identified contradictory sequential surface
arm fields, missing full-row/filtered/fingerprint/terminal tests, and incomplete
public-output evidence.

Ran: the original focused runtime and contract suites passed `27/27`, proving
the defects were gaps in the producer-adjacent tests rather than observed by
those tests.

Re-review of the remediated commit is required before closure.
