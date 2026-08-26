# Worker handoff

Status: complete. No implementation handoff remains.

Defect `VEG-V9-CRYPTO-001` is closed at implementation candidate `8f4a9b84c`.
Future V9 checks use the content-bound package verifier. Exact-host remains the
only authority-generation route; provider equivalence verifies existing frozen
bytes only. A future runtime change that creates any second descriptor mismatch
or fails provider capability/output proof is a typed verification failure, not
authority to weaken this guard or rebind V9 bytes.
