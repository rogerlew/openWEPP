# Vendored Hourly Archive Custody

The Dryad dataset is version 1, published 2019-01-31, DOI
`10.5061/dryad.c9h35`, under CC0-1.0. Its usage note identifies the hourly
observations as a cleaned, formatted version of UCAR RDA dataset `ds464.0`;
cite both sources when using the observations.

The hourly CSV is `1,206,721,342` bytes and `17,810,806` lines including the
header. Its SHA-256 is
`0cc82fbc5211c2c24b19653c4711d63a88fc4ed7bd90fc39cce84913d071f3a1`.

The exact path-scoped rule in the repository `.gitattributes` tracks this file
through Git LFS. A normal Git LFS-enabled clone may download the 1.2 GB object;
use `GIT_LFS_SKIP_SMUDGE=1` when only source inspection is needed. Ordinary CI
and tests do not invoke the opt-in full-corpus consumer or require the object.

From this directory, verify the installed Dryad files with:

```console
sha256sum -c SHA256SUMS
wc -l jennings_et_al_2018_file2_ppt_phase_met_observations.csv
```

The real opt-in consumer is `openwepp-snowbench jennings-phase`. Vendoring
changes data custody only; it does not change scoring or scientific authority.
