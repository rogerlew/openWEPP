# Module and dependency design

The production implementation is the bounded workspace crate
`openwepp-persisted-restart-v1`. This avoids an orchestrator dependency cycle:
the crate depends one-way on the orchestrator and scientific-owner crates, and
no existing owner crate depends on it. It composes but does not duplicate owner
authority. The released package remains outside the production dependency
graph.

Wire DTOs, canonical admission, explicit projection, and the default-off host
are public where a persistence caller needs them. Orchestrator accessors are
sealed behind its nondefault `persisted-restart-v1` feature. Admission
constructs isolated owners, performs all fallible checks, and exposes one
atomic replacement boundary. Failure precedence follows released parse,
canonical bytes, outer digest, identities, phase, nested owners, joins,
reconstruction, then installation order. Evidence fixtures are separately
feature-gated. No new file reaches 2,000 lines.
