# Implementation

The four public mapping methods now delegate to private, cohesive mapping helpers:
`scalar_value`, `unit_label_for_variant`, `phase_label`, and `adapter_label`.
Each extracted helper retains the original match arms in their original order;
the public method is now a one-call seam. A `#[cfg(test)]` characterization module
pins the results and typed guard behavior.

The characterization also exercises the public, constructible state in which an
empty dense view falls back to legacy dense slots. This directly covers both
`or_else` fallback closures in the request lookup helpers.

The extracted helpers have cyclomatic values `18`, `18`, `14`, and `7`, all below
the `30` bound. No formula, threshold, public type, serialized representation, or
typed fail-closed behavior changed.
