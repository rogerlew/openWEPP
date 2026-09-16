from pathlib import Path
import subprocess
A=Path('/workdir/openWEPP/docs/work-packages/20260911-b01-wb14-verified-cadence-repair-001/artifacts/native-context-restoration-20260916')
b='/tmp/openwepp-b01-wb14-native-context-target/debug/deps/openwepp_hillslope_orchestrator-6acf9a74daf335ab'
for label,test in [('scoped-prefix-016','snow_free_member_export_restores_clock_and_parent_without_native_decode'),('scoped-provisional-017','snow_free_member_export_reconstructs_provisional_on_clone_from_pinned_seed'),('scoped-provider-018','prepared_day_member_restores_provider_and_forcing_component')]:
 r=subprocess.run(['/workdir/openWEPP/.venv/bin/python',str(A/'run_frozen_probe.py'),label,b,'snow_stage3_v11_current_context_capture::member_backed_restore_tests::'+test])
 if r.returncode: raise SystemExit(r.returncode)
