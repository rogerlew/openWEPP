from pathlib import Path
import re,difflib
r=Path('/workdir/openwepp-experiments/b01-wb14-cadence/native-context-restoration-reader-20260916')
a=Path(__file__).resolve().parent
p=r/'crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow.rs';old=p.read_text();s=old
start=s.index('    pub(crate) fn diagnostic_restore_native_consumer_v1(');end=s.index('\n    }',s.index('        Ok(restored)',start))+6
fn=s[start:end]
pat=r'serde_json::from_value\(get\(\s*"([^"\n]+)"\s*,?\s*\)\?\)\s*\.map_err\(\|(?:e|error)\|\s*\{\s*DirectV10RealConsumerError::Runtime\(DirectV9RealConsumerError::Serialization\(\s*(?:e|error)\.to_string\(\),?\s*\)\)\s*\}\)\?'
fn,n=re.subn(pat,lambda m:'diagnostic_native_field(captured, "'+m.group(1)+'")?',fn)
print('simplified field decoders',n)
gstart=fn.index('        let get = |name| {');gend=fn.index('        let vegetation_configuration',gstart)
assert 'get(' not in fn[gend:],fn[gend:]
fn=fn[:gstart]+fn[gend:]
bstart=fn.index('        {\n            if &frozen.lse_configuration');bend=fn.index('        let accepted_interval_count',bstart)
block=fn[bstart:bend]; assert block.endswith('        }\n')
body=block[len('        {\n'):-len('        }\n')]
body='\n'.join(line[4:] if line.startswith('    ') else line for line in body.splitlines()).replace('restored.install_restored_frozen_litter_v4_residents','self.install_restored_frozen_litter_v4_residents')
helper='''
    fn diagnostic_restore_frozen_litter_residents_v1(
        &mut self,
        frozen: DiagnosticFrozenLitterResidentsV1,
        pinned_frozen_litter_lse_configuration: &LandSurfaceEnergyConfiguration,
        pinned_surface_configuration: &DirectSurfaceLiquidConfiguration,
    ) -> Result<(), DirectV10RealConsumerError> {
'''+body+'\n        Ok(())\n    }\n'
fn=fn[:bstart]+'''        restored.diagnostic_restore_frozen_litter_residents_v1(
            frozen, pinned_frozen_litter_lse_configuration, pinned_surface_configuration,
        )?;
'''+fn[bend:]
s=s[:start]+fn+helper+s[end:]
m='include!("v9_real_consumer_shadow/v10_soil_thermal_v2.rs");'
helper='''
#[cfg(feature = "persisted-restart-v1")]
fn diagnostic_native_field<T: serde::de::DeserializeOwned>(
    captured: &serde_json::Value,
    name: &str,
) -> Result<T, DirectV10RealConsumerError> {
    let value = captured.get(name).cloned().ok_or_else(|| {
        DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Identity(
            "diagnostic native constructor operand",
        ))
    })?;
    serde_json::from_value(value).map_err(|error| {
        DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Serialization(
            error.to_string(),
        ))
    })
}
'''
s=s.replace(m,m+'\n'+helper)
(a/'native-constructor-quality-proposal.patch').write_text(''.join(difflib.unified_diff(old.splitlines(True),s.splitlines(True),fromfile='a/'+str(p.relative_to(r)),tofile='b/'+str(p.relative_to(r)))))
