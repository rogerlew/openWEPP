# Terminal rejected-pair evidence-correlation authority V2 review disposition

Disposition: **HOLD / NO SOURCE IMPLEMENTATION AUTHORITY**

Reviewed authority SHA-256:
`f4a7ff15127fdfd5068f16126f440a57a25026b44a5c610f175dfab30417cc5c`.
Both independent reviewers verified the exact hash, made no authority or
source edits, and did not communicate with each other.

The Rust/custody reviewer returned `GO-to-evidence`. The review found that the
sealed compile-time mode, fixed-size key, private forwarding chain,
crate-private `cfg(test)` path, caller-local arena and post-return failure
boundary can preserve production API and physical behavior. Its findings bind
any successor authority: the exact forwarding files must be named, coupling
selection must be emitted by its actual owner, the mode must remain sealed and
private, capture-only payload work must remain in the unit-test build, and the
physical result must be retained before any fallible diagnostic work.

The numerical/evidence reviewer returned `HOLD`. Therefore the two-GO gate did
not pass and no correlation seam, diagnostic test, receipt capture, matrix or
source implementation is authorized. The blocking findings are:

1. an append-time provider record cannot truthfully finalize its
   `selected-for-trial` bit and digest before the coupling owner selects it;
2. the wire conflates pair position with the live provider roles (`Retry`,
   `Half1`, `Half2`) and omits exact `BracketLower`/`BracketUpper` mappings;
3. the prior pair's `reject_retry` decision and the following pre-provider
   `BelowCarrierDomain` floor decision lack separate canonical custody;
4. signed raw-error direction and deterministic maximum-component tie handling
   are undefined; and
5. required complete-receipt adapter field lists are deferred instead of
   closed by the reviewed authority.

These are accepted authority findings. Fixing them requires a distinct revised
authority and fresh independent reviews; they may not be resolved by source
implementation judgment. A successor should make provider records immutable
and selection a separate coupling-owned binding, separate pair position from
the exact live call-role enum, add an explicit floor-decision record, define
`full-refined` or `refined-full` and first/last tie semantics, and freeze every
adapter schema and field order before review.

The existing physical result remains
`Stage3(TerminalNumerics(BelowCarrierDomain))`. Production Rust remains
unchanged from the last qualified physical implementation
`43cc9bbea2fbf5fe6ab6596cee4162de75cef999`. SnowEnergy v21/LSE v11/
SnowFreeze v139/CoupledTime v6 remain corrected but unverified candidates.
Temporal-operator, Batch V2, event, terminal receiver, restart, runner, Child 3
and cutover implementation remain prohibited.
