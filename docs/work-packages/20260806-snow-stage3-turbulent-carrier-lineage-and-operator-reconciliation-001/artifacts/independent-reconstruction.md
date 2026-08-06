# Independent Reconstruction

Status: `PASS`.

Evidence mode: `Ran` twice from retained real schema-v6 rows at exact clean
`5ebfc5135b80d250cb6b38d1b6241a7d2a72d6c5`.

The tracked independent consumer completed the frozen four-site reduction and
then completed `--verify-existing` against the retained namespace. Verification
bound `143` artifacts, the execution receipt, exact binary, frozen inputs,
inventories, and raw result to the execution commit.

| Check | Result |
| --- | --- |
| Primitive sensible, latent/vapor, shortwave, longwave, and advected reconstruction | `PASS` |
| Complete external and legacy external-plus-active-conduction totals | `PASS` |
| Sequential mass and total-cold endpoint closure | `PASS` |
| Same-state unchanged endpoint identity | `PASS` |
| Sequential after-to-next-before continuity | `PASS` |
| Daily/hourly identity, fingerprint, ordering, and support joins | `PASS` |
| Three-way common-support reduction and signed omission inventories | `PASS` |
| Shortwave invariance and projection/evolution delta closure | `PASS` |
| Known-alias anti-tautology vectors | `PASS` |
| Exact retained-artifact verification | `PASS` (`143/143`) |

Retained hashes:

- raw result: `3b885fa0f04201744da5c24766d413cd2e74f1273021a1a35d6fd0f7227f691e`;
- execution receipt: `61564035575b165722213abe0657a4dc70b04a1d72c1200b1bb5e35d435fdc9e`;
- manifest: `b0c161e7915a8fc52d5da12fb3b17d189480cceae0114cc9fda7d34fd7dedfcf`;
- release binary: `c8d433af71911befb174c17c843a0b4a77a95936702683e54bc9569746782b09`.

The predecessor estimator alone fails its frozen target. That is a scientific
custody result, not a reconstruction failure: current schema-v6 legacy
reconstruction closes internally but does not reproduce the earlier schema-v5
trajectory.
