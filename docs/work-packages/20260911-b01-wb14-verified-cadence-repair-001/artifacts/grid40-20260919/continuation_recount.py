"""Offline receipt/event/trace recount for the retained pair; no solver execution."""
from collections import Counter, defaultdict
import hashlib
import json
from pathlib import Path
import struct
import sys

HERE = Path(__file__).resolve().parent
RUNS = Path('/home/roger/openwepp-experiments/b01-wb14-grid40-run-evidence-20260919')

def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()

def require(condition, message):
    if not condition:
        raise ValueError(message)

def main(arm):
    d = RUNS / arm
    receipt = json.loads((d/'receipt.json').read_text())
    freeze = json.loads((RUNS/'pair-freeze.json').read_text())
    side = json.loads((d/'sidecar.json').read_text())
    trace = json.loads((d/'trace.json').read_text())
    events = [json.loads(line) for line in (d/'events.jsonl').read_text().splitlines()]
    counts = side['counts']
    terminal = side['terminal']
    require(receipt['process_exit'] == 0 and receipt['stop_reason'] is None and receipt['integrity_unchanged'], 'process/integrity failure')
    require(receipt['freeze_sha256'] == sha(RUNS/'pair-freeze.json'), 'freeze changed')
    for key in ['source_sha256', 'binary_sha256', 'input_sha256']:
        require(receipt[key] == freeze[key], 'identity '+key)
    for name, info in receipt['files'].items():
        require(sha(d/name) == info['sha256'] and (d/name).stat().st_size == info['bytes'], 'file drift '+name)
        require(info['bytes'] <= receipt['limits']['per_file'], 'per-file cap')
    require(receipt['elapsed_seconds'] <= 120, 'process wall cap')
    require(sum(p.stat().st_size for p in d.iterdir() if p.is_file()) <= 1024**3, 'aggregate cap')
    require(side['schema'] == 'openwepp.grid40.sidecar.v1' and side['arm'] == arm, 'sidecar identity')
    require(events == counts['events'], 'journal/sidecar mismatch')
    grid = 20 if arm == 'baseline' else 40
    require(counts['numerical_identity'] == f'EXP-B01-LSE-GRID40-20260919;strict=0..{grid};witness=0..20;updates=50', 'effective numerical identity')
    capture = trace['records'][0]
    original = json.loads(Path(freeze['input']).read_text())['records'][0]
    # Reuse the strict typed/binary64 comparator also for treatment input custody.
    from compare_baseline import compare
    compare(original, capture, '/solver_input')
    require(capture['value']['snow_accuracy_policy'] == 'R0' and capture['value']['caps'] is None, 'R0/Potential identity')
    require(not any(r['kind'] == 'diagnostic_directional_probe' for r in trace['records']), 'extra probe')
    grouped = defaultdict(list)
    for event in events:
        grouped[event['operation_id']].append(event)
    require(set(grouped) == set(range(1, counts['next_operation_id']+1)), 'operation ID gaps')
    phases = Counter((e['counter_class'], e['phase']) for e in events)
    metrics = {}
    for cls in ['base','witness','strict','jacobian','ordinary','complete']:
        starts = {e['operation_id'] for e in events if e['counter_class']==cls and e['phase']=='started'}
        completed = {e['operation_id'] for e in events if e['counter_class']==cls and e['phase']=='completed'}
        errors = {e['operation_id'] for e in events if e['counter_class']==cls and e['phase']=='error'}
        require(not completed & errors and completed | errors <= starts, 'invalid lifecycle '+cls)
        inflight = starts - completed - errors
        require(len(starts) == phases[cls,'started'] and len(completed) == phases[cls,'completed'] and len(errors) == phases[cls,'error'], 'duplicate lifecycle '+cls)
        require(not inflight, 'in-flight operation '+cls)
        metrics[cls] = {'attempted':len(starts),'completed':len(completed),'errors':len(errors),'inflight':len(inflight)}
        if cls != 'jacobian': require(counts[cls] == len(starts), 'counter mismatch '+cls)
    for key, cls in [('domain_predicate','domain_predicate'),('jacobian_assembly','jacobian_assembly'),('updates','newton_update'),('signed_probe','signed_jacobian_probe')]:
        require(counts[key] == phases[cls,'started'], 'counter mismatch '+key)
    require(counts['complete'] == counts['ordinary'] + metrics['jacobian']['attempted'], 'complete join')
    require(counts['ordinary'] == counts['base']+counts['strict']+counts['witness'], 'ordinary join')
    signed = [e['operation_id'] for e in events if e['counter_class']=='signed_jacobian_probe' and e['phase']=='started']
    for oid in signed:
        own = [e for e in grouped[oid] if e['counter_class']=='signed_jacobian_probe']
        finishes = [e for e in own if e['phase'] in ['identity_anchor_complete','full_evaluation_complete','full_evaluation_error','child_complete_evaluation_denied','interrupted']]
        require(len(finishes)==1 and finishes[0]['phase'] in ['identity_anchor_complete','full_evaluation_complete'], 'signed lifecycle')
        children = [e for e in own if e['phase']=='child_complete_evaluation_started']
        require(len(children)==(finishes[0]['phase']=='full_evaluation_complete'), 'signed child classification')
        for child in children:
            joined = grouped[child['related_operation_id']]
            require(any(e['related_operation_id']==oid and e['counter_class']=='jacobian' and e['phase']=='started' for e in joined), 'signed child link')
    require(counts['signed_probe'] == phases['signed_jacobian_probe','identity_anchor_complete'] + metrics['jacobian']['attempted'], 'signed/identity double counting')
    require(counts['errors']==metrics['complete']['errors']==0 and counts['inflight']==0 and counts['inflight_operation_ids']==[] and counts['denied']==0 and counts['budget_stop'] is None, 'invalid/error/budget measurement')
    require(not any(not e['admitted'] or e['phase'] in ['denied','interrupted'] for e in events), 'denied/interrupted journal')
    caps={'base':51,'jacobian_assembly':50,'updates':50,'signed_probe':2900,'domain_predicate':20000,'witness':100,'strict':50*(grid+1),'ordinary':1201 if arm=='baseline' else 2201,'complete':4101 if arm=='baseline' else 5101}
    for key, cap in caps.items(): require(0<=counts[key]<=cap,'cost ceiling '+key)
    attempts=[r['value'] for r in trace['records'] if r['kind']=='line_search_attempt']
    require([r['exponent'] for r in attempts]==counts['strict_attempted_exponents'],'strict attempt exponent join')
    bits=lambda xs:[struct.unpack('>Q',struct.pack('>d',x))[0] for x in xs]
    require([bits(r['coordinates']) for r in attempts]==counts['strict_attempted_trial_bits'],'strict attempted coordinate bits')
    require(all(0<=r['exponent']<=grid for r in attempts),'strict exponent cap')
    accepted=[r['exponent'] for r in attempts if r['accepted']]
    require(accepted==counts['accepted_strict_exponents'] and len(accepted)==counts['updates'],'installed strict updates')
    bases=[r for r in trace['records'] if r['kind']=='accepted_base']
    require(len(bases)==counts['base'],'base trace join')
    require(sum(r['kind']=='linear_system' for r in trace['records'])==counts['jacobian_assembly'],'assembly trace join')
    if terminal['kind']=='Rejected':
        final=trace['records'][-1]
        require(final['kind']=='terminal_failure','missing terminal failure')
        failure=final['value']['failure']
        for s,t in [('failure_kind','kind'),('iterations','iterations'),('backtracking_count','backtracking_count'),('current_base_coordinates','failed_solution'),('ordered_residuals','ordered_residuals'),('normalized_residuals','normalized_residuals'),('step_norms','step_norms')]: compare(failure[t],terminal[s],'/terminal/'+s)
        require(terminal['iterations']==counts['updates'],'update/terminal join')
    if arm=='baseline': require(terminal['kind']=='Rejected' and terminal['failure_kind']=='BacktrackingLimit' and terminal['iterations']==6 and terminal['backtracking_count']==65,'baseline endpoint')
    result={'evidence_class':'Ran: offline typed/binary64 trace, receipt and event lifecycle reconstruction; no evaluator','arm':arm,'status':'PASS','freeze_sha256':sha(RUNS/'pair-freeze.json'),'receipt_sha256':sha(d/'receipt.json'),'trace_sha256':sha(d/'trace.json'),'sidecar_sha256':sha(d/'sidecar.json'),'journal_sha256':sha(d/'events.jsonl'),'tool_sha256':sha(Path(__file__)),'terminal':{k:v for k,v in terminal.items() if k in ['kind','failure_kind','iterations','backtracking_count']},'counts':{k:counts[k] for k in caps},'caps':caps,'lifecycle':metrics,'event_count':len(events),'operation_ids':len(grouped),'signed_identity_shortcuts':phases['signed_jacobian_probe','identity_anchor_complete'],'signed_full_evaluations':metrics['jacobian']['attempted'],'ci_bracket_loop_iterations':counts['ci_bracket_loop_iterations'],'ci_inventory_scope':'Actual bracket loop only; zero is not zero nested Ci work; endpoint/dark/final work remains under unchanged component limits','accepted_strict_exponents':accepted,'strict_attempted_candidates':len(attempts),'domain_invalid_strict_candidates':sum(not r['domain_valid'] for r in attempts),'errors':0,'inflight':0,'denied':0,'original_input_typed_binary64_match':True,'R0_uncapped_Potential':True,'extra_probes':0,'elapsed_seconds':receipt['elapsed_seconds']}
    (RUNS/(arm+'-cost-reconstruction.json')).write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps(result,indent=2))

if __name__=='__main__':
    main(sys.argv[1])
