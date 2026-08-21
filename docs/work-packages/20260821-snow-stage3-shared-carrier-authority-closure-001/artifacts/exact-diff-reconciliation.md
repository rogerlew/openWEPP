# Exact diff reconciliation

Status: reconciled / terminal dual verification PASS

Evidence mode: Static + Ran

Static: the protected Child 2B release is
`1d0239f4aab78966537c465bdfd4d1efc69f5ef1` (`origin/main` at intake). The
terminal Child 2C write set is limited to:

- `Cargo.toml`, only one package-owned `[[test]]` registration;
- the five named canonical science contracts and their index;
- the Child 2C package tree, including schemas, vectors, oracle, reviews,
  verification artifacts, and handoff;
- the explicitly reconciled campaign/roadmap/handoff documentation;
- `tests/integration/snow_stage3_shared_carrier_authority_contract.rs`.

Static: `git diff --name-only 1d0239f4a..HEAD` plus the current untracked-file
inventory contains no `crates/` or production source path. The only Rust
source addition is the contract integration test; no dependency, feature,
workspace, selector, default, or runtime target changed. Child 2B contract
and receipt content is consumed and protected; the one changed Child 2B
worker-handoff file is the authorized stale Child 2C handoff-line correction.

Ran: `git diff --check` passed. Ran: the Cargo diff contains only the named
test registration. Ran: strict Binding Exposure checks passed for the five
amended contracts with 4, 1, 13, 16, and 5 rows. Ran: the focused contract
gate passed 5/5 tests. Ran: scoped package Markdown lint passed 29 files with
0 errors and 0 warnings. A full `docs` lint inventory reports 15 unrelated
pre-existing broken-link errors outside this write set; they are not folded
into the Child 2C gate.

Ran after staging: the staged allowlist contains only the reconciled Child 2C
write set; no unrelated path enters the authority checkpoint.
