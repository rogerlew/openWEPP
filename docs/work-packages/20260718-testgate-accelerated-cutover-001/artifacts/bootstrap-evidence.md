# Bootstrap And Cleanup Evidence

Ran: exact forest1 candidate on 2026-07-19 PDT.

- The runner started with empty `/runner-work`, `/cache/cargo`, and `/t`
  surfaces. The pinned image supplied Rust 1.92.0, Nextest 0.9.138, cargo-deny
  0.19.6, cargo-llvm-cov 0.8.7, cargo-crap 0.2.2, Python/pandas/PyArrow, PHP,
  markdown-doc, and uk2us without job-time installation.
- `bootstrap_dependencies.sh` fetched the locked planning-base and candidate
  graphs before the gate invoked offline Cargo/Nextest inventory. The fresh
  cache included the previously missing `alloc-no-stdlib 2.0.4`.
- The same job then completed full-profile coverage execution with 2,165 tests
  passed and every nested offline inventory reconstruction accepted.
- Peak observations were 21.4/48 GiB runner memory, 116/8,192 PIDs, and 4.6/40
  GiB primary coverage-target use. Host headroom remained healthy; the
  container never approached a resource limit.
- Source manifests remained identical before, during, and after measurement.
  Output stayed beneath the external executable target/evidence tmpfs.
- After extracting the hashed receipt, only the GitHub listener remained.
  Source, Cargo, target, evidence, home, temporary, and diagnostic content was
  deleted, all six writable surfaces were verified empty, and GitHub reported
  the provider online and idle.

This proves a cold writable-surface lifecycle. Persistent executable job state
is deliberately not retained between increments; dependency warmth is not an
acceptance assumption.
