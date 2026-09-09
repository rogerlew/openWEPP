# Source and build manifest

Ran: Q is `/tmp/openwepp-attribution-Q-3r3eP8`, copied from verified P custody
before edits. The exact P-to-Q seven-file patch is
`combined-instrumentation.patch`, SHA-256
`a254e07c09953339bb8f7c9ecfe354c799fc8d41db436ceb6684835cef4550f4`.

Final release runner:
`/workdir/.cache/openwepp/targets/openwepp-attribution-Q-3r3eP8-a0d08c9a4ce0/release/deps/openwepp_runner-682944f41aa5eb67`, SHA-256
`3e684ac3e8b4dcc93f0dfcffe88faa0c18289f39770237ef4bd404f782d4ce53`.
Sidecar SHA-256 is
`f2ece6f207d4473d11ad4a35dd12d778419c8f6cba4791fa280b2e5734fb600c`;
admission identity SHA-256 is
`f9f2d53217f2a60d4b5ee06f3b10926e9962cad393abebabdd673f25d574df06`.

Final series is `raw/series-05`; protocol SHA-256
`201fc3abbc1a886daa69aa700125e1879989d6f6fe087afae123bb7cebfc60e3`.
Final cost analysis SHA-256
`1088a52ab64a64af1993986761afc4171f228f02f7b9867e126cd025d108d497`.
Exact-source detailed carrier trace SHA-256
`9290f1f37728bea8d7b29ab60de65760829e8aad3268d54629e71b98a157f302`;
portable recomputation analysis SHA-256
`c5f72d64b39362a2b4ce50a4a522d7b51dfa516338848fde220f10a14de77ce3`.

The runtime environment input remains external at
`/tmp/openwepp-compact-cost-P-vKI6gb/custody-stage/fixture/runtime-environment.json`,
SHA-256
`463cd045d5413611e2785f80d12f58803f36e1db5ac39533cb7af03011074a84`.
It is bound by `raw/series-05/protocol.json`. Commands and dependencies are
enumerated in `command-and-dependency-receipt.md`.

Final external custody bundle is `/tmp/openwepp-attribution-Q-final-bundle`;
manifest SHA-256
`de78b15b7fa1f69252fdd073002fe6c25fc45bc0552e354045adcc45615291e5`.
It captures the seven changed source files, exact executable/sidecar, identity,
protocol/scripts/patch, and compact analyses. Complete source/toolchain and raw
series remain external/committed dependencies; custody is not rebuildability.
