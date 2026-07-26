# Reproduction Verdict

Evidence class: `Ran`

Verdict: `NOT_REPRODUCIBLE`

The authorized WEPPpy SSURGO `2006.2` soils lift the executable-format hold:
all five 100-year arms complete under the exact WEPP 2012.800 executable. This
is not Bill's exact Windows reconstruction because his manually transcribed
soil/project remains missing and the reconstructed soils retain current SSURGO
properties.

The delivered Hubbard `dropfc=0.95` arm reproduces reported live biomass
(18.871 versus about 19 kg/m2) and the current/previous residue pools within
the frozen chart tolerance. Its old and total residue are high (3.513 and
5.280 versus 3.2 and 4.7 kg/m2). The report-described `dropfc=0.92` branch
instead reaches 11.505 kg/m2, directly resolving the management discrepancy in
favor of the delivered 0.95 branch for the reported live-stock plot.

Santee does not reproduce the report values on the report's 40-year basis:
years-31--40 mean live biomass is 18.104 versus 15 kg/m2 and total flat
residue is 3.473 versus 2.5 kg/m2. Annual runoff,
sediment, and daily-runoff return levels also materially differ from Bill's
missing Windows project. Peak-runoff-rate return levels remain
reconstructed from `.element.dat`; most report return levels are contradicted,
although a small subset of Santee recurrence points pass the frozen tolerance.
Hubbard 0.92 and Santee mixed also fail the frozen practical-equilibrium rule
(years-91--100 live-stock range exceeds 2% of the mean).

The execution is a successful bounded mukey-derived reconstruction, but the
scientific verdict is `NOT_REPRODUCIBLE` because load-bearing results
contradict the report. This is close enough to resolve the campaign question:
do not pursue Bill's missing byte-identical project and do not treat his
outputs as independent scientific authority.

The follow-on Linux 260725/source-native-9002 lane also completes all five
arms. It reproduces Hubbard 0.95 live biomass (18.886 kg/m2) and mean annual
runoff (12.077 versus 11.8 mm/year), but Hubbard old residue and most other
load-bearing targets remain contradicted. Santee's years-31--40 live biomass
is 19.886 versus 15 kg/m2. The additional lane therefore strengthens rather
than changes the `NOT_REPRODUCIBLE` verdict.

Pinned perennial senescence code and rounded daily producer output now
reconstruct gross annual aboveground live-to-current-residue transfer within
the `0.001 kg/m2` publication precision. The operator accepted that bounded
precision, lifting the package's prior sole hold without a direct or exact
internal-flux claim.
