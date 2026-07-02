# D7-S0 — Cut-Point Map (like-for-like, fail-closed)

Evidence: **Ran** — `pandas` reads of the gitignored `Figure_4.xlsx`
(sha256 `2bf68787…d2fe8`, verified) + magnitude sanity against authoritative
operands (`3.1_Validation_Input.docx`, sha256 `0aee1455…f9362`). No series
vendored.

## The four D-val cases live in Figure_4 (by source experiment)

The paper's Figure 4 = the four validation cases (a-d). `Figure_4.xlsx`
carries them by **experiment name**, so case↔column mapping is:

| Case | Experiment | Enhanced_WEPP sheet | time col (unit) | q col (unit) | ref peak (m²/s) |
|---|---|---|---|---|---|
| 1 bare | Abban 2017 | `Enhanced_WEPP` | col 10 (min → s) | **col 11 (m²/s)** | 9.451e-5 |
| 2 isolated | Jomaa 2012 | `Enhanced_WEPP` | col 16 (min → s) | col 17 (m²/s) | 1.061e-4 |
| 3 vegetation | Neibling 1979 | `Enhanced_WEPP` | col 5 (min → s) | col 8 (m²/s) | 1.685e-4 |
| 4 shock | Iwagaki 1955 | `Enhanced_WEPP` | col 2 (s) | col 1 (m²/s) | 8.132e-3 |

`q` is `m³/s/m` = **m²/s** = openWEPP `outlet_unit_discharge_m2_s` — a **direct
unit match** (no width scaling). `Original_WEPP` sheet has header-confirmed
`Time (min)` / `q (m³/s/m)`; the `Enhanced_WEPP` sheet is header-less, so its
columns are pinned by dataset-name row + magnitude sanity below.

**Figure 5 is NOT a validation case** — its enhanced peak (~2.8e-3 m²/s at
t→29 h) is the Walnut Creek *hillslope* (paper §3.2), not the Case-1 7.5 m
plot. Figures 6-8 are the §3.3 thought-experiments. Only Figure 4 feeds D-val.

## Magnitude sanity (fail-closed gate)

Max plot unit discharge `q_max = I·L` (all rainfall as excess):

| Case | I·L (m²/s) | ref peak | disposition |
|---|---|---|---|
| 1 | 60 mm/h × 7.5 m = **1.25e-4** | col 11 = 9.45e-5 ✓ (RC 0.76) | **col 11 PASS** |
| 1 | — | col 12 = 1.80e-4 **> 1.25e-4** ✗ | **col 12 REJECTED** (unphysical as m²/s) |
| 2 | 74 mm/h × 6 m = 1.23e-4 | col 17 = 1.06e-4 ✓ | PASS |
| 3 | 74 mm/h × 6.1 m = 1.25e-4 | col 8 = 1.685e-4 **> 1.25e-4** ✗ | **CAVEAT** (see below) |
| 4 | impermeable, lateral pulse | col 1 = 8.13e-3 | PASS (no infiltration cap) |

**Case-1 column disambiguation (load-bearing):** the header-less enhanced
sheet has two candidate Abban q columns; col 12 (1.80e-4) exceeds the physical
max and is rejected, so **col 11 (9.45e-5, RC 0.76) is the enhanced hydrograph**.
This is a reasoned like-for-like determination (physical-consistency filter),
not a header confirmation — recorded as a residual S0 caveat.

**Case-3 anomaly:** col 8 peak (1.685e-4) also exceeds I·L (1.25e-4) for the
stated 6.1 m plot — a magnitude the plot cannot produce under Hortonian excess.
Either the digitized geometry differs or col 8 is a different quantity. Case 3
therefore enters S3 **cut-point-caveated**; its verdict is bounded accordingly.

## Disposition

S0 **passes for Cases 1, 2, 4** (like-for-like unit + geometry established,
Case 1 with a documented column caveat). Case 3 carries an **unresolved
magnitude anomaly** — its comparison is valid for *shape* but not certifiable
on *magnitude*. No NS was computed before this map closed.
