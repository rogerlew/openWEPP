# Terminal V6 compiler-index provenance

Ran from the pinned Nix environment:

- `rustc 1.95.0 (59807616e 2026-04-14)`, host
  `x86_64-unknown-linux-gnu`, LLVM `21.1.8`;
- rustc wrapper SHA-256:
  `607f7e69e8dd21f730cfda4addef54f4c9e43ed4da8372e4ce6768c0bd5f6bf4`;
- rustdoc wrapper SHA-256:
  `fc9197c7c6fb5b00caa0b67149154eba690d2387be681f3e5eb47b7132af6249`;
- selected private rustdoc-page index aggregate SHA-256:
  `5105e10d1a367dd03dba550b31a6fec8540d40431cf964aca55ef94a7f935683`.

Capability finding: pinned stable rustdoc rejects JSON output as nightly-only;
rust-analyzer is absent from the pinned shell. Private-item HTML is generated
by the pinned rustdoc compiler and is the V6 compiler index. Its actual paths
correct V5's filename-derived carrier path.
