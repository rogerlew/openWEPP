# WEPPpy SSURGO 2006.2 Soil Provenance

Evidence class: `Ran`

Operator follow-on authority admitted WEPPpy's canonical SSURGO `2006.2`
serializer as a bounded hold-lift reconstruction for the exact site mukeys.
No WEPPcloud project rsync was needed.

Build environment:

- repository: `/workdir/wepppy`;
- source module: `wepppy/soils/ssurgo/ssurgo.py`;
- API: `SurgoSoilCollection([665220, 131976])`,
  `makeWeppSoils(initial_sat=0.75, ksflag=False)`, then
  `writeWeppSoils(version="2006.2")`;
- data: temporary copy of
  `wepppy/soils/ssurgo/data/surgo/surgo_tabular.db`, populated through the
  module's canonical SSURGO synchronization path;
- temporary output root:
  `/tmp/openwepp-cal02-2006-soils-z1YZUz`.

| Site | Mukey | Major component | Layers | SHA-256 |
| --- | ---: | ---: | ---: | --- |
| Hubbard Brook | 665220 | 14214185 | 3 | `b355411acf4f0e774b7e1d1d686f45c1c15c6ee5fa94f9857f1638934fa6d6af` |
| Santee | 131976 | 27097758 | 5 | `69ac7512339e978c6bbc70ff6d81ccbe37b8a05b09bbc8435d099ad3a34ed1fe` |

Both builds are valid and select the same major-component identities retained
in the source-native 9002 files. The files are WEPP `2006.2`, not `2006.5`;
the repository serializer names only `2006.2`. They reconstruct the site soil
from current authoritative SSURGO records and canonical WEPPpy computations.
They are not Bill's missing byte-identical Windows files, and differences from
the 9002 forest-modified operands remain part of the result interpretation.
