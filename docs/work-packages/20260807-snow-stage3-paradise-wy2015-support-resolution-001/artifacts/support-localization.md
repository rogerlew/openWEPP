# Paradise WY2015 Support Localization

Status: `PASS / exact support cause localized`.

Evidence mode: `Ran`.

Immutable attempt 002 at `167e2021a` reconciles the parent row exactly:

- affected hours: `202` = `19` partial + `183` unmatched;
- omitted magnitude: `98,075,671.33551113 J m^-2`;
- support ratio remains `0.06217301915749281` against frozen `0.05`;
- result SHA-256: `871122d75395adb1ae3b0c735d782f8611b052ec14cd0a8c95c9ea3b5a4af1f4`;
- receipt SHA-256: `b388c495d0cb99f3da198d17d5a16b4d7e6f62772845ecd50e38016924bc6e41`.

## Direct Cause Evidence

All 19 affected dates have the same exact sequence:

1. S remains a full-hour evaluated immutable tuple.
2. Q evaluates for `300--3300 s`, then its final tuple reports
   `post_substep_no_resolved_surface`; final retained ice is
   `0.956002--0.999927 kg m^-2`.
3. Every later affected hour that day has S status `evaluated` and Q status
   `thin_pack_boundary_reached`, producing 183 S-only hours.

This directly identifies why common support ends: the sequential evaluator
reaches its thin-pack/no-resolved-surface boundary while the immutable arm does
not mutate out of support. It does not prove the boundary is physically
correct, the snow state is realistic, or the omitted energy is noise.

## Episode Inventory

| Date | Partial hour | Q support (s) | Affected through | Hours | Omitted (MJ m^-2) |
| --- | ---: | ---: | ---: | ---: | ---: |
| 2014-10-27 | 15 | 1920 | 23 | 9 | 2.359350 |
| 2014-11-01 | 16 | 1800 | 23 | 8 | 0.583006 |
| 2014-11-25 | 16 | 3300 | 23 | 8 | 9.860325 |
| 2014-11-26 | 16 | 3300 | 23 | 8 | 6.915105 |
| 2014-11-27 | 5 | 360 | 23 | 19 | 22.224796 |
| 2014-12-05 | 20 | 300 | 23 | 4 | 1.175682 |
| 2014-12-06 | 13 | 2700 | 23 | 11 | 4.377329 |
| 2014-12-11 | 12 | 1920 | 23 | 12 | 7.154668 |
| 2015-01-24 | 21 | 420 | 23 | 3 | 2.919493 |
| 2015-02-02 | 13 | 540 | 23 | 11 | 4.956893 |
| 2015-02-25 | 10 | 840 | 23 | 14 | 2.716813 |
| 2015-03-05 | 10 | 600 | 23 | 14 | 3.499820 |
| 2015-03-15 | 12 | 900 | 23 | 12 | 3.166249 |
| 2015-03-16 | 9 | 1080 | 23 | 15 | 7.418616 |
| 2015-03-21 | 12 | 3000 | 23 | 12 | 6.201421 |
| 2015-03-22 | 15 | 1020 | 23 | 9 | 1.464877 |
| 2015-03-26 | 14 | 960 | 23 | 10 | 4.070206 |
| 2015-04-09 | 12 | 720 | 23 | 12 | 3.174995 |
| 2015-04-10 | 13 | 1320 | 23 | 11 | 3.836028 |

## Omitted Magnitude by Term

| Term | MJ m^-2 | Share |
| --- | ---: | ---: |
| Sensible | 52.355416 | 53.383% |
| Latent | 22.732252 | 23.178% |
| Longwave | 13.600082 | 13.867% |
| Shortwave | 8.121750 | 8.281% |
| Advected | 1.266172 | 1.291% |

Partial hours contribute `6.428157 MJ m^-2`; the 183 subsequent unmatched
hours contribute `91.647515 MJ m^-2`.
