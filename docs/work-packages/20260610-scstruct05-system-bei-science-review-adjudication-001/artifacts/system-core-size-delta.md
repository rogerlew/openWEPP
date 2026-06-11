# SCSTRUCT05 System Core Size Delta

Evidence: Ran
Date: 2026-06-10

## Measurements

| Surface | Before SCSTRUCT05 | After SCSTRUCT05 | Delta |
|---|---:|---:|---:|
| `SC-SYSTEM-001.md` bytes | 128291 | 125779 | -2512 |
| Approx whitespace tokens | 14385 | 13939 | -446 |

Command snippets:

```console
wc -c docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md
python3 - <<'PY'
from pathlib import Path
p=Path('docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md')
print(len(p.read_text().split()))
PY
```

The core reduction comes from relocating three historical HPHYS profile-lineage
sections to `contracts/provenance/SC-SYSTEM-001-provenance.md`.
