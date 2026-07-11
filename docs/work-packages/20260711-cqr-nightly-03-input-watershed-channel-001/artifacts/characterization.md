# Characterization

Ran: the tests-first phase expanded the focused suite from `21` to `24` tests
before production decomposition. It bound all warning strings, all typed error
IDs/messages/source branches, missing-path IO context, and selected nominal
fields. Focused nextest passed `24/24`.

Review then required stronger ADR bindings. The provisional suite ultimately
passed `32/32` and exercised every required-record truncation, all integer enum
and float guard boundaries, every float token with NaN and both infinities,
compatibility datver boundaries, and broad nominal per-channel/public fields.
These useful test edits are attempt evidence only: local-hold rules required
exact rollback with the production refactor.

The attempt exposed a required case that cannot pass without changing public
error semantics: an extra rating triple when `icntrl != 4` is classified by the
current parser as `CHN-E-002`, while the canonical contract requires
`CHN-E-006`. See `hold-legitimacy-audit.md`.
