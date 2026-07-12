# Numeric Equivalence

Evidence class: **Static** for source equivalence; **Ran** for tests.

The point-sampling match and multiplication moved verbatim into a private helper;
the call receives the same operands at the same point. Focused
cascade tests passed 7/7 and both full-library coverage executions passed
341/341, including conservation, width-aware handoff, D10B, and Iwagaki oracle
tests. Current whole-file SHA-256 is
`574d98ab6708c9332a6ddef3adc35df843f6cec3a00a05c80c1f5f042ab1d3fb`.
No float production expression or accumulation order changed.
