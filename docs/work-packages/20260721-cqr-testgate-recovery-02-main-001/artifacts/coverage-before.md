# Coverage Before

Ran: the exact pre-production baseline was measured once at clean scaffold HEAD
`62cb1086a1abbfb1f6b9993db101fc8bf0240882`. The selected binary traversal
passed 5/5 tests in 20.65 seconds; matching CRAP processing exited zero in 0.15
seconds. No unchanged traversal was rerun.

The production boundary is lines 1-882 (`#[cfg(test)]` begins at line 883).
Production-only coverage was 219/702 lines (31.20%) and 333/1,308 regions
(25.46%). The source SHA-256 was
`2c346898c288c23f5593bf920cd1eaac7274db495bf76a4a610aa3508a9734b4`.

Evidence root: `/tmp/cqr-main-baseline-MVGA48`.

| Artifact | SHA-256 |
| --- | --- |
| LCOV | `8b8dd49750861b5dfee8600f6aed9e0ca4261d28070412db1813ee17dbf563b5` |
| CRAP JSON | `92d6be13ddbf7ac2bcdb27553c48e6df5541504612fc661ed6dd35db6a8f4449` |
| coverage JSON | `53a72514923d55504be8ed372d66f9a26ed60b32c3538d030ebebbf034f4c0ce` |
| llvm-cov log | `a09f95d28aa86e46ec411fd421a1802ad19904c9bcb81359d8ddd0e288f3a84a` |
| CRAP log | `94f6263df2fb485d3334f4ba8333c6c0df5ad7b9fdfc2549d3dd0810b76ebe8e` |
