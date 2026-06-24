# Output Parity

Evidence class: Ran.

Protected outputs:

- HBP
- WAT
- PASS
- loss
- plot
- manifest metadata/checksum map

Current-code comparisons:

| Output | Comparison | Result | Notes |
| --- | --- | --- | --- |
| HBP | direct default vs explicit direct | pass | checksum `e28dd29a68290221a8d3c7fddd36e9c55a19d5cfcee450aef08fdfa36ae8ac84` |
| WAT | direct default vs explicit direct | pass | checksum `ddee3edb2cbe22964ff57d6b4c3a723fc477f569ea59774045c7a6c927b5acfb` |
| PASS | direct default vs explicit direct | pass | checksum `0110ea7d0cf5a7a9f69627b322834f6a7189e8f278de8894b9faef8a13b0a85e` |
| loss | direct default vs explicit direct | pass | checksum `220ccc1032c1934b4d4f7b7a0c0578e6e0f02d2a9c799ab56049402982fd72b8` |
| plot | direct default vs explicit direct | pass | checksum `55a51e71738c9eb039ce1aff8a2cfb64e14d7d7fc1d8e3c902ceba3e5bcf8bff` |
| HBP | default compatibility vs explicit rollback | pass | checksum `266601331e13c095bb2e0ddea8ff03b4994df517a829647f6c66e5cfa0e22ed8` |
| WAT | default compatibility vs explicit rollback | pass | checksum `83053b5b3afee9ed245708b3cc5f5e666c1050925630e200e5b688308628cec0`; DuckDB `EXCEPT ALL` `0/0` |
| PASS | default compatibility vs explicit rollback | pass | checksum `b4acd0ae8524a48ee4e51cf58b8b8bf9cb40b63a129823e8ead45ae98e0afcfb` |
| loss | default compatibility vs explicit rollback | pass | checksum `220ccc1032c1934b4d4f7b7a0c0578e6e0f02d2a9c799ab56049402982fd72b8` |
| plot | default compatibility vs explicit rollback | pass | checksum `55a51e71738c9eb039ce1aff8a2cfb64e14d7d7fc1d8e3c902ceba3e5bcf8bff` |
| HBP | explicit direct vs default compatibility | fail | direct `e28dd29a68290221a8d3c7fddd36e9c55a19d5cfcee450aef08fdfa36ae8ac84`; compatibility `266601331e13c095bb2e0ddea8ff03b4994df517a829647f6c66e5cfa0e22ed8` |
| WAT | explicit direct vs default compatibility | fail | direct `ddee3edb2cbe22964ff57d6b4c3a723fc477f569ea59774045c7a6c927b5acfb`; compatibility `83053b5b3afee9ed245708b3cc5f5e666c1050925630e200e5b688308628cec0`; DuckDB `EXCEPT ALL` `235907/235907` |
| PASS | explicit direct vs default compatibility | fail | direct `0110ea7d0cf5a7a9f69627b322834f6a7189e8f278de8894b9faef8a13b0a85e`; compatibility `b4acd0ae8524a48ee4e51cf58b8b8bf9cb40b63a129823e8ead45ae98e0afcfb`; DuckDB `EXCEPT ALL` `12415/12415` |
| loss | explicit direct vs default compatibility | pass | checksum `220ccc1032c1934b4d4f7b7a0c0578e6e0f02d2a9c799ab56049402982fd72b8` |
| plot | explicit direct vs default compatibility | pass | checksum `55a51e71738c9eb039ce1aff8a2cfb64e14d7d7fc1d8e3c902ceba3e5bcf8bff` |
| manifest | direct vs compatibility | fail by design | runtime provenance differs: direct uses `direct-publication-frame`; compatibility/rollback use `scheduler-kernel` |

Parity reductions:

- WAT field reduction for direct explicit vs default compatibility:

  | Field | Differing rows | Max abs |
  | --- | ---: | ---: |
  | `SoilWaterTotal` | `235872` | `140.6160639448699` |
  | `Total-Soil` | `235872` | `140.6160639448699` |
  | `Q` | `220389` | `34.34960264345201` |
  | `QOFE` | `220389` | `491.53978297874187` |
  | `UpStrmQ` | `208040` | `461.5473075939456` |
  | `latqcc` | `203901` | `26.007953025347657` |
  | `SubRIn` | `191497` | `26.007953025347657` |
  | `Es` | `108961` | `3.870672305294199` |
  | `Ep` | `104831` | `3.6256955983831154` |
  | `frdp` | `34603` | `260.42707938173714` |
  | `frozwt` | `34596` | `7.094744920925079` |
  | `Snow-Water` | `21305` | `183.04425009202413` |
  | `RM` | `14870` | `39.94882220799281` |
  | `Interception` | `12766` | `0.891459703930619` |
  | `Dp` | `278` | `47.18678591179722` |

- First material WAT divergence above `1e-9` occurs on Julian day 6. It is a
  frost split: direct retains more liquid soil water and has less `frozwt` /
  `frdp` than compatibility. Day 5 is parity-clean at the WAT frost fields.
- PASS field reduction for direct explicit vs default compatibility:

  | Field | Differing rows | Max abs |
  | --- | ---: | ---: |
  | `sbrunv` | `12404` | `233.43730392521928` |
  | `runvol` | `12352` | `5342.8373842872525` |
  | `peakro` | `7017` | `0.022577345286866557` |

- PASS sediment fields are clean in this reduction. The PASS residual follows
  hydrology/frost runoff state, not sediment publication.
