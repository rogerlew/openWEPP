# Verification

Status: PASS. Evidence mode: Static + Ran.

Verifier: Codex `comparator_suite_runner` subagent.

Ran:

- `git branch --contains b1d5fd4410b700012d857ef4056000163e6aa6a0`
- `test ! -f docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`
- `python -m json.tool` on baseline and after identity JSON files
- Targeted content/reference checks for hybrid and `SC-OFEROUTE-002`
  references
- JSON hash comparison over baseline/after members

## Results

- PASS: archive branch `abandoned/hybrid-implicit-stepping` exists at
  `b1d5fd4410b700012d857ef4056000163e6aa6a0`.
- PASS: `SC-OFEROUTE-002.md` is deleted from active main tree and marked
  `withdrawn` in `docs/specifications/science-contracts/index.md`.
- PASS: `SC-OFEROUTE-001` records the ADR-0037 removal posture and retains
  hybrid-era rows as historical provenance.
- PASS: active-plain HBP and pass-parquet hashes are identical before/after
  for `h2637`, `mn_corn_h4`, `n_idaho_forest_h1`, and
  `wa_cascades_forest_h1`.
- PASS: raw run dirs are not required as committed evidence; committed
  evidence is the script, logs, JSON, and Markdown hash summaries.
- PASS: no live hybrid implicit references remain beyond allowed ADR /
  historical docs and the explicit env-rejection path.

Blockers: none.
