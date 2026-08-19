# Module and dependency design

The orchestrator owns restart composition because it already owns the direct frame, V10/LSE-V2 shadow, GSI/provider owners, soil thermal, BGC, and scheduler position. Production modules depend only on workspace crates and never on `docs/work-packages`.

Wire DTOs and canonical admission are public only where callers must persist or admit bytes; owner-specific projectors and static-context construction remain sealed. Admission constructs isolated owners, performs all fallible checks, and exposes one atomic replacement boundary. Failure precedence follows the released parser, outer digest, identities, phase, nested owners, joins, reconstruction, then installation order. No new file may reach 3000 lines; 2000 lines triggers a split or recorded WARN rationale.
