"""Report selected operand joins without changing values or asserting native acceptance."""
import hashlib
from pathlib import Path
from decode_pair import decode_payload, exact_json


def inspect(directory):
    def read(name):
        return decode_payload((directory/name).read_bytes())
    g=read('123091.row.json');c=read('123092.row.json')
    inp=read('123092.input_typed_bytes.raw.json')
    b=read('123092.beginning_typed_bytes.raw.json')
    w=read('123092.working_typed_bytes.raw.json')
    parent=read('123092.parent_working_typed_bytes.raw.json')
    binding=c['coupled_binding']
    result={
        'evidence_class':'Ran: bounded decoded operand comparison; Static: source interpretation remains separate',
        'input_transaction_id':inp['transaction_id'],
        'input_day_interval':[inp['day_index'],inp['interval_index']],
        'input_interval_s':inp['interval_s'],
        'guard_metadata':{k:v for k,v in g.items() if not k.endswith('_typed_bytes')},
        'caller_metadata':{k:v for k,v in c.items() if not k.endswith('_typed_bytes')},
        'parent_schema':parent['schema'],
        'parent_day_interval':[parent['parent_day_index'],parent['parent_interval_index']],
        'parent_accepted_until_ns':parent['accepted_until_ns'],
        'parent_finalizations':parent['parent_finalizations'],
        'resource_continuations':b['continuations'],
        'equalities':{
            'guard_caller_input_exact_bytes':(directory/'123091.input_typed_bytes.raw.json').read_bytes()==(directory/'123092.input_typed_bytes.raw.json').read_bytes(),
            'guard_caller_beginning_exact_bytes':(directory/'123091.beginning_typed_bytes.raw.json').read_bytes()==(directory/'123092.beginning_typed_bytes.raw.json').read_bytes(),
            'caller_beginning_working_exact_bytes':(directory/'123092.beginning_typed_bytes.raw.json').read_bytes()==(directory/'123092.working_typed_bytes.raw.json').read_bytes(),
            'parent_persistent_beginning_equals_resource_beginning':parent['persistent_beginning_state']==b,
            'parent_candidate_equals_resource_working':parent['candidate_state']==w,
            'parent_parameters_equal_input':parent['parameters']==inp['wb14_parameters'],
        },
        'ofe_authorities':{},
        'payload_file_sha256':{p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(directory.glob('*.raw.json'))},
        'limits':'Operand equality and recorded prefix lineage are not complete prefix authentication, full current-context restoration, conservation, reader, witness or restart acceptance.'}
    for ofe,item in parent['per_ofe_authorities'].items():
        a=item['authority'];prefix=a.get('inactive_prefix')
        r={'authority_day_interval':[a['parent_day_index'],a['parent_interval_index']],
           'authority_day_interval_matches_outer_parent':(a['parent_day_index'],a['parent_interval_index'])==(parent['parent_day_index'],parent['parent_interval_index']),
           'beginning_cursor':item['beginning_cursor'],'working':item['working'],
           'receipts':item['receipts'],'inactive_prefix':prefix}
        if prefix is not None:
            r['lineage_hex']={k:bytes(v).hex() for k,v in prefix.items() if isinstance(v,list)}
            r['joins']={
                'prefix_end_equals_child_start':prefix['prefix_end_ns']==int(binding['child_support_start_ns']),
                'prefix_end_equals_parent_accepted_until':prefix['prefix_end_ns']==parent['accepted_until_ns'],
                'prefix_end_equals_working_accepted_until':prefix['prefix_end_ns']==item['working']['accepted_until_ns'],
                'prefix_parent_support_matches_binding':(prefix['parent_support_start_ns'],prefix['parent_support_end_ns'])==(int(binding['parent_support_start_ns']),int(binding['parent_support_end_ns'])),
                'prefix_coupled_parent_matches_authority_and_binding':prefix['coupled_parent_transaction_sha256']==a['coupled_parent_transaction_sha256']==binding['coupled_parent_transaction_sha256'],
                'prefix_beginning_owner_matches_authority':prefix['parent_beginning_owner_sha256']==a['parent_beginning_owner_sha256'],
                'prefix_ending_owner_matches_binding_beginning_complete_owner':prefix['prefix_ending_owner_sha256']==binding['parent_beginning_complete_owner_set_sha256'],
            }
        result['ofe_authorities'][ofe]=r
    return result


if __name__=='__main__':
    import sys
    print(exact_json(inspect(Path(sys.argv[1]))))
