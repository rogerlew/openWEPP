# Coverage Before

Ran: the true pre-production baseline was measured once in an isolated detached
worktree at scaffold commit `5e0e92c5865df38e20edbb75b15f50fee1eccc32`.
The next commit, `564ff95c`, is the first source/test characterization edit.

The historical `#[cfg(test)]` boundary begins at line 1,324, so production is
lines 1-1,323. One instrumented package-library traversal passed 100/100 tests
in 686.297 seconds (run ID `4b0009cc-dae9-4bc2-9a99-8a9e74790103`).

- production line coverage: 478/1,078 (44.34%);
- production region coverage: 732/1,760 (41.59%);
- production functions mapped: 50, with zero mapping failures;
- functions below 75% region: 24; functions at zero: 20.

The baseline source SHA-256 is
`476b78dc855e9626a170406b85fc1fec3563aae6b00cab8a97003521554c67bc`.
Evidence root: `/tmp/cqr-pre-heavy-true-baseline-ikB6hd/evidence`.

| Artifact | SHA-256 |
| --- | --- |
| LLVM JSON | `6d189655a684d88ef9267ed4ca2b66adedf65114349c964410d73da45ad40623` |
| LCOV | `ed7cba2c93433e1bf52516ebafd91031eb0d94b6a0e5709e5ea591a8a7712d8b` |
| CRAP JSON | `625e03848fc89e997cb83919842a2523c66b598ed3da997ad403f4d9f66f2844` |
| compact summary | `7bccf1a7953b5802ab6d3aa1decdf432a5c1c2c9e4477c25b92bdac9425b59a3` |
| 50-row function TSV | `dde53e7f85c00987dd1bfe4daf1dd9d2b20ceb6097d2c18ea7cc1c93f5fcaf29` |
| run log | `a8b51180187967027678cdcdf3bb5e541ee2f151f9f140985876daf26a99d344` |

Static: final production line coverage is 96.08% and region coverage is
89.64%; both strictly improve over this exact package base. The final function
floor also improves from 24 below 75% to zero.
