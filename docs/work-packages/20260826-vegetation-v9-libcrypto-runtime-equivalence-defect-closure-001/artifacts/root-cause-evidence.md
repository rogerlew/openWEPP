# Root-cause evidence

Status: reproduced.

Static: V9 committed on 2026-08-17 binds `libcrypto.so.3` SHA-256
`0cd331307536a397ab9c83c6dbeeb3474d0a5114f397ce03d1762adb96d3c781`.
The exact bytes are Ubuntu `libssl3t64 3.5.5-1ubuntu3.3`. Host apt logs show
`.3` installed on 2026-08-14 and `.4` installed by unattended upgrade on
2026-08-26. Current bytes hash to
`23265e4027cb6439687be04311a0f37e27f29a23bfa4c750c49725da14f986bb`.

Ran: with only the stale libcrypto checksum result substituted diagnostically,
the current `.4` provider produced output SHA-256
`f86770cce11235ba282b47e81de2fa5dc9af19c29dc3bd91c62256957c590633`,
byte-identical to the frozen V9 vector; imported V8 execution remained
`e9a53d6d15cb04136683c24d5b7ebd4437eb26569cf984b5cecfb5e47d73b416`.

Disposition: owned, reproducible over-binding in active authority verification;
not a vegetation numerical or production defect.
