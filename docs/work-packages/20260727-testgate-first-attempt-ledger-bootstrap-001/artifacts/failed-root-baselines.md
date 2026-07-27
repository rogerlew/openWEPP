# Failed-Root Evidence Baselines

Status: `IMMUTABLE INPUT`

Evidence class: `Ran`

These SHA-256 values pin every retained evidence file in the two failed roots
before implementation. Closeout must recompute and match every value.

## Original first-attempt root

Root: `/home/workdir/gate-auth11-test-provider-canonical-001`

| Relative path | SHA-256 |
|---|---|
| `attempt-index.json` | `88a921523539717e4f0c3fcd273d2133922b17459423e7df2a1c0bbe7fb7317b` |
| `attempts.jsonl` | `a221c15800c2b956638d9b9c7c553860833cd74f60ccb8ab525cbc2053be60ba` |
| `authorized-paths.json` | `a58e4af3581247602be3ceb66762511b029e5b9519325bbf76c733eb8808d55c` |
| `intent-authorization.json` | `c26b643d3ce6222cbaec72047c45b0cc0d7337430ba981ba2d304c321b3199a1` |
| `intent-plan.json` | `c22e8193f6bea92270b1ca200714079fca756622df6242025d75ead95c14f7d2` |
| `observation.json` | `1334d911b6a4c1ef934ea12deb3f4b6dc22b569a204be97dc8b5f6e86f073c32` |
| `package-authority-chain.json` | `c26b643d3ce6222cbaec72047c45b0cc0d7337430ba981ba2d304c321b3199a1` |
| `terminal-plan.json` | `1683cccec6abf70696b474e8fa550cb244a153939474cafcbb5d7e1a39aa0aa5` |

The indexed evidence surface above contains no symlinks. The unindexed
`execution/.work/reconstruction/cargo-target/**` tree is a disposable build
cache that does contain build-generated symlinks; it is excluded from the
evidence baseline and must not be cited or admitted as retained evidence.

## Invalid wrong-campaign root

Root: `/home/workdir/gate-auth11-test-provider-canonical-001-retry-2`

| Relative path | SHA-256 |
|---|---|
| `authorized-paths.json` | `a58e4af3581247602be3ceb66762511b029e5b9519325bbf76c733eb8808d55c` |
| `intent-authorization.json` | `c26b643d3ce6222cbaec72047c45b0cc0d7337430ba981ba2d304c321b3199a1` |
| `observation.json` | `82cf4d53049b28b5ff00c68175862c2388d21bcd2be0c46c7e83231211539be6` |
| `package-authority-chain.json` | `c26b643d3ce6222cbaec72047c45b0cc0d7337430ba981ba2d304c321b3199a1` |

The retained evidence surface contains no symlinks and no execution evidence.
It remains invalid and must never be admitted as a retry.
