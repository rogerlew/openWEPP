# Snow and Frozen-Soil Dataset Provenance

Evidence class: Static

This report uses normalized local fixtures and retained evaluation results. The
large observation corpora are not all committed to Git. This record gives the
exact local documentation identities, licenses, and reacquisition routes needed
to audit those inputs; it is provenance evidence, not a substitute for the
source publications or data archives.

| Corpus | Local documentation | SHA-256 | License/access | Reacquisition or source |
| --- | --- | --- | --- | --- |
| Jennings hourly precipitation phase | `tests/fixtures/precip_phase_observed/jennings2018/README.md` | `b018efb56a9ed30206621d71214b300a060b5347bd67ee4f799647431ede2566` | CC0 source data; repository documentation | Dryad DOI `10.5061/dryad.c9h35`; verify normalized files and thresholds under the local README procedure. |
| NRCS SNOTEL daily SWE and depth | `tests/fixtures/snotel_observed/README.md` | `b702b96e85c39241e8ccad3b3e43334f8264945eb3bd62b5bc300073469651a5` | Public federal station records; repository normalization metadata | NRCS AWDB/SNOTEL station records identified by the per-site provenance files. |
| Harvard and Marcell canopy/open snow surfaces | `tests/fixtures/cancov_forest/README.md` | `703a138076900f24a3232457dfab8744e60f69ab196b4b361eeb12bbfedb268c` | Repository fixture documentation; source-specific access retained per site | Inspect the README and per-site manifests before reusing normalized observations. |
| Frost tube and soil-temperature sites | `tests/fixtures/snowfreeze_observed/README.md` | `93bb72a83571104095ee0bc66137b569f428aff175d63384a2043ce7803351a3` | Mixed public federal archives; source-specific terms | USGS Sleepers River DOI `10.5066/P96753GI`; NSIDC GGD498 DOI `10.7265/1mcs-q536`; NRCS SCAN Mandan AWDB; USDA-ARS Reynolds Creek archive. |

Normalized-input identity is retained through the originating evidence objects
and their content hashes. A reproduction must reacquire the cited corpus,
reapply the documented normalization and exclusion rules, and compare the
resulting identities before rerunning a scientific evaluation. The report does
not imply that every external archive byte is redistributed in its research
package.
