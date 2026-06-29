# Gradient Entry Gate

Status: `PASS`

The entry gate must prove the Stage 1 layered candidate develops a density
profile with basal snow denser than surface snow on real direct-production trace
rows before frost coupling is evaluated.

Ran:

- Command: `.venv/bin/python tools/snowfreeze_observed/paradigm2_stage2_insulation_profile.py`
- Artifact: `artifacts/paradigm2-stage2-gradient-entry-gate.json`
- Candidate trace rows: `159986`
- Multi-layer rows after snow step: `56831`
- Positive basal-minus-surface gradient rows: `49548`
- Material positive gradient rows (`>= 10 kg m^-3`): `48464`
- Negative gradient rows: `585`
- Max gradient: `446.5207296110246 kg m^-3`
- Min gradient: `-60.66582900192148 kg m^-3`

Disposition: the entry gate passed because the Stage 1 candidate develops a
material basal-denser-than-surface profile on real direct-production rows. The
negative transient rows are carried forward as diagnostic evidence but did not
block coupling because the material positive gradient is widespread.
