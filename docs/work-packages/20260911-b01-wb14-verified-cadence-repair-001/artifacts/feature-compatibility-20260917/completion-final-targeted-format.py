from pathlib import Path
from difflib import SequenceMatcher
import subprocess, shutil, tempfile
root=Path('/home/roger/openwepp-experiments/b01-wb14-feature-compatibility-20260917')
targets={
'crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_current_context_capture.rs':[(1560,1610),(1670,1790),(1980,2080),(2575,2610)],
'crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_restart.rs':[(620,670)],
'crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow.rs':[(1,120),(650,690),(780,875),(1470,1600),(4190,4240)],
'crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/native_soil_restart_admission.rs':[(25,95)],
'crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/v10_soil_thermal_v2.rs':[(1235,1290)],
'crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow_publication_retention.rs':[(1325,1400),(1670,1720)],
'crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow_v10_accessors.rs':[(250,305)],
'crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/direct_v9_real_consumer_shadow_impl.rs':[(190,245)],
'crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs':[(600,635)],
}
with tempfile.TemporaryDirectory() as td:
    td=Path(td)
    for rel,ranges in targets.items():
        src=root/rel; tmp=td/Path(rel).name; shutil.copy2(src,tmp)
        subprocess.run(['rustfmt','--edition','2024','--config','skip_children=true',str(tmp)],check=True)
        old=src.read_text().splitlines(keepends=True); new=tmp.read_text().splitlines(keepends=True)
        ops=SequenceMatcher(None,old,new).get_opcodes()
        selected=[]
        for tag,i1,i2,j1,j2 in ops:
            if tag=='equal': continue
            # 1-based old line interval; insertion attaches to preceding line.
            lo=i1+1; hi=max(i2, i1+1)
            if any(lo<=b and hi>=a for a,b in ranges): selected.append((i1,i2,j1,j2))
        for i1,i2,j1,j2 in reversed(selected): old[i1:i2]=new[j1:j2]
        if selected: src.write_text(''.join(old))
        print(rel, len(selected))
