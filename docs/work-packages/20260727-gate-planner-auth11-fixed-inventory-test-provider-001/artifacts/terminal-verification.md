# Terminal Verification

Evidence class: `Static + Ran`

Two independent read-only terminal verifiers returned `PASS` at clean current
HEAD `194b3d097bf5d87dfc078703fff3731a06d9126c`.

## Predecessor AUTH11 Reconstruction

Retained root: `/home/workdir/gate-auth11-reconstruction-002`

Exact source:

- base `e815821e4aa3691174f7ec1b3fffc7806dc6f1e5`;
- head `964b449a51163811f5737bff98b6295df52364d3`;
- package chain `READY`, no reasons, ID
  `4c0d4684d76177e461ee7d2558b312ca565a3033ec044e7206d18879cd98a384`;
- intent plan
  `ef301bb03f02049ee3c24e825169e6bd8a59b91dc3035feff2617d081c09662d`;
- terminal plan
  `653b2631e4df6dd7c2bc1686c6dacd6705949d458ab9571b81e29a440dd25b81`.

Retained SHA-256:

| Artifact | SHA-256 |
|---|---|
| `authorized-paths.json` | `67f8a5433376d42bb823b7cbcb46dd86f753bafaaed54fc4f478c329ee935b06` |
| `package-authority-chain.json` | `1ac3a84fd4b830077c34fa4a24bc4999f9b239e36e9c0ade38975259f96767bc` |
| `intent-plan.json` | `31d3c67e9dc1db21fbf798a42c3b4a5a9d6af7d2a68ef4196c1be37201bf699a` |
| `terminal-plan.json` | `14a6e29b9a619839688ca252c0ff5806aa9d0437979ad5e62a4b2fc0b4171c18` |

Both verifiers independently reproduced:

- red: 12 nodes / 2,376 globally unique / 3,090 summed per-node /
  2,350 workspace;
- green: 13 nodes / 2,378 globally unique / 3,095 summed per-node /
  2,352 workspace;
- one AUTH11 node with exact argv and three exact independently enumerated
  inventory IDs;
- AUTH11 prerequisite IDs resolve exactly to generated admission and
  anti-evasion nodes;
- all prior authority, alignment, and workspace-full nodes remain present.

The old incomplete plan remains retained and unexecuted. The green
reconstruction is planning evidence only; it has no LIGHT, audit, ledger,
receipt, or HEAVY artifact.

## Successor Security Gate

Both verifiers confirmed:

- the Rust diff from accepted pre-edit head `5993320c` is confined to
  `planner_coverage_tests.rs`;
- only the AUTH11 `FixedInventory` branch and the named deterministic-test
  assertions changed;
- remaining successor paths are authorized evidence;
- no production Rust, policy, fixture, schema, executor, verifier, inventory
  mode, minimum count, enumeration, or fail-closed behavior changed.

Independent current-head reruns:

- verifier A full planner 227/227, run
  `74acb474-226f-48ae-b65b-6aaf3631c8d7`;
- verifier B full planner 227/227, run
  `d9d487bd-0550-4f95-b0d5-f68c5a2c7919`;
- AUTH11 obligation guards 3/3, strict Clippy, and diff hygiene pass.

## Current-Head Plan Preview

Retained root:
`/home/workdir/gate-auth11-test-provider-reconstruction-001`

At head `194b3d09`, package chain is `READY`, intent plan ID is
`648c3e6e76acb4f58e142e0a52dd017943510f996ab3ced458b72f073e6f45d6`,
and terminal plan ID is
`990a73fee05af04f554a501b0d676086fd0036928987d8b9137081435f0cca3f`.
The exact test-only successor diff correctly selects 12 nodes, 2,378 globally
unique IDs, 3,092 summed per-node entries, and workspace-full 2,352. It omits
AUTH11 because none of the successor's test/evidence paths is an
external-authority posture surface; the predecessor reconstruction above proves
the AUTH11 policy-change selection.

No HEAVY, CAL population, or Harvard access occurred.
