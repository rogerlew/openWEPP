# Reference Acquisition Ledger

Status: `complete / load-bearing set captured`

Evidence mode: `Static + Ran`

All successful acquisitions are local exact bytes reviewed only after their
bibliography entry was updated. Access date is 2026-08-11.

| Source | Route | Result | Exact identity / authority use |
|---|---|---|---|
| Farquhar et al. (1980) | DOI discovery; Colorado State SiB mirror | acquired, 13-page PDF | SHA-256 `ce15f7a78456bf8a9153b20204a6a0d51c3e2697c3a03105315634bf1fe05048`; primary FvCB equations |
| de Pury and Farquhar (1997) | DOI discovery; Colorado State SiB mirror | acquired, 21-page PDF | SHA-256 `8a847133cf3d546bccd3e2dc076fa3b1e5e6f71edf2dd2efcc32282f3fc41fc6`; direct/diffuse sunlit/shaded equations |
| Medlyn et al. (2011) | DOI; UTS author manuscript | acquired, 39-page PDF | SHA-256 `57f9754dac8f81f257d819d474f6ed250b801179ceebadbe88c3f9c56cf17623`; primary stomatal equations |
| CLM5.0 Technical Description | official CESM/NCAR file service | acquired, 337-page PDF | SHA-256 `9ca0f0e5b7aff712a0ef7f5198f111c4b250cac4417a4f000e36c6c143f2e363`; selected established-model definition |
| Biome-BGC 4.2 Theoretical Framework | official University of Montana model site | acquired, 71-page PDF | SHA-256 `476dd8d5606941ccfdd59de277d03671e764ac6ceac44d9bebd68bf61f00be85`; corroborating reference-model definition |
| White et al. (2000) | DOI/AMS PDF and full-text endpoints; author/full-text discovery; local cache search | not reacquired: AMS returned HTTP 403 and no stable author/institutional PDF was found | not used as binding authority; CLM5 and Biome-BGC technical definitions independently close executable C/N family, while White remains parameter-family discovery only |
| Wullschleger (1993) | OUP PDF (HTTP 403); CiteSeerX result (certificate failure then 404); DOI/institutional/government/local cache search | not reacquired | not used as binding authority or selected capacity relationship; caller supplies `Vcmax25/Jmax25`, and FvCB/CLM5 define equations/domains |
| Kennedy et al. (2019) | DOI and Wiley open PDF endpoint | HTTP 403 | official CLM5 technical note Chapter 11 independently supplies the exact selected plant-hydraulic reference-model definition; article remains corroborating discovery |

## Reused Exact Corpus

Ran: exact local SHA-256 was recomputed for Gash (1979), Shuttleworth-Wallace
(1985), Best/JULES (2011), Forrester et al. (2014), Bonan et al. (2014), and
the new files above. Values match the canonical bibliography for all prior
files. Jarvis, Stewart, Kelliher, and Pereira remain verified local comparator
or rejected-alternative evidence and retain their bibliography hashes.

## Attempts And Sufficiency

The White and Wullschleger failures do not block admission because neither is
selected as binding equation/value authority. Their exact blocked roles are
closed by independently sufficient sources: official CLM5/BIOME-BGC technical
definitions for persistent C/N transactions and caller-owned capacity fields,
and Farquhar plus CLM5 for biochemical equations and temperature/numerical
branches. No inaccessible equation is being paraphrased into authority.
