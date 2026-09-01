# V55 private Q-lattice witness pre-implementation red

Evidence class: `[DIRECT][Ran] + [DIRECT][Static]`.

Retained canonical r139 failed closed at exact support `2100..2160 s` with
`phase-consistent authentic receipt-cycle endpoint witness`. The bounded r140
failure audit is `/tmp/wghl_001d_v55_64m_r140.log`, SHA-256
`3482ada5075aa921fc1e71d0f5fa253765b009fa4e833b05f3e9edc598147628`,
wall `6:45.03`, RSS `441756 KiB`. It proved a two-member exact receipt cycle,
shared budget `84 -> 86/96`, and no replay.

The two own-artifact attempts remained nonfixed:

- member 0: Q `5340.494294593433 -> 5340.494294593502 J m^-2`;
- member 1: Q `5340.494294593449 -> 5340.494294593433 J m^-2`.

Both retained unchanged branch, finite full residuals, and exact derived-z.
Static accounting places the first tolerance-closed private root near shared
budget 63, before 18 V45 polish charges and three receipt probes. Its canonical
same-map endpoint receipt Q is 21 positive binary64 values from the coordinate,
so a complete private candidate interval plus the two protected authentic
charges fits the unchanged maximum 96. V54 forbade this enumeration, providing
the expected contract/source red for V55.

Retained r142 `/tmp/wghl_001d_v55_64m_r142.log`, SHA-256
`4c4d63a75b39494b005cacc21d1d2777c03faeebff7af503a44d80ca1ebf7473`,
wall `5:08.77`, RSS `442360 KiB`, exposed the eligibility red. Its valid root
had merit about `0.010248`, budget `30/96`, and a 1394-member Q interval; the
required 1396 charges cannot fit, but unchanged V45 polishing remains lawful.
Therefore overcapacity is a zero-charge specialization miss, while malformed
shape, nonfinite Q, or stale canonical Q lineage remains a hard refusal.
