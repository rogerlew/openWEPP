# Contract Gate Evidence

Status: `EXECUTED`

Evidence class: `Static` and `Ran`.

Touched surfaces:

- Public watershed CLI routed-stage handoff.
- Watershed dispatch.
- Typed network frame routed-state storage and publication.
- WS10/WS11/WS12/WS18/WS20 watershed routing helper access path.
- WS12 impoundment coefficient projection visibility.

Applicable authority:

- W4DC01 is behavior-preserving with respect to routing physics because the
  direct typed path calls the existing authoritative helper implementations
  rather than new process math.
- No canonical `SC-*` amendment was required.
- No output schema was changed.
- Runtime guard posture remains fail-closed.

Contract-derived tests and gates:

- W4 public CLI source guard updated before closure.
- WS10 production kernel contract passed.
- WS11/WS20 channel routing physics equivalence contract passed.
- WS12 impoundment physics equivalence contract passed.
- Full workspace nextest profile passed.

Pre-implementation hold:

- The prior shortcut attempt was rejected because it bypassed existing routing
  physics.
- This correction resolves that finding by direct re-implementation over typed
  inputs with the existing physics helper bodies.
