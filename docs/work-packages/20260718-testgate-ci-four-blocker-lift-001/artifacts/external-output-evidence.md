# External Output Evidence

Ran: the passing combined affected gate confined generated Nextest config and
store, Cargo target, LLVM profiles/objects, JUnit, LCOV, CRAP reports, logs, and
temporary paths beneath `/tmp/openwepp-testgate-four-blocker-final-lJfizstz`.

- External JUnit: 36 test cases, SHA-256
  `5e8af464016d832a0345c46d72c7f50c10bacf78a78ddc784312af05f2fd285d`.
- External LCOV SHA-256:
  `256b88c5bca967223a5991ee4657f660a3061b269c54fa50aa47f5518c9b0304`.
- PASS CRAP report SHA-256:
  `f01e7e54c21c1608d3039995b01a16b37c94eb133fbcd76b4cac2a5c1cb2cfcc`.
- The pre-existing ignored repository JUnit remained unchanged across the
  externalized run: SHA-256
  `91138634d1ffb5b5e9561748e1784621bd46a988f3c2d14679a2d11917caa94e`,
  mtime `1784388710`, size 7601 bytes.
- The end-to-end executor fixture deletes/observes no repository `target/`
  output after preflight and execution. Direct Nextest nodes receive an
  executor-generated external config plus external `--target-dir`; the CRAP
  adapter creates its external Nextest config from the canonical repository
  profile without modifying that profile.
- Executor plan reconstruction now places committed-source graph snapshots,
  inventory snapshots, Cargo targets, and verifier reconstruction work beneath
  the caller-selected artifact root. Directory-backed verification supplies
  the same root independently; in-memory tests use precise external temporary
  workspaces. Snapshot directories are removed on drop while external Cargo
  caches remain inspectable until the artifact root is removed.
