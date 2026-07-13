# Restart Conservation And Consumer Evidence

Status: `POST-FIX-EVIDENCE-WITH-H2637-GROUNDWATER-LIMITATION`

Evidence class: **Ran + Static** at frozen commit
`40db90ae57d99d70877c4cf616de614b3fb242eb`.

## Result

Independent reads of frozen-source HBP, WAT, pass-Parquet, and watershed
Parquet outputs close the p61 sediment, p102 hillslope sediment, p102 routed
publication, selected snow/frost, and W11B interval water/sediment claims.
The restarted production tests that generated or consumed those surfaces
passed. Serial and four-worker p102 publication is identical in ordered
values, null posture, schemas, field metadata, and schema metadata across all
14 Parquet products.

One limitation remains closure-significant: H2637 publishes cumulative
groundwater recharge, generated baseflow, and deep seepage in its manifest,
but neither its HBP nor pass Parquet publishes the terminal groundwater
reservoir storage needed to observe the recurrence independently. The
inferred terminal storage change is `120.97394672682913 m3`. This evidence
therefore does not independently close the complete H2637 groundwater
storage recurrence. The actual H2637 HBP was independently decoded, while
the downstream watershed baseflow-once binding was exercised separately.

## Operand Lineage

| Claim | Operands and basis | Real serialized reader / consumer | Rejected alias |
| --- | --- | --- | --- |
| H2637 active routing | `source`, routed outlet, end-window storage, and clamp, all `m3` over 731 hillslope days | run manifest plus independent arithmetic; HBP decoded without the production parser | precipitation magnitude, pass `runvol`, shadow state |
| H2637 groundwater | recharge, generated baseflow, deep seepage, and reservoir storage change, all `m3` | manifest totals; HBP schema 1.1 carries event baseflow/deep seepage; watershed intake maps them to `HillslopeContribution` | `latqcc`, `cbase`, dependency baseflow added as a new local source |
| p61 sediment | `tdet`, `tdep` in `kg`; `runvol` in `m3`; five `sedcon` values in `kg/m3` | independent PyArrow pass reader and independent HBP binary decoder | unit width, an adjacent concentration, producer aggregation helper |
| p102 hillslope sediment | HBP `sum(S_h)`, chain `tdet-tdep` in `kg`; HBP `sum(V_h)` in `m3` | independent HBP decoder; production watershed pass inventory parser | outlet-only detachment as chain total, zero fill |
| p102 watershed sediment | public `tdet`, `tdep`, `sed_del`, and EBE yield in `kg` | totalwatsed3 and EBE Parquet readers | raw `tdet-tdep` as routed yield |
| Snow | `P`, `RM`, and `Snow-Water`, all `mm` over one OFE | production WAT Parquet | physical `Snow-Depth`, static snow control, diagnostic snowbench output |
| Frost | liquid `Total-Soil`, `frozwt`, fluxes, all `mm` over one OFE | production WAT Parquet on dry, unchanged-snow days | frozen depth `frdp`, treating freeze/thaw transfer as external water |
| W11B water | external HBP hourly volume, terminal EBE outflow, terminal channel storage, all `m3` | watershed CLI EBE and channel-water-balance Parquet | scalar event peak/duration, upstream flow re-added as local flow |
| W11B sediment | HBP hourly/class mass and terminal EBE yield in `kg` | watershed CLI plus direct same-grid per-interval/class consumer test | daily scalar sediment, concentration without water volume |

Fixture-tree hashes are in
`logs/restart-reconstruction-fixture-tree-hashes.log`. The hashes cover H2637,
p61, watershed p102, and the p313 snow/frost fixture.

## Independent Arithmetic

### H2637 hydrology and groundwater

**Ran:** the active-owner test passed at the frozen commit. Independent total
arithmetic from the generated manifest gives:

```text
374420.2511558311 source
- 371254.38460113516 routed outlet
-   3165.866554692616 end-window storage
-      4.9829504321846435e-14 clamp
=      3.322334316072185e-9 m3 residual
```

The relative residual is `8.873276233900748e-15`. The generated manifest,
HBP, and pass hashes are respectively
`5cc0023639736df405c1765009b3a429712be3a42c310f88a6341870947e12f2`,
`378a8c1d80a22c9452fb256cf9a95eab09035f3a6cd387c6d626ab26c426c453`,
and `915f3b99c2ff20e3e0632b4e90a6ceb1cb8e7fee58f0d3e29b41de10c540f550`.

The independent schema-1.1 decoder found event baseflow
`5.032033091000001 m3` and deep seepage `0 m3` in the serialized HBP. The
manifest totals are recharge `3668.610172576748 m3`, baseflow
`3547.636225849919 m3`, and deep seepage `0 m3`. Their difference is the
unpublished `120.97394672682913 m3` storage change, not an independently
observed storage operand.

### p61 and p102 sediment

**Ran:** p61 has one detaching row. Independently reading the pass Parquet:

```text
sum(sedcon_1..5) = 3.5549094565627795 kg/m3
runvol             = 794.04731794515601 m3
reconstructed mass = 2822.7663195215473 kg
tdet                = 2822.7663195215482 kg
residual            = 9.0949470177292824e-13 kg
```

The independent HBP decoder separately reports hourly sediment
`2822.766319521547 kg`, hourly runoff `794.0473179451559 m3`, class-fraction
sum `1.0`, and minimum class fraction `0.026647828384138175`.

**Ran:** the p102 HBP decoder reports `584.2332653870001 kg` detachment,
`282.14618621700004 kg` deposition, and hourly sediment
`302.08707916989255 kg`, closing to the chain export within
`1.08e-10 kg`. Its five fractions sum to `1.0` and the minimum is
`0.03380000000000001`. The production watershed output routes that load to
`302.13161177539132 kg`; this equals EBE sediment yield exactly and differs
from raw hillslope export by `0.044532605391282232 kg`, proving `sed_del` is
not the raw-export alias.

### Snow and frost

**Ran:** two focused production-runner snow tests passed. The generated WAT
at the frozen commit closes accumulation and release:

```text
day 1: 0 + 4.4 - 8.662176878559968e-10 - 4.399999999133782
       = 8.881784197001252e-16 mm
day 2: 4.399999999133782 + 0 - 4.399999999133782 - 0
       = 0 mm
```

The WAT hash is
`f8569f2bacc747e6954b8291da109e71d3448afebdc42f3c9979457e97bc61c6`.
The same rows publish physical snow depths `40.630422948035005 mm` and
`0 mm`, distinct from SWE.

**Ran:** a frozen-source p313 production WAT supplies material freeze and
thaw rows. On dry days with unchanged snow, independent combined liquid plus
frozen storage reconstruction gives `5.684341886080802e-14 mm` residual on a
freeze-growth day and exactly `0 mm` on year 4, Julian 290. On the thaw day,
frozen storage falls `2.2209619173766457 mm`, liquid storage rises
`1.068583684363432 mm`, and ET/percolation/baseflow removes
`1.152378233013205 mm`; the combined ledger closes exactly. The WAT hash is
`6e8791348a1daf6bddcf8e6f0facdf8341fbc9efcd73ceb57bcec09e5a13ea30`.

### Watershed p102 serial/parallel and W11B

**Ran:** all 14 serial and `--jobs 4` Parquet products have equal ordered
tables and full semantic schemas, including schema and field metadata. File
bytes differ because the serialized `ARROW:schema` metadata encodes map-key
order differently; decoded metadata maps are equal. HBP and pass bytes are
identical between modes. This distinction is recorded rather than claiming
false byte identity.

**Ran:** the two-channel spike and spread fixtures each inject `7200 m3` and
`240 kg`. Independent serialized-output reconstruction gives:

| Shape | Terminal outflow (`m3`) | Storage (`m3`) | Residual (`m3`) | Yield (`kg`) | Peak (`m3/s`) |
| --- | ---: | ---: | ---: | ---: | ---: |
| Spike | 7089.7398318200121 | 110.26016817998794 | 0 | 240 | 1.9999938168811557 |
| Spread | 7161.017078834353 | 38.982921165647085 | 0 | 240 | 0.5 |

The different peaks reject scalar-total replay. The focused direct-consumer
rerun also passed the same-grid class-egress and external-baseflow-once tests:
the downstream interval inlet equals upstream `q1`/class egress, downstream
local lateral baseflow is zero, and `864 m3` external baseflow closes against
terminal outflow plus both channels' storage.

## Real Consumer And Negative-Path Proof

**Static:** the runner validates each generated HBP with the production parser
at `watershed_supervisor.rs:506-548`. The watershed CLI maps HBP baseflow,
deep seepage, sediment classes, and hourly arrays into typed
`HillslopeContribution` fields at
`openwepp-cli-watershed.rs:514-557`. The interval kernel reads contributor
hourly runoff, dependency `q1`, and local generated baseflow at
`hourly.rs:219-279` and `hourly.rs:1140-1187`.

**Static + Ran:** when the interval lane is active, the channel kernel returns
the interval result at `direct.rs:174-180`, before scalar/event routing. The
same-grid and baseflow-once tests passed in
`logs/restart-reconstruction-w11b-direct-consumers.log`. Generated hillslope
manifests selected `direct-production-executor`, report zero compatibility
edge invocations, and H2637 additionally reports zero skeleton runs and no
compatibility rollback. The p102 routed/raw mass difference and W11B timing
sensitivity provide output-level negative proofs independent of those
counters.

## Evidence Files And Limitations

- `logs/restart-reconstruction-hbp-reader.log`: independent binary HBP reads.
- `logs/restart-reconstruction-h2637-ledger.log`: H2637 totals and hashes.
- `logs/restart-reconstruction-serialized-arithmetic.log`: p61, p102, W11B,
  serial/parallel arithmetic and hashes.
- `logs/restart-reconstruction-serial-parallel-semantic-metadata.log`:
  decoded schema/metadata equality.
- `logs/restart-reconstruction-snow-wat.log` and
  `logs/restart-reconstruction-frost-wat.log`: selected production WAT
  reconstructions.
- `logs/restart-reconstruction-snow-production-test.log` and
  `logs/restart-reconstruction-w11b-direct-consumers.log`: focused reruns.

The independent HBP decoder intentionally reads only schema 1.1 payload
fields needed for this audit; the production parser remains the format and
guard authority. Temporary run directories are not durable package assets,
so hashes, exact operands, and compact logs are the retained evidence. Most
importantly, the missing H2637 terminal groundwater storage operand prevents
this artifact alone from satisfying the campaign's complete independent
groundwater recurrence requirement.
