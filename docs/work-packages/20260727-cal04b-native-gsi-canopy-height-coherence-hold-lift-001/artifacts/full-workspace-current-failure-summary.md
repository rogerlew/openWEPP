# Exact-Head Full-Workspace Failure Summary

Status: `FAIL / SINGLE PRE-EXISTING ASSURANCE ROOT`

Evidence class: `Ran + Static`

Command:

```text
cargo nextest run --workspace --profile full
```

Run identity: `497732f4-6d9c-41a0-b55d-ca5f871e98d0`.

Exact production HEAD: `bdeaa2b2`. The working-tree changes during the run were
documentation disposition and the generated native replay CSV only; no Rust,
test, assurance, or fixture source changed after that commit.

Summary:

- 2,292 tests run;
- 2,229 passed, including six slow tests;
- 63 failed;
- 43 skipped;
- 358.256 seconds;
- exit code 100.

The generated JUnit at `target/nextest/full/junit.xml` contained exactly 63
failure elements and had SHA-256
`f0193cea9ba6097ed25a27cc40445135579b63aeaccdf12551de9d4addb6142b`
at disposition time. All 63 failures are in `openwepp-assurance` or integration
test binaries whose names begin `assurance_`. No canopy, runner, orchestrator,
erosion, frame-size, or native replay test failed.

The failure payloads reduce to one source:

- generated identity drift for
  `tests/fixtures/cancov_forest/README.md`;
- SHA-256 mismatch for that same identified source; or
- the dependent `snow-and-frozen-soil-process-evaluation` report being not
  current and therefore not assemblable.

Current README SHA-256:
`b81fbe2efa5624e5018c18f24c55ada53d7c484ff020b19d6fa1deae8bd1dd7b`.

Bound identity:
`703a138076900f24a3232457dfab8744e60f69ab196b4b361eeb12bbfedb268c`.

The mismatch exists at authenticated package base `f4b3db6c`; the bound value
is exactly the README identity at `502dd745^`, and commit `502dd745` introduced
the current README content. This package changes neither side of the mismatch.
