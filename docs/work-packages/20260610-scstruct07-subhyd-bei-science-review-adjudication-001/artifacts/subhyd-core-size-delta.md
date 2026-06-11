# SCSTRUCT07 SUBHYD Core Size Delta

Evidence: Ran
Date: 2026-06-11

## Measurements

| Surface | Before SCSTRUCT07 | After SCSTRUCT07 | Delta |
|---|---:|---:|---:|
| `SC-SUBHYD-001.md` bytes | 96231 | 97253 | +1022 |
| Approx whitespace tokens | 11347 | 11437 | +90 |

Command snippet:

```console
python3 - <<'PY'
from pathlib import Path
import subprocess
path='docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md'
current=Path(path).read_text()
before=subprocess.check_output(['git','show',f'HEAD:{path}'], text=True)
for name,text in [('before',before),('after',current)]:
    print(name, 'bytes', len(text.encode()), 'tokens', len(text.split()))
print('delta_bytes', len(current.encode())-len(before.encode()))
print('delta_tokens', len(current.split())-len(before.split()))
PY
```

SCSTRUCT07 did not reduce core size. The science-review result is map-in-core for
all rows, so the only size change is more precise BEI mapping notes.
