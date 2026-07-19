# Reproduction And Correction

Evidence class: `Ran` and `Static`

Retained failures:

- receipt `601158fb9c26446fec5af28cb8663a55a7de8223197cf9eea428dbd2eb2d6e37`:
  two assurance socket cases fail with `path must be shorter than SUN_LEN`;
- short-root receipt
  `49ecc4a97cfa2e67925140067b9999b8d808a4aeda442b1ef3899644408f0c27`:
  the public socket case passes, while the longer retired-root fixture still
  fails.

The correction diff is exactly one insertion and one deletion: temporary
scratch label `assure03-release-symlink-preflight` becomes `p`. The socket
target remains `assurance/dossiers`; all fixture content and assertions remain
unchanged.

Pre/post inventories:

| Construct | Before | After |
| --- | ---: | ---: |
| assertions | 82 | 82 |
| tests | 13 | 13 |
| `UnixListener::bind` | 3 | 3 |
| `transition_fixture(` | 11 | 11 |
