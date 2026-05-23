# WB16 Peak-Flow Kernel Authority and Guard Map

Status: `completed`
Evidence mode: `Static`

## Canonical WB16 Runtime Inputs
- Runoff and forcing symbols: `Q`, `ninten`/`nbrkpt`, `timem_####`,
  `intsty_####`, `I`, `irrigation.runtime_rate_m_per_s`
- WB16 branch symbols: `timep`, `efflen`, `ealpha`, `m`

## WB16 Runtime Algorithm Map
1. Resolve hyetograph series and event duration: `effdrr = timem_last - timem_first`.
2. Compute mean runoff and normalized rates:
   - `vave = Q / effdrr`
   - `remax = max(intsty_####) + irrigation.runtime_rate_m_per_s`
   - `vstar = vave / remax`
3. Compute time ratio:
   - `te = (efflen / (ealpha * vave^(m-1)))^(1/m)`
   - `tstar = te / effdrr`
   - `tc = timep`
4. Branch-authoritative `qpstar`:
   - `tstar >= 1`: `qpstar = 1 / tstar^m`
   - `tc < tstar < 1`: `qpstar = 1 / tstar`
   - `0 < tstar <= tc`: `qpstar = 1/vstar - 0.6*((1-vstar)/vstar)*tstar`
5. Peak and duration outputs:
   - `peakro_raw = vave * qpstar`
   - `peakro = max(peakro_raw, 3.63e-8)`
   - `watdur = min(Q/peakro, 86400)`
6. Emit trace symbols: `wb16_peak_method_branch`, `wb16_tstar`,
   `wb16_qpstar`, `wb16_vstar`.

## Guard-Family and Status Mapping
- Phase class: `HydrologyPeakRunoff`
- Success: `HKERNEL-WB16-PEAK-OK-001`
- Missing required symbol: `HKERNEL-WB16-PEAK-E-001`
- Non-finite symbol: `HKERNEL-WB16-PEAK-E-002`
- Domain/closure violation: `HKERNEL-WB16-PEAK-E-003`
