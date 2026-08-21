# Security Impact

Status: `MEDIUM / IMPLEMENTATION HOLD`

The implementation adds no credential, endpoint, deployment, or external
message behavior. It does add a default-off runtime candidate and canonical
restart bytes. Positive controls are typed domain rejection, sealed exposure
identity, complete-owner manifest validation, cloned frame/runtime staging,
row buffering before commit, tamper-digest admission, and no selector/CoE
change.

The typed opt-in owner-aware scheduler now stages the concrete V11/LSE/BGC/
soil-thermal stack and rejects a commit unless its produced owner receipt joins
the handoff. Event participant joins, contiguous event ordinals, and complete
receipt-body/digest history are also enforced at restart admission. The
integrity boundary is still incomplete for the ordinary runner: it does not
construct that typed bundle, and terminal liquid custody plus durable
publication/outbox atomicity remain open. This is a release hold, not a
permission to add a fallback wrapper.
