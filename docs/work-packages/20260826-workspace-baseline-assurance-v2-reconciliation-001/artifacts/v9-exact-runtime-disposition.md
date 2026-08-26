# Protected V9 exact-runtime disposition

Status: `QUALIFIED IN EXACT BOUND RUNTIME`.

Static: the frozen descriptor binds
`/usr/lib/x86_64-linux-gnu/libcrypto.so.3` to SHA-256
`0cd331307536a397ab9c83c6dbeeb3474d0a5114f397ce03d1762adb96d3c781`.
The current host object has different bytes after a system package update. The
descriptor, calculator, vectors, expected output, and integration guard remain
unchanged.

Ran: the exact descriptor object was recovered from Ubuntu package
`libssl3t64_3.5.5-1ubuntu3.3_amd64.deb`. Its extracted `libcrypto.so.3` hashes
exactly to the descriptor value above. No host `/usr` file was replaced.

Ran: a read-only bubblewrap overlay bound only that extracted object at the
descriptor's absolute path. In that overlay:

- the frozen V9 calculator completed successfully;
- generated output SHA-256 was
  `f86770cce11235ba282b47e81de2fa5dc9af19c29dc3bd91c62256957c590633`;
- generated output compared byte-for-byte equal to the frozen V9 vectors; and
- `vegetation_boundary_authority_contract
  v9_oracle_successor_is_exactly_bound_and_v8_is_immutable` passed 1/1.

Disposition: external-runtime mismatch, not source debt and not an oracle
defect. Exact runtime execution is the qualifying path. No runtime descriptor,
expected hash, oracle byte, test, or production behavior is weakened or
rebound. The exact-clean workspace run must use the same one-file read-only
overlay so the protected test executes rather than being waived.
