# Discrete complete-owner terminal support-root defect hold

Status: `EXECUTED / HOLD / CHILD1-DISCRETE-SUPPORT-ROOT-001`.

Evidence base: exact clean intake
`221e94ef3e6ccf646f732bf104b0fb563208d338`. Production remains
`BelowCarrierDomain`; `43cc9bbea2fbf5fe6ab6596cee4162de75cef999`
remains the last fully qualified physical implementation.

## Disposition

Ran: the test-only endpoint map executes the actual one-lane complete-owner
covered carrier from immutable beginning state, applies the unchanged terminal
enthalpy transition once over the exact integer-nanosecond support, binds the
selected hydrology ending joint back to the matching typed six non-snow owners,
and retains all seven real ending owners, the snow--soil receipt, closure
residuals, and canonical replay bytes. It does not construct derivatives,
interpolate owner state, or invoke the continuous DAE/LTE search.

Ran: the current production call still returns typed `BelowCarrierDomain`.
The existing 937,500,000-ns fine-1 candidate is reproducibly `PreTerminal`, not
`TerminalAtEndpoint`: ice bits `0x3fe322f330bff947`, liquid bits
`0x3f6048fffae62796`, cold-content bits `0x0000000000000000`, and
terminal-unallocated-energy bits `0x0000000000000000`.

Ran: classification retains raw energy bytes and uses the canonical
`1.0e-6 J m^-2` comparison tolerance. Two valid exact-tick brackets over the
same immutable beginning owners both select `615737728343` ns as the first
materially invalid endpoint. At that tick:

- `event_occurred = false` and end ice remains positive;
- complete melt is `0x3fe3333333333333` and ending liquid has the same bits;
- ending ice and positive deposition are both `0x3f6710bec24da293`;
- terminal unallocated energy is `0x3eb106e000000000`, above `1.0e-6 J m^-2`;
- mass, water and energy closures remain within their unchanged tolerances;
- exact endpoint replay is byte-identical;
- tick `615737728342` is `PreTerminal`, while `615737728344` is `Invalid`.

The batch-shaped typed root API returns `InvalidEndpoint`, not a root, for the
real one-lane callback. The endpoint map therefore reaches complete melt plus
material excess energy while positive deposited ice survives and no actual
terminal event exists. No admissible `TerminalAtEndpoint` root exists in the
real parent support. This is the explicit owner-defined no-admissible-root stop
condition, so the checkpoint terminates as a defect-shaped `HOLD`.

The real evidence is deliberately one-lane because the first fixture stops the
exploration. It does not claim a successful multi-lane Batch V2 installation.
Synthetic complete-owner tests establish only the generic joint callback and
failure semantics. No v22/v12/v140/v7 candidate is authored. No production
temporal operator, Batch V2, receiver, restart, runner, owner publication,
Child 3/4, Stage-3 activation, CoE, public API/output, constitutive equation,
constant, or 600-ms floor changes.

## Exact evaluated tick/class record

Ran: the real fixture emitted the following calls in execution order. Repeated
ticks are deliberate bracket, typed-root, replay and neighborhood evidence.

```text
600000000 PreTerminal
900000000000 Invalid
450300000000 PreTerminal
675150000000 Invalid
562725000000 PreTerminal
618937500000 Invalid
590831250000 PreTerminal
604884375000 PreTerminal
611910937500 PreTerminal
615424218750 PreTerminal
617180859375 Invalid
616302539062 Invalid
615863378906 Invalid
615643798828 PreTerminal
615753588867 Invalid
615698693847 PreTerminal
615726141357 PreTerminal
615739865112 Invalid
615733003234 PreTerminal
615736434173 PreTerminal
615738149642 Invalid
615737291907 PreTerminal
615737720774 PreTerminal
615737935208 Invalid
615737827991 Invalid
615737774382 Invalid
615737747578 Invalid
615737734176 Invalid
615737727475 PreTerminal
615737730825 Invalid
615737729150 Invalid
615737728312 PreTerminal
615737728731 Invalid
615737728521 Invalid
615737728416 Invalid
615737728364 Invalid
615737728338 PreTerminal
615737728351 Invalid
615737728344 Invalid
615737728341 PreTerminal
615737728342 PreTerminal
615737728343 Invalid
937500000 PreTerminal
1799999999999 Invalid
900468749999 Invalid
450703124999 PreTerminal
675585937499 Invalid
563144531249 PreTerminal
619365234374 Invalid
591254882811 PreTerminal
605310058592 PreTerminal
612337646483 PreTerminal
615851440428 Invalid
614094543455 PreTerminal
614972991941 PreTerminal
615412216184 PreTerminal
615631828306 PreTerminal
615741634367 Invalid
615686731336 PreTerminal
615714182851 PreTerminal
615727908609 PreTerminal
615734771488 PreTerminal
615738202927 Invalid
615736487207 PreTerminal
615737345067 PreTerminal
615737773997 Invalid
615737559532 PreTerminal
615737666764 PreTerminal
615737720380 PreTerminal
615737747188 Invalid
615737733784 Invalid
615737727082 PreTerminal
615737730433 Invalid
615737728757 Invalid
615737727919 PreTerminal
615737728338 PreTerminal
615737728547 Invalid
615737728442 Invalid
615737728390 Invalid
615737728364 Invalid
615737728351 Invalid
615737728344 Invalid
615737728341 PreTerminal
615737728342 PreTerminal
615737728343 Invalid
600000000 PreTerminal
900000000000 Invalid
615737728342 PreTerminal
615737728343 Invalid
615737728344 Invalid
615737728343 Invalid
```

## Claim boundary

This is a discrete endpoint model and makes no continuous-time or LTE claim.
All constitutive physics are inherited unchanged. Terminal time would be the
earliest admissible exact tick of the complete endpoint map, but the real map
supplies a bracket-independent typed invalid candidate instead of any
`TerminalAtEndpoint`. Accordingly there is no successor-contract,
multi-lane-installation, or production-installation claim.
