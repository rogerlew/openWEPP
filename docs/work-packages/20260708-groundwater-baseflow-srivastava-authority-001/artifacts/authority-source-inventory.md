# Authority Source Inventory

Status: executed.

Static evidence was gathered with targeted PDF text extraction into
`/tmp/openwepp_gwbaseflow_pdf/` and source-level reads from the pinned baseline.
No paper text extracts are committed.

| Source | Static evidence | Contract use |
|---|---|---|
| Srivastava (2013) dissertation | Local PDF exists at `/workdir/wepp-forest/references/Srivastava_Diss2013_14.pdf`; Chapter 2 defines a linear groundwater reservoir with WEPP deep percolation input, baseflow coefficient, deep-seepage coefficient, storage state, and daily recurrence. Appendix C includes the matching daily vector update and fitted coefficient workflow. | Primary process authority for `S_i`, `D_i`, `Qb_i`, `Qs_i`, `bfcoeff`, `dscoeff`, and `igwstrd` in `SC-GWBASEFLOW-001`. |
| Srivastava et al. (2013) | Local PDF exists at `references/copyrighted/Srivastava2013.pdf`; verified as the ASABE paper, not the dissertation. The paper presents WEPP streamflow components including generated groundwater baseflow and the same linear-reservoir storage/baseflow/deep-seepage structure. | Peer-reviewed companion authority for the linear-reservoir process and WEPP coupling. |
| Srivastava et al. (2017) | Local PDF exists at `references/copyrighted/Srivastava2017_ToASABE_wepp_streamflow.pdf`; text confirms later baseflow lineage and nonlinear/storage-residence extensions. | Context only for lineage terminology; nonlinear algorithms are explicitly out of current `gwcoeff.txt` authority. |
| Dun et al. (2009) | Local PDF exists at `references/copyrighted/dun2009.pdf`; text supports WEPP forest subsurface/deep-percolation/lateral-flow context. | Companion context separating forest lateral subsurface flow from the later groundwater-reservoir baseflow routine. |
| `references/annotated_bibliography.md` R-21, R-22, R-22A, R-70 | Entries already identify source roles, local paths, and caveats. | Bibliographic routing; no source-role correction was needed during execution. |
| `/workdir/wepp-forest_260430_baseline` | Repository resolves to `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`. Required files exist: `main.for`, `contin.for`, `wshpas.for`, `wshdrv.for`, `wshchr.for`, `wshcqi.for`, `watbalprint.for`, plus `cchrt1.inc` and `cstore2.inc` for symbol definitions. | Normative implementation provenance for branch selection, state variables, recurrence order, pass handoff, watershed/channel consumption, threshold behavior, and publication semantics. |

## Accepted Authority Decisions

- `SC-GWBASEFLOW-001` is a new process contract. `SC-INFILE-GWCOEFF-001` owns
  sidecar parsing only and is not the process-physics authority.
- The accepted process is the Srivastava linear-reservoir routine driven by
  WEPP deep percolation. Later nonlinear baseflow work remains lineage context,
  not implementation authority.
- Groundwater-reservoir baseflow (`Qb_i`/`gwbfv`), deep seepage
  (`Qs_i`/`gwdsv`), lateral subsurface export (`latqcc`), and `chan.inp`
  `cbase` are separate namespaces.
- Missing `gwcoeff.txt` disables this reservoir process. Present but malformed,
  out-of-domain, or mixed Lane D authority fails closed; no coefficient defaults
  are inferred.
