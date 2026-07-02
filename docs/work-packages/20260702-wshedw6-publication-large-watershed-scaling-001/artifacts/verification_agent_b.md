# Verification Agent B

Status: `passed`

Evidence mode: `Static:` artifact verification plus `Ran:` fixture and scaling
evidence review.

Verification focus: committed-fixture auditability, scaling evidence,
legacy-comparison taxonomy, and fixture manifest integrity.

Result:

- `onshore-xenophobia` is a committed full `1305`-hillslope fixture with README
  and checksum manifest.
- `carnivorous-adobo` includes committed W6 launch runfiles and checksum
  manifest.
- Both fixture manifests validate with `sha256sum --quiet -c`.
- Full current W6 scaling artifacts record no subsetting and prove output
  identity across job counts for all required watershed parquet outputs.
- Pinned legacy evidence is same-fixture completion evidence and does not claim
  output parity or cross-scope speedup.

Verification verdict: `PASS`.
