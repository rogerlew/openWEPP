# ASSURE-05 Deterministic Staging Evidence

Status: PASS

Evidence class: Ran

Two unrelated disposable roots were prepared with the required model narrative
and built independently from the final source root:

- `/tmp/assure05-us-stage-a.7exyib`
- `/tmp/assure05-us-stage-b.4Yk48v`

For each root, the named `build` and `check` commands passed. `diff -qr` over
the complete roots produced no output. The report source root reported by the
planner was
`08e2b5e3b6444067db7204f790a6670af2d6f16bf1b733879cbc3e95d235dfa6`.
The deterministic staged build manifest was
`072c260e71b835f8f2b5005dd0fe3e489171f82d444191407f9b4ba705af45f2`.

The staged version `1.0.0` contains the manuscript, supplement, two accessible
SVG figures, build manifest, and 15 public-safe research objects. The research
objects include the exact archived execution prompt; study protocol and
realization freeze; science contract; analytical input, result, and independent
procedure; current test, path, and production evidence; H2637 result plus the
exact accepted manifest, HBP, and pass-Parquet inputs; and the agent-assistance
packet.

The staged independent procedure was executed against the staged analytical
input and against the staged H2637 manifest/HBP/Parquet objects. Canonicalized
JSON for both outputs was exactly equal to the corresponding retained result.
The H2637 procedure authenticated the raw object hashes before reconstruction.

All staged research objects are byte-identical to their declared source
objects. The main manuscript and supplement contain no unresolved typed
directives, absolute repository paths, or broken local links. These are
disposable review bytes, not a public release or publication snapshot.
