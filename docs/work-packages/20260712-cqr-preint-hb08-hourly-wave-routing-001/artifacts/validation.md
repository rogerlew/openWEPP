# Terminal Validation

Evidence class: **Ran + Static**

| Gate | Result | Evidence |
| --- | --- | --- |
| HB08 focused | PASS | Two bounded A–H tests; final reviewer boundary test `1/1`. |
| Full orchestrator | PASS | `128/128` |
| W11C real consumer | PASS | `7/7` |
| Fixed target | PASS | CC `28`, coverage `92.398%`, CRAP `28.344`. |
| Function floor | PASS | Zero eligible production functions below 75%; `ws11_ntchr` is 100%/CRAP 4. |
| Format/Clippy/diff | PASS | All-target orchestrator/runner Clippy with `-D warnings`; format/diff clean. |

Whole-file LLVM: lines `1658/1908` (`86.897%`), regions `1918/2157`
(`88.920%`), functions `80/91`. Uncovered functions include non-production
surfaces; the eligible audit is clean. The sole raw sub-75 cargo-crap row is
`#[cfg(test)] ws11_geometry_detachment_mass`, excluded from production.

| Item | SHA-256 |
| --- | --- |
| `hourly.rs` | `27552e89a97522f9d11e595f5c058c7dfab9ff8d0707be0e664978c03790a91d` |
| `hourly_tests.rs` | `d52f385562690a59598534c0e56fbf94e73e5128158795028b71911c06b4d387` |
| Reviewed JSON | `97c9fc96badaaaa38b0cb73af8962c1e98657fc98f27b5e2a3a95c99c8d03d36` |
| Reviewed LCOV | `baecd085998b160a8b7abf220612d5bd138184a03b0341f64de2a3056eac9d04` |
| Reviewed CRAP | `15f96f2982ca98eabfc8552bad0a14e14893726355fcd38ce4e44dc0eebd2695` |

`/tmp/hb08-terminal3*` is **rejected as stale**: it predates the explicit
parse-overflow vector and reports `ws11_ntchr` at 68.75%. Only archived
`hb08-final-reviewed*` is terminal authority: CC 4, coverage 100%, CRAP 4.

Line governance: target `2,111` lines, WARN below the 3,000-line blocker;
private tests `1,711` lines, below WARN.

