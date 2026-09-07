set pagination off
set debuginfod enabled off
break openwepp_runner::hillslope::tests::component_replay_audit::aggregate_component_replay_audit
commands
  silent
  printf "BREAK_AGGREGATE\n"
  printf "RDI=%p RSI=%p RDX=%p RCX=%p\n", $rdi, $rsi, $rdx, $rcx
  set $caller_sp=$rdi-0x80
  printf "CALLER_SP=%p SECOND_TAKE_STORAGE=%p\n", $caller_sp, $caller_sp+0x7c0
  x/16gx $caller_sp+0x7c0
  printf "RETURN_STORAGE_RDI\n"
  x/16gx $rdi
  printf "AUDIT_CANDIDATE_RSI\n"
  x/64gx $rsi
  printf "AUDIT_HEAP_CANDIDATES\n"
  set $p0=*(void**)($rsi+8)
  set $p1=*(void**)($rsi+32)
  set $p2=*(void**)($rsi+48)
  set $p3=*(void**)($rsi+64)
  printf "P0=%p P1=%p P2=%p P3=%p\n", $p0, $p1, $p2, $p3
  x/96gx $p0
  x/96gx $p1
  x/96gx $p2
  x/96gx $p3
  bt 4
  continue
end
run --ignored --exact hillslope::tests::stage3_laned_release_one_ofe_positive_baseline_profile --nocapture --test-threads=1
