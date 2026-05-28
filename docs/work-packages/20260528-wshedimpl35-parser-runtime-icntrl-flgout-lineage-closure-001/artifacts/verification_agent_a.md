# WSHEDIMPL35 Verification Agent A

Status: complete  
Evidence mode: ran  
Date: 2026-05-28

## Ran
- `cargo test --test infile_watershed_channel_parser_contract strict_mode_rejects_icntrl_out_of_domain` -> pass
- `cargo test --test infile_watershed_channel_parser_contract strict_mode_rejects_flgout_out_of_domain` -> pass
- `cargo test -p openwepp-watershed-orchestrator watershed_channel_runtime_seed_rejects_out_of_domain_icntrl` -> pass
- `cargo test -p openwepp-watershed-orchestrator watershed_channel_runtime_seed_rejects_out_of_domain_flgout` -> pass
