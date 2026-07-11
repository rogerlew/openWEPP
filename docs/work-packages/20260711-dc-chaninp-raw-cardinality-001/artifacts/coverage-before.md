# Coverage/CRAP before

Status: complete
Evidence mode: Ran

The contract-correction safety net, immediately before decomposition, passed
35/35 focused parser tests. Targeted coverage was 650/698 lines (93.123%),
704/726 regions (96.970%), and 32/34 functions. The only eligible function
above the CRAP ceiling was `parse_required_branch`: cyclomatic complexity 41,
86.592% region coverage, CRAP 45.052.

Raw evidence:

- `coverage-before.json` SHA-256
  `a1078ec1403cf470d6ec2a88518a6cb0193635955300c8040e5796e1b8ece7eb`
- `lcov-before.info` SHA-256
  `8343e713c02b16caa86f4f90fcec11384d450e0191638f20e44299451362c629`
- `crap-before.json` SHA-256
  `03d375c6da12b814cb981bb24642eca7d690de899590b3bdaa7d7d5c8e00be98`

The LCOV/CRAP capture preceded two additional `dtchr` characterization cases;
the JSON figure is the final pre-decomposition safety-net capture. Production
source was unchanged between those captures.

## Review-corrected monolith reconstruction

To supply independently reproducible source-state provenance after review
corrections, the finalized behavior was reconstructed in an isolated mirror as
one monolithic `parse_required_branch`; the terminal workspace source was not
edited. The snapshot is retained as `chaninp-monolith-reconstruction.rs` at
SHA-256
`63f700ff562fd9fb351ee9ce6cc95faf89db055d7981fb4561a5149b3f7f2dbd`
(28,745 bytes, 965 lines). It used the exact terminal focused test at SHA-256
`bb7475b308acd1364e7c8037fc4495a321ec1de8d46abb78a0fdf4c62f620c9e`
and passed 36/36.

Exact measurement commands used `CARGO_TARGET_DIR=/tmp/openwepp-fq02-reconstruct-target`:

```text
cargo llvm-cov clean --workspace
cargo llvm-cov --workspace --test infile_chaninp_parser_contract --lcov --output-path /tmp/chaninp-fq02-reconstruct.lcov
cargo llvm-cov --workspace --test infile_chaninp_parser_contract --json --no-clean --output-path /tmp/chaninp-fq02-reconstruct.json
cargo crap --workspace --lcov /tmp/chaninp-fq02-reconstruct.lcov --min 0 --format json --output /tmp/chaninp-fq02-reconstruct-crap.json
```

All exited zero in 0.38, 37.76, 1.04, and 1.16 seconds. Target coverage was
652/706 lines (92.351%), 708/733 regions (96.589%), and 32/34 functions.
`parse_required_branch` had CC 42, 83.957% coverage, and CRAP 49.283, proving
the finalized monolith both met the science-tier safety net and retained the
eligible decomposition target.

Reconstruction raw evidence:

- `lcov-reconstruction.info`: SHA-256
  `f5d5b88ab52abd65125d7f2592a2d774520b23410d3c06688dab1fb90ca8c7db`,
  208,169 bytes.
- `coverage-reconstruction.json`: SHA-256
  `b3dc21a7e33f5facaa7ada02f64ee0bfb4a379cc6cb3e2d06d064fd73980503d`,
  1,067,845 bytes.
- `crap-reconstruction.json`: SHA-256
  `13cbb36851014914b942fe705caf16ba03b9ec35bfd05c54e9f70713e07ede46`,
  2,850,604 bytes.
- Exact timing files are retained as `reconstruction-{clean,lcov,json,crap}.time`.

This labeled reconstruction supplements, rather than rewrites, the
contemporaneous pre-decomposition raw capture above.
