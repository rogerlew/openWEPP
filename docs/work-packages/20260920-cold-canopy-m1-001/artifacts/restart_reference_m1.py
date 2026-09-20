"""Synthetic encoding/guard controls, not a valid full simulation checkpoint."""
from pathlib import Path
import hashlib
import json


def canonical(obj):
    return (json.dumps(obj, sort_keys=True, separators=(',', ':'), ensure_ascii=False, allow_nan=False)+'\n').encode()


if __name__ == '__main__':
    root=Path(__file__).parent
    bindings={
        'model_definition_digest_sha256': {'control_kind':'synthetic_model_binding','model_definition_id':'OPENWEPP_C3_WOODY_COLD_M1_V1'},
        'configuration_digest_sha256': {'control_kind':'synthetic_configuration_binding','case':'restart_byte_control'},
        'topology_digest_sha256': {'control_kind':'synthetic_topology_binding','ofes':[{'id':'ofe-0','tiles':[{'id':'tile-0','occupancies':['upper','lower']}]}]},
        'source_manifest_sha256': {'control_kind':'synthetic_primary_source_register_binding','entries':[]},
    }
    digests={k:hashlib.sha256(canonical(v)).hexdigest() for k,v in bindings.items()}
    record=dict(schema_version=1,model_definition_id='OPENWEPP_C3_WOODY_COLD_M1_V1',**digests,
        transaction_id=17,consumed_transaction_cursor=17,day_index=3,support_start_s=120.0,support_end_s=180.0,
        ofe_id='ofe-0',tile_id='tile-0',occupancy_id='upper',area_basis='tile_ground',
        mass_unit='kg_h2o_m2',enthalpy_unit='J_m2',m_kg_h2o_m2=.018,h_j_m2=0.0)
    cases=[]
    for phase,h in [('ice',-6385.68),('mixed',-3003.3),('liquid',0.0)]:
        row={**record,'h_j_m2':h}
        encoded=canonical(row)
        cases.append(dict(name='valid_'+phase,record=row,canonical_utf8=encoded.decode(),
            artifact_sha256=hashlib.sha256(encoded).hexdigest(),expected_phase=phase,
            expected_phase_extension='admit_only_with_matching_synthetic_parent_context'))
    poison_specs=[
        ('unknown_field',{'extra':1},[], 'parse'),
        ('forcing_as_state',{'I_kg_m2_s':0.0},[],'parse'),
        ('missing_h',{},['h_j_m2'],'parse'),
        ('wrong_schema',{'schema_version':2},[],'version'),
        ('wrong_model',{'model_definition_id':'OPENWEPP_C3_WOODY_V11'},[],'version'),
        ('wrong_digest',{'configuration_digest_sha256':'0'*64},[],'identity'),
        ('bad_digest_syntax',{'topology_digest_sha256':'X'*64},[],'identity'),
        ('cursor_regression',{'consumed_transaction_cursor':16},[],'chronology'),
        ('staged_transaction',{'transaction_id':18},[],'chronology'),
        ('wrong_support',{'support_start_s':60.0},[],'chronology'),
        ('reversed_support',{'support_start_s':180.0,'support_end_s':120.0},[],'chronology'),
        ('wrong_ofe',{'ofe_id':'ofe-1'},[],'ownership'),
        ('wrong_occupancy',{'occupancy_id':'missing'},[],'ownership'),
        ('wrong_area',{'area_basis':'ofe_ground'},[],'units_area'),
        ('wrong_units',{'mass_unit':'mm'},[],'units_area'),
        ('mh_inconsistent',{'m_kg_h2o_m2':0.0,'h_j_m2':1.0},[],'mh_consistency'),
        ('negative_mass',{'m_kg_h2o_m2':-.018},[],'mh_consistency'),
        ('ice_below_domain',{'h_j_m2':-7000.0},[],'domain'),
        ('combined_version_mh',{'schema_version':2,'m_kg_h2o_m2':0.0,'h_j_m2':1.0},[],'version'),
        ('combined_unknown_version',{'extra':1,'schema_version':2},[],'parse'),
        ('combined_owner_mh',{'ofe_id':'ofe-1','m_kg_h2o_m2':0.0,'h_j_m2':1.0},[],'ownership'),
    ]
    poisons=[]
    for name,changes,deletions,stage in poison_specs:
        row={**record,**changes}
        for key in deletions: del row[key]
        poisons.append(dict(name=name,canonical_utf8=canonical(row).decode(),expected_error='VEG-E-144',first_rejection_stage=stage))
    poisons += [dict(name='nonfinite_json',canonical_utf8=canonical(record).decode().replace('"h_j_m2":0.0','"h_j_m2":NaN'),expected_error='VEG-E-144',first_rejection_stage='parse'),
        dict(name='truncated_json',canonical_utf8=canonical(record).decode()[:-2],expected_error='VEG-E-144',first_rejection_stage='parse')]
    output=dict(evidence_class='Synthetic byte and guard vectors only; no Rust or full-checkpoint execution',
        context='Digest preimages below are synthetic test context. Never claim these are actual model/config/source identities.',
        generator_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        external_bindings={k:dict(canonical_utf8=canonical(v).decode(),sha256=digests[k]) for k,v in bindings.items()},
        positives=cases,poisons=poisons,
        consumption_control=dict(first_load_cursor=17,next_transaction=18,attempted_reuse_transaction=17,expected_error='VEG-E-144'),
        required_later_evidence='Complete actual owner checkpoint, fresh-process split, future flux/closure and late-rollback verification are NOT RUN.')
    path=root/'restart-reference-m1.json'
    path.write_text(json.dumps(output,indent=2)+'\n')
    print(f'{path}: {len(cases)} positive encoding and {len(poisons)} poison controls')
