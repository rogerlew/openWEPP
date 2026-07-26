# Nested Link Resource Correction

Evidence class: Ran + Static.

Two exact-head QA attempts reproduced `lld` signal 7 in the same three
repository-snapshot verifier tests.

Read-only forest diagnosis established:

- all 12 repository-snapshot tests match a group with `max-threads = 1`;
- each reserves all 32 configured Nextest slots, making the tests globally
  mutually exclusive;
- the three failures ran sequentially;
- the container has a 64 GiB memory cap and recorded no OOM or OOM-kill event;
- `/tmp` is a 24 GiB executable tmpfs;
- each failing test constructs a fresh repository snapshot and nested Cargo
  target under `/tmp/openwepp-gate-*`;
- the cgroup lifetime peak reached 61.48 GiB, about 96.1 percent of its cap;
- `/testgate-history` is disk-backed ext4 with 298 GiB free and no inode
  pressure after the attempt.

The high-confidence cause is tmpfs pressure during each individual nested
workspace link. An mmap-backed linker output can fault with `SIGBUS` when its
backing tmpfs cannot satisfy the page.

The correction retains the existing Nextest schedule and exact inventories.
Admission creates `local/tmp` under the disk-backed quality attempt root, sets
`TMPDIR` for inventory/build and both instrumented profiles, and binds the
absolute root plus policy identifier into the instrumented build identity.
Heavy validation fails closed if the directory, path, or policy changes.
Ordinary failure cleanup removes it with the attempt-local tree.

The first disk-backed qualification proved all three former linker failures
pass, but exposed a Unix-socket path-budget failure and PID exhaustion in the
nested CQR self-test. The follow-up retains disk backing while shortening the
attempt root to `/testgate-history/q/<run>-<attempt>`. It assigns only the CQR
self-test to the existing globally exclusive repository-snapshot cohort,
preventing its nested Cargo inventory from competing for the 8,192-process
container limit.
