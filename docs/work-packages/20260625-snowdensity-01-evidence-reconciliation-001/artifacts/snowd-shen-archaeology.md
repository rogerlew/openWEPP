# `snowd.for` and Shen Archaeology

Evidence mode: Static.

## Pinned `snowd.for`

Source: `/home/workdir/wepp-forest_260430_baseline/src/snowd.for`.

Key observations:

- Lines around `112-116`: a 2007 Dun edit documents using a daily model in an
  hourly way. The active cold-snow branch uses daily mean temperature
  `(tmax + tmin) / 2 < 0` rather than the older hourly-temperature threshold.
- Lines around `124-129`: CRM Eq. 3.7.1 applies the settling factor, then clamps
  the factor to `1` when `densgy > ssd`. The historical `250 kg m^-3` threshold
  is still visible as a commented predecessor. H verified `snow.txt` field 3 is
  a legitimate `ssd` arm.
- Lines around `168-183`: two branches carry comments questioning whether
  `snodpt` should be replaced by prior-day snow depth for density mixing. That
  affects `densgt` and depth/density carry-state lineage.
- Lines around `294-299`: CRM Eq. 3.7.5 is present only in a commented block,
  with a source note that it differs from the user documentation. The active
  code path after the 2008 edit sets `snodpt`/`densg` earlier and then proceeds
  through melt/depth/density update without executing that commented block.
- The source therefore preserves unresolved authority questions: daily-vs-hourly
  regime control, `ssd` threshold authority, density-mixing state timing, and
  code-vs-documentation divergence.

SNOWDENSITY-01 disposition: these are not enough to select production physics.
They justify SNOWDENSITY-02 as a contract/ADR package that must decide whether
the opt-in model follows baseline legacy wobble, CRM documentation, or external
physics.

## Shen 2011/2012 Thesis

Source: `references/copyrighted/D_Shen_020312.pdf`. Local text extraction was
used only for search and summary.

Summary:

- The thesis studies WEPP snow distribution/drift, not bulk densification as a
  standalone physics model.
- It reports WEPP snow-drift subroutine problems in a Pullman-area application:
  slope azimuth handling, threshold velocity coding, and snow storage capacity.
- It includes field snow depth/SWE/density observations for a residue/tillage
  snow-distribution study and shows snow depth is strongly affected by residue,
  storage capacity, and drift/scour.
- It supports the broader point that snow-depth residuals may include spatial
  distribution and storage-capacity effects, especially in agricultural/residue
  settings.

Disposition:

- Shen is useful for SNOWDENSITY-02 context and for deciding what to defer. It
  is not the direct authority for Anderson/SNOBAL densification equations.
- SNOWDENSITY-02 should avoid claiming the SNOTEL mountain density residual is
  fully explained by the Pullman drift/storage-capacity thesis.
- Wind redistribution and storage-capacity remediation should remain separate
  unless the opt-in bulk solver fails specifically on distribution-controlled
  signatures after density/compaction physics is tested.
