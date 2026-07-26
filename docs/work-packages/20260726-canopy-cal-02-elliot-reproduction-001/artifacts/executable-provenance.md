# Executable Provenance

Evidence class: `Live read-only host inspection`

## WEPP Windows

| Field | Value |
| --- | --- |
| Host | `BLARHG` |
| Operating system at inspection | Microsoft Windows 10 `10.0.26200.8894` |
| Executable | `C:\WEPP\wepp\wepp_2012.exe` |
| Size | `1,597,440` bytes |
| Modified time reported by host | `2012-09-18 12:57 PM` |
| SHA-256 | `6104a3440624ad54aa6c3660794280adfd600d4a11b98559c6205a73cd47fc3f` |
| File-version metadata | blank |

The executable identity is exact and accessible for an isolated CAL-02 run.
It does not identify the runtime libraries or operating-system state used for
Bill's April 2026 analysis.

## WEPPcloud source

The retained run controls encode 100 years. Both exact climate headers identify
CLIGEN `5.32300`, seed `12345`; Hubbard uses `nh275639.par` and Santee uses
`sc381544.par`.

The April 2026 WEPPcloud executable/data commit is not yet identified. Retained
selected outputs permit byte-level source-output checks regardless.
