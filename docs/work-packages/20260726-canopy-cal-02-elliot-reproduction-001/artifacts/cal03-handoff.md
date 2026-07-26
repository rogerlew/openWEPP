# CAL-03 Handoff

Evidence class: `Ran`

CAL-03 may use the following bounded findings:

- the delivered Hubbard `dropfc=0.95` branch reaches 18.871 kg/m2 equilibrium
  live biomass and is the branch consistent with Bill's approximately
  19 kg/m2 plot;
- the report-described `dropfc=0.92` branch reaches only 11.505 kg/m2 under
  the paired reconstruction;
- Hubbard old/total residue and all load-bearing Santee stock targets are
  contradicted under the reconstructed soils;
- annual hydrology, sediment, and daily-runoff return levels do not reproduce
  Bill's missing Windows-project rows; and
- daily and peak-rate return levels are mixed but mostly contradicted; peak
  rate is reconstructed from the retained `.element.dat` `PeakRO` field.

These are sensitivity and provenance constraints, not independent calibration
authority. CAL-03 must retain the `NOT_REPRODUCIBLE` verdict and bounded-soil label and must not treat
WEPP 2012 agreement or Bill-derived values as proof of native CP2 correctness.
The first actionable item is to test native parameters against independently
admissible field targets while carrying the soil-representation uncertainty.

The Linux 260725/source-native-9002 lane additionally supplies gross annual
litter-transfer results. CAL-03 may use these as legacy mechanism
characterization, not field-validation authority: Hubbard 0.95 averages
0.82563 kg/m2/year over 100 years and 0.99403 over years 91--100; Santee mixed
averages 1.42949 and 1.63683 kg/m2/year over the same windows.
