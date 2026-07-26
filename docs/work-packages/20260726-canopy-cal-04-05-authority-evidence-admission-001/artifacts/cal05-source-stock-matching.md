# CAL-05 Source and Stock Matching

Status: `EXECUTED / PARTIALLY_LIFTED / OPERATOR ASSISTANCE REQUESTED`

Harvard HF161 v20 resolves hemlock needles, white-pine needles, several
deciduous leaf groups, miscellaneous material, and a pooled
twig/bark/cone/seed class for Simes plots during 2005-2020. HF324 v5 supplies
companion Simes organic-horizon stock and a broader litter/soil synthesis.
HF324 also contains 28 exact plot joins between annual hardwood EMS foliar and
nonfoliar litter and 2014 organic-horizon carbon stock.

`cal05-hf324-plot-matching.csv` retains those 28 joins with source keys,
periods, row/replicate counts, units, means, and `use.not=1`. The deterministic
`tools/extract_cal05_matching.py` rebuild reads the original HF324 litterfall
and soil-carbon tables. Carbon stock remains `kg C/m2`, while annual litter
flux remains `g C/m2/year`; the table does not compare or convert them and is
partial evidence only.

These are strong site/plot evidence, but they do not isolate fine woody mass:
HF161 pools twigs with bark, cones, and seeds; HF324 pools nonfoliar wood with
reproductive material. Experimental hemlock-removal arms cannot represent an
undisturbed calibration target; only observational control plots are eligible.
HF161's documented Plot 7 basket-loss interval must remain missing.

Hubbard Brook fine-litter package 49.11 covers foliage, buds, seeds, fruits,
bark, and wood under 2 cm in 0.097 m2 traps. Its metadata says autumn samples
were sorted by species and tissue, but the public table retains total dry mass
plus limited leaf masses/counts rather than a complete mass partition.
Package 50.10 separately measures wood over 2 cm in adjacent cleared plots but
excludes dead-tree inputs. Neither object fills the fine-wood gap.

Marcell and Santee candidates remain context only because their material
classes or site matches are insufficient. Unknown fine wood is never zero.
CAL-05 may use these objects for source-sufficiency analysis and bounds, but
must not fit decomposition or declare source adequacy until separately weighed
fine wood is supplied on a compatible stock basis.
