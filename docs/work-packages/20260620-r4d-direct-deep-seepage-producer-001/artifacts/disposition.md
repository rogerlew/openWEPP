# R4D Disposition

Status: complete.
Evidence mode: Ran.

Verdict: `COMPLETE-R4D-DIRECT-DEEP-SEEPAGE-PRODUCER`.

Disposition:

- R4D direct deep-seepage producer implemented and tested.
- R4B consumes R4D-produced `deep_seepage_m` and fails closed if R4D did not
  run.
- No scheduler, output schema, publication, compatibility runtime, dependency,
  or default-activation change was made.
- Full Rust gates, markdown lint, `git diff --check`, no-compatibility proof,
  and H2637 default-disabled regression gate passed.
- No accepted blocking review finding remains.

Next recommended route: scaffold R4E for the direct `subsurface_loss_m` / `Qd`
producer under `SC-SUBHYD-001`, keeping the same no-publication/no-default/
no-scheduler boundaries until a later explicit publication package.
