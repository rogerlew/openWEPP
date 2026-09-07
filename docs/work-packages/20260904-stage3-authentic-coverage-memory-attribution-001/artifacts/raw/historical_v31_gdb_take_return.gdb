set pagination off
set debuginfod enabled off
set $take_count=0
break openwepp_land_surface_energy::solver::take_covered_component_dependency_replay_audit
commands
  silent
  set $take_count=$take_count+1
  set $take_output=$rdi
  set $take_return=*(void**)$rsp
  printf "TAKE_ENTRY=%d OUTPUT=%p RETURN=%p\n", $take_count, $take_output, $take_return
  tbreak *$take_return
  commands
    silent
    printf "TAKE_RETURN=%d OUTPUT=%p\n", $take_count, $take_output
    x/64gx $take_output
    set $sweep_ptr=*(void**)($take_output+8)
    set $internal_ptr=*(void**)($take_output+32)
    printf "TAKE_VECS SWEEP_PTR=%p SWEEP_LEN=%llu INTERNAL_PTR=%p INTERNAL_LEN=%llu\n", $sweep_ptr, *(unsigned long long*)($take_output+16), $internal_ptr, *(unsigned long long*)($take_output+40)
    if $take_count == 1
      set $sweep_i=0
      while $sweep_i < *(unsigned long long*)($take_output+16)
        set $sweep_addr=$sweep_ptr+($sweep_i*160)
        set $sweep_occ=*(unsigned long long*)($sweep_addr+112)
        set $sweep_soil=*(unsigned long long*)($sweep_addr+120)
        if $sweep_occ == 2 && $sweep_soil == 6
          printf "N2S6_SWEEP i=%llu solve=%u map=%llu iter=%u sweep=%u logical=%u identity=%u component=%u complete=%u collecting=%u completed=%u failed=%u\n", $sweep_i, *(unsigned char*)($sweep_addr+153), *(unsigned long long*)($sweep_addr+104), *(unsigned int*)($sweep_addr+128), *(unsigned int*)($sweep_addr+132), *(unsigned int*)($sweep_addr+136), *(unsigned int*)($sweep_addr+140), *(unsigned int*)($sweep_addr+144), *(unsigned int*)($sweep_addr+148), *(unsigned char*)($sweep_addr+152), *(unsigned char*)($sweep_addr+154), *(unsigned char*)($sweep_addr+155)
          if $sweep_i == 0 || $sweep_i == 2
            set $stencil_ptr=*(void**)($sweep_addr+32)
            printf "N2S6_STENCILS i=%llu PTR=%p LEN=%llu\n", $sweep_i, $stencil_ptr, *(unsigned long long*)($sweep_addr+40)
            x/40ub $stencil_ptr
          end
        end
        set $sweep_i=$sweep_i+1
      end
    end
    printf "SWEEP_ELEMENTS_HEAD\n"
    x/96gx $sweep_ptr
    continue
  end
  continue
end
run --ignored --exact hillslope::tests::stage3_laned_release_one_ofe_positive_baseline_profile --nocapture --test-threads=1
