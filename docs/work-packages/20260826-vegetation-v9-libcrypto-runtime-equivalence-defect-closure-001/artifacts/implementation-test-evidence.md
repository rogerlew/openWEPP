# Implementation and test evidence

Status: implemented and focused-pass.

Static: `verify_v9_runtime_equivalence.py` first rechecks five immutable V9/V8
source artifacts. It inventories every descriptor dynamic object and permits
only zero mismatches or the sole `libcrypto.so.3` mismatch. The equivalence
route retains an open provider descriptor through completion, hashes those
bound bytes, matches their path/device/inode to `/proc/self/maps`, and requires
fixed and streaming SHA-256 known answers. It delegates every remaining
descriptor check to the exact frozen calculator, executes its exact
`build_vectors`, and compares complete canonical stdout to frozen bytes. It
records the observed provider digest on stderr and writes only verified vector
bytes to stdout.

Ran: the current `.4` provider executed the equivalence route with observed
digest `23265e4027cb6439687be04311a0f37e27f29a23bfa4c750c49725da14f986bb`.
Output SHA-256 was the frozen
`f86770cce11235ba282b47e81de2fa5dc9af19c29dc3bd91c62256957c590633`.

Ran: the extracted historical `.3` provider at digest
`0cd331307536a397ab9c83c6dbeeb3474d0a5114f397ce03d1762adb96d3c781`
executed under a read-only one-file bubblewrap overlay. The verifier reported
`route=exact-host`; stdout again had the exact frozen V9 vector digest above.

Ran: `--self-test-poisons` rejected, in order,
`wrong_sha256_provider_result`, `mapped_provider_identity_mismatch`,
`second_runtime_mismatch`, `changed_protected_bytes`, and
`changed_generated_output`. Each poison traversed `verify()` and was required
to fail at its intended error check; the same self-test first admitted a clean
baseline execution.

Ran: final owning-binary Nextest
`da356d53-b633-4df4-bf46-c85144b36dc5` passed 28/28, including both new
tests. Required-suite guard Nextest
`f7157e86-60cc-483d-a29e-2464d61fad07` passed 3/3.
