# Owned File Manifest

Status: `PASS at closure candidate`.

Evidence mode: `Ran`.

Base is the parent of the result-blind scaffold:
`3e4c2b76eeec1fc84207e3a1ca5295b690ef9d36`. The closure-candidate diff has
`113` tracked paths, all in the prospectively declared write set:

| Group | Paths | Disposition |
| --- | ---: | --- |
| Package-local plan, prompt, artifacts, analyzer, and tests | 43 | Declared package tree; kickoff archived with identical SHA-256 `6b87603a...`. |
| Rust production and protected-shape test | 15 | Declared additive diagnostic write set. |
| Integration/contract-version tests | 38 | Declared exact v129 binding and observability tests. |
| Canonical contract and index | 2 | Declared `SC-SNOWFREEZE-001` amendment. |
| Assurance generated custody | 3 | Identity lock, snow report review lock, typed transaction `31798778...`. |
| Roadmaps and package catalog | 3 | Declared disposition surfaces. |
| Root `Cargo.toml` | 1 | Declared integration-test registration. |
| Assurance release guard | 1 | Prospectively admitted correction excludes only governed nonpublic review drafts from the public inventory. |
| Generated assurance review drafts | 7 | Review index plus six snow DRAFT build/prose/research-object files selected by the canonical renderer. |

There are no fixture, observation, dependency, approved public output,
selector-default, or unrelated crate changes. Rejected v1/v2 and admitted v3
bulk outputs remain ignored and read-only under their distinct target
namespaces. The final index and worktree are clean; terminal verification must
recheck this inventory against the exact closure commit.
