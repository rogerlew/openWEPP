# Validation Receipt

Status: pass

Evidence mode: **Ran**

Working directory for every command: `/home/workdir/openWEPP`.

| Exact command | Exit / result |
|---|---|
| `.venv/bin/python -B docs/work-packages/20260803-snow-surface-eb-04x-harvard-depth-swe-geometry-interception-001/tools/analyze_harvard_pair_v2.py` | `0`; `trajectories=80 phase_rows=16466 profiles=260` |
| same exact analyzer command after outputs existed | nonzero as required; `RuntimeError: terminal-v2 output already exists` |
| independent inline Python hash/inventory reconstruction recorded in the execution transcript | `0`; every frozen input/protocol hash, geometry boolean, three screen states, and CSV row count passed |
| `git diff --check` | `0` |
| `markdown-doc lint --path <EB-04X package> --path docs/ROADMAP.md --path docs/planning/snow-surface-energy-balance-roadmap.md --path docs/work-packages/README.md --format plain` | `0`; 21 files, zero errors/warnings |
| `.venv/bin/python -m json.tool <terminal-v2 freeze>` and same for results | `0` for both |
| protected-path empty-diff assertion over `crates`, `tests`, `docs/specifications`, and the EB-04 predecessor package | `0`; empty diff |
| generated-bytecode absence assertion | `0`; no `__pycache__` directory |

## Terminal-V2 Output Identities

| Output | SHA-256 | Data rows |
|---|---|---:|
| `freeze.json` | `ea4af0c280de624a41e1e432aae71367e0e7fe4d10e5432945e4187e8a2b71ba` | n/a |
| `results.json` | `aab693960062edc19f4a862344800698d051acd6052d617b719c0c5d31def4b1` | n/a |
| `density-trajectory.csv` | `0247e70476a3bc5f2deb41342448e6bd5ed05d2e43ff9877ce1971f9d5b79fca` | 80 |
| `profile-density.csv` | `6356db7f8d391d4adffa4b2bcd08ba033774375eb41452b84e6eddcd3585cb7a` | 260 |
| `daily-phase-identity.csv` | `913c0e82117a35b09df3bc083fc977a213b37ce142cd21566dfd73951dd9b14b` | 16,466 |
| `paired-state-extrema.csv` | `36e065a55a59d3f415328cd7c5cc7d8f1effa6c3f60c1d08dea1609cebd8abac` | 24 |

The terminal-v2 tool SHA-256 is
`c082f6571b995d74043340bd12dcb7be33c98ec804c959efcf3041715a495f9b`
and reproduces its frozen input identity. The prospective protocol SHA-256 is
`8bdaa35eb93a295eafa6b0fb359090acf31999d68c08c0e82b585f960553d4d0`.
