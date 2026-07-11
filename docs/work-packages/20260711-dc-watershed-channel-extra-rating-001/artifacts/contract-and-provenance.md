# Contract and provenance

Status: complete
Evidence mode: Static

Contract `0.1.2` and spec `0.1.1` define `INV-CHN-016`: canonical retained
layout wins; only a valid rating triple whose single-record deletion uniquely
closes the full remaining declared-channel suffix is a prohibited extra rating
record. Both modes return exact `CHN-E-006`; generic, invalid-domain,
ambiguous, and duplicate-enabled-rating residuals remain ordinary errors.

Pinned direct authority is baseline commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`, especially
`wshinp.for:370-433`: arbitrary comments at 376-378, fixed records through
control at 418, and rating read only for `icntrl==4` at 429-433. Exact EOF/
leftover diagnostics and unique suffix recognition are explicitly labeled
openWEPP inference.

Final hashes: contract
`835facb44b2065f5c4505228d83d52200e8472e9826e54db75efe553850cdb0c`;
spec `2faf80f9285099711c8b0f169ff3d69ee16c252822a4abf7735a0424cf7e199f`.
