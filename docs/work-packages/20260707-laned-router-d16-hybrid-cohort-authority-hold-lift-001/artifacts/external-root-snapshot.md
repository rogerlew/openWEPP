# External Root Snapshot

Status: EXECUTED-HOLD-ROUTE-COEFFICIENT-AUTHORITY. Evidence mode: Ran.

This artifact strengthens the count-based `/wc1` evidence with compact list and
content digests. It does not make the external roots immutable; it records what
was inspected in this session.

## Management File List Digests

| Root | Count | Command | Digest |
|---|---:|---|---|
| `/wc1/runs/al/algebraic-radium/wepp/runs` | 44 | `find ... -name '*.man' -print \| sort \| sha256sum` | `bba16a6580497763e39b659989143396823e835f3f4f41b351d0ef954a58283d` |
| `/wc1/runs/un/unpalatable-rind/wepp/runs` | 40 | `find ... -name '*.man' -print \| sort \| sha256sum` | `cfe38d70f0e97a1d744eb9e579c4e8b864b18a471f30f327be6909025bd31529` |
| `/wc1/runs/ar/arboreal-dendrite/landuse` | 36 | `find ... -name '*.man' -print \| sort \| sha256sum` | `86e960463cdc2cc9723a176c4c5bb91f6639ee8e5d5dc1f37bfe9ca49cceadec` |

## Management File Content Digests

| Root | Count | Command | Digest |
|---|---:|---|---|
| `/wc1/runs/al/algebraic-radium/wepp/runs` | 44 | `find ... -print0 \| sort -z \| xargs -0 sha256sum \| sha256sum` | `31c6750577ff6c006bb4a47855baa1ea31ff1fce5f0b61101f748b3adafded56` |
| `/wc1/runs/un/unpalatable-rind/wepp/runs` | 40 | `find ... -print0 \| sort -z \| xargs -0 sha256sum \| sha256sum` | `faa34e91fe00d9f83e678e2bf1afe319346a0f80352105316e9b14ae9a6244f9` |
| `/wc1/runs/ar/arboreal-dendrite/landuse` | 36 | `find ... -print0 \| sort -z \| xargs -0 sha256sum \| sha256sum` | `8347fc0fa6a4ea2bfcbd399f181cb4c3f2eabb380a678862eac4fdaf32e3716e` |

## Interpretation

The route-coefficient hold still rests on the explicit zero-match scans in
`route-coefficient-authority-audit.md`; these digests provide a compact
provenance handle for the inspected external file sets.
