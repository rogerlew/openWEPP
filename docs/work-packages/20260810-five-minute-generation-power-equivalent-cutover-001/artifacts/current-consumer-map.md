# Current Consumer Map

Status: `Static audit complete — retained under prerequisite HOLD`

Evidence mode: `Static`

## Operand map

| Operand | Units / support / basis | Current source and use | Power-equivalent disposition |
|---|---|---|---|
| authoritative hourly runoff depth `q_h` | `m`; one hour; OFE depth | Closing WB14+WB19 ledger; water authority; crosses HBP as `V_h=q_h*Area` and inter-OFE water. `runoff.rs:1450-1467,1630-1640`. | Never replace. |
| WB16 `peak_runoff_rate_m_s` | `m/s`; maximum one-hour mean; hillslope/OFE depth | `max(q_h/3600)`; public output multiplies area once. `runoff.rs:1576-1600`; `01_publication.rs:629-640`. | Never replace. |
| hourly Wave-1 `peakro_m_s` | `m/s`; representative hourly erosion quantum; local OFE | Currently `q_h/3600`; drives local hydraulics and `qout`. `erosion.rs:771-783`; `erosion_seed.rs:332-341`. | Eligible only after a typed erosion/water split and only in the bounded single-OFE domain. |
| `effdrn_s` | `s`; representative erosion rectangle | Currently `3600`; rate times duration reconstructs `q_h`; controls normalization and exported load. `erosion.rs:782`; `erosion_continuity.rs:2214-2219,2272-2279`. | A candidate must carry the volume-equivalent duration; fixed-hour power mean alone is inadmissible. |
| `effdrr_s` | `s`; rainfall-excess support | Sum of positive rainfall-excess interval durations; interrill REID operand. `erosion_operands.rs:70-132,903-943`. | Never replace; rainfall semantics remain unchanged. |
| `qout_m2_s` | `m2/s`; representative quantum; unit width | `peakro*efflen`; both local hydraulic forcing and inter-OFE water handoff. `erosion_seed.rs:332-341`; `erosion_continuity.rs:2473-2481`; `03_executor.rs:1058-1063`. | Split meanings; boundary remains arithmetic-hour mean. |
| `qin_m2_s` | `m2/s`; hour mean; unit width | Prior-OFE `qout`, carrying water/sediment continuity. `erosion_seed.rs:192-218,552-579`; `03_executor.rs:1074-1081`. | Never replace in V1. |
| `qshear_m2_s` | `m2/s`; representative quantum; rill spacing | `qout*rspace`, or `qin*rspace` under full reinfiltration. `erosion_seed.rs:332-362`. | Candidate-local only where `qin=0`. |
| rill width | `m`; persistent across chronological hours/days | Nonlinear `qshear^0.303` growth capped by spacing; persistent production state. `erosion_operands.rs:729-781`; `erosion.rs:732-736,800-820`. | Clone for diagnostics; a real cutover would intentionally alter it. |
| shear | `Pa`; local representative quantum | Nonlinear Chezy depth/hydraulic-radius response to rate, width, and slope. `erosion_operands.rs:710-781,801-857`. | Candidate constitutive response. |
| transport capacity | `kg m^-1 s^-1`; representative quantum | Yalin/shear response; `kt=tottc/shear^1.5`, `tcend=kt*shear^1.5`. `erosion_operands.rs:520-643`. | Candidate constitutive response; coefficient carry prevents MOFE adoption. |
| deposition driver `phi` | dimensionless; representative quantum | `beta*veleff/((qout-qin)/slplen)`. `erosion_continuity.rs:2222-2241`. | Candidate-local only; `qin` coupling makes MOFE ineligible. |
| sediment load / `S_h` / `qsout` | `kg/m`, `kg`, `kg m^-1 s^-1` | Exported load uses `effdrn`; `S_h` is hour-integrated mass; inter-OFE `qsout=S_h/(width*3600)`. `erosion_continuity.rs:2272-2378`; `erosion.rs:855-903`; `03_executor.rs:1058-1064`. | Candidate may change single-OFE `S_h`; boundary discharge keeps the arithmetic-hour time base. |
| HBP | `V_h` in `m3`, `S_h` in `kg`; 24 slots | Serialization/publication surface. `04_direct_publication.rs:401-423,480-528`; `SC-INFILE-HBP-001.md:116-124`. | Water unchanged; no five-minute forcing enters HBP. |
| public peak | `m3/s`; maximum hourly mean; hillslope area | Publication-only, and HBP reconstructs it from `max(V_h/3600)`. `SC-WATBAL-001.md:129-131,297-299`; `SC-INFILE-HBP-001.md:116-124`. | Unchanged. |

## Milestone 2 answers

1. A distinct erosion hydraulic rate is structurally possible only after the
   current conflated `peakro_m_s`/`qout` meanings are split. Water discharge
   must remain the hourly mean.
2. The Wave-1 solver requires representative rate times effective duration to
   reconstruct hourly runoff depth. Therefore only the two-moment
   power-and-volume rectangle is structurally admissible; a fixed-hour power
   mean alone is not.
3. Nonlinear dependencies include rill width, Chezy depth, shear, Yalin
   transport capacity, `kt/tcend`, detachment capacity, and the reciprocal
   deposition driver.
4. Hourly water depth, WB16/public peak, HBP water, inter-OFE `qin/qout`,
   routed water, and rainfall `effint/effdrr` remain arithmetic-mean authority.
5. A variable erosion duration breaks current multi-OFE handoff because
   water/sediment boundary discharge is explicitly normalized over 3600 s
   while receiver quanta also use `effdrn=3600`.
6. A V1 domain can be bounded to single-OFE rainfall-driven local runoff hours,
   subject to the later contract, constitutive, source-completeness, mutation,
   and real-consumer gates. This static answer is not adoption authority.

Important naming finding: daily WB16 `peak_runoff_rate_m_s` is copied into an
erosion `peakro_m_s` slot, but the hourly production plan overwrites that slot
with each hour mean. Any resumed design must eliminate that ambiguous reuse.
