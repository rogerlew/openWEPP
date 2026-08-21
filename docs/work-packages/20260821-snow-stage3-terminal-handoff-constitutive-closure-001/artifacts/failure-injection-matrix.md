# Failure-injection matrix

Status: `NOT RUN / BLOCKED BY INCOMPLETE COVERED PATH`.

`Static:` The attachment stages a cloned candidate and installs only after
the parent receipt chain length check; typed support, identity, event, and
receiver guards are fail-closed. This is structural rollback posture only.

`Ran:` no current test injects wrong carrier participant/support, wrong owner
digest, wrong topology fraction, event-tick mismatch, restart cursor rewind,
or terminal parcel double consumption through the new attachment. Poison and
rollback qualification remains a required hold-lift gate.
