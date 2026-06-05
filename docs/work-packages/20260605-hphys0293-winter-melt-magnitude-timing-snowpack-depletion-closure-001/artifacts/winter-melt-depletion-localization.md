# Winter Melt Depletion Localization

Status: complete
Evidence mode: Ran

Ran:

- Full H1..H39 suite and H1/H7/H39 target traces completed under `/tmp/hphys0293_full_20260604T212429Z`.
- Focused contract tests and prior HPHYS0284/HPHYS0292 guards passed.

Findings:

- WB14 runoff ownership remains excluded for the target residual: full-suite `Q` parity is `39/39`, and target-row `ΔQ` is zero within floating tolerance.
- Snow-state accounting is internally closed: representative H1/H7/H39 target rows have `SWE closure` equal to zero within trace tolerance.
- Candidate spring `Snow-Water` is persistently below the pinned comparator before terminal depletion:
  - H1 2014-132: `ΔSnow = -27.366889 mm`.
  - H1 2014-133: `ΔSnow = -31.591426 mm`.
  - H7 2014-133: `ΔSnow = -27.980960 mm`.
  - H39 2014-132: `ΔSnow = -28.364899 mm`.
  - H39 2014-133: `ΔSnow = -32.666437 mm`.
- Terminal `RM` deficits track the previously accumulated snow deficit rather than a new WB13 inference defect:
  - H1 2014-145: `ΔRM = -21.008350 mm`.
  - H7 2014-146: `ΔRM = -16.787045 mm`.
  - H7 2016-111: `ΔRM = -15.906187 mm`.
  - H39 2014-145: `ΔRM = -22.466657 mm`.
- The evidence is consistent with corrected negative-melt carried-state authority creating semantic divergence from the pinned comparator without authorizing empirical WB18/WB19/WB17 compensation.

Disposition:

- No production physics patch was applied in HPHYS0293.
- Continue with post-ingress storage/percolation/lateral retention work only with the HPHYS0293 snow-producer residual separated from WB14 ownership.
