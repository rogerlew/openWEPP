set pagination off
set debuginfod enabled off
set $begin_count=0
set $take_count=0
break openwepp_land_surface_energy::solver::begin_covered_component_dependency_replay_audit
commands
  silent
  set $begin_count=$begin_count+1
  printf "BEGIN_HIT=%d RDI=%p\n", $begin_count, $rdi
  continue
end
break openwepp_land_surface_energy::solver::take_covered_component_dependency_replay_audit
commands
  silent
  set $take_count=$take_count+1
  printf "TAKE_HIT=%d RDI=%p\n", $take_count, $rdi
  bt 3
  continue
end
break openwepp_runner::hillslope::tests::component_replay_audit::aggregate_component_replay_audit
commands
  silent
  printf "AGGREGATE_HIT BEGIN=%d TAKE=%d RDI=%p RSI=%p RDX=%p\n", $begin_count, $take_count, $rdi, $rsi, $rdx
  continue
end
run --ignored --exact hillslope::tests::stage3_laned_release_one_ofe_positive_baseline_profile --nocapture --test-threads=1
