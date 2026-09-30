"""Synthetic analysis controls only: no model or fixture execution."""
import copy
import importlib.util
from pathlib import Path
import unittest

path = Path(__file__).with_name('cost-attribution-analyze.py')
spec = importlib.util.spec_from_file_location('analyze', path)
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)


def rows():
    orders = [('warmup', ('R','D','E'), 16)] + [('triplet-' + str(i+1), o, 32) for i,o in enumerate(m.ORDERS)]
    out = []
    for label, order, count in orders:
        for arm in order:
            factor = {'R': 1, 'D': 1.04, 'E': 1.04*1.08}[arm]
            out.append({'label': label+'-'+arm, 'configuration':arm, 'count': count, 'exit_code':0,
                        'initialization': {'cpu_ns': int(count*1000*factor), 'wall_ns': int(count*2000*factor)},
                        'proposal': {'cpu_ns': int(count*3000*factor), 'wall_ns': int(count*6000*factor)},
                        'result': {'outcomes': [None]*count}})
    return out


class AnalysisTests(unittest.TestCase):
    def test_known_cost_ratios_and_combined(self):
        value = m.analyze_outer(rows())
        self.assertEqual(value['cohort_complete_outcomes'], 624)
        self.assertEqual(value['costs']['R']['combined']['cpu']['median'], 4)
        self.assertAlmostEqual(value['comparisons']['D/R']['proposal']['cpu']['median'], 1.04)
        self.assertAlmostEqual(value['comparisons']['E/D']['proposal']['wall']['median'], 1.08, places=4)
        self.assertTrue(value['comparisons']['E/D']['proposal']['wall']['paired_median_screen_pass'])

    def test_inventory_and_incomplete_rejected(self):
        for variant in ('missing', 'order', 'count', 'exit', 'nonpositive'):
            data = rows()
            if variant == 'missing': data.pop()
            if variant == 'order': data[3],data[4] = data[4],data[3]
            if variant == 'count': data[3]['count'] = 31
            if variant == 'exit': data[3]['exit_code'] = 1
            if variant == 'nonpositive': data[3]['initialization']['wall_ns'] = 0
            with self.assertRaises(ValueError): m.analyze_outer(data)

    def test_overhead_failure_not_silenced(self):
        data = rows()
        for row in data:
            if row['configuration'] == 'E': row['proposal']['wall_ns'] *= 2
        value=m.analyze_outer(data)
        self.assertFalse(value['comparisons']['E/D']['proposal']['wall']['paired_median_screen_pass'])

    def test_nested_reconciliation_and_reject_double_charge(self):
        row = rows()[2]
        phases = []
        for name in ('initialization','proposal'):
            metrics = [dict(name=n,inclusive_wall_ns=0,exclusive_wall_ns=0,calls=0,jacobi_sweeps=0) for n in m.SCOPE_NAMES]
            edges = [[0]*15 for _ in range(15)]
            metrics[0].update(inclusive_wall_ns=100,exclusive_wall_ns=50,calls=1)
            metrics[2].update(inclusive_wall_ns=20,exclusive_wall_ns=20,calls=1)
            metrics[3].update(inclusive_wall_ns=30,exclusive_wall_ns=30,calls=2)
            edges[0][2]=20; edges[0][3]=30
            phases.append(dict(name=name,metrics=metrics,direct_child_wall_ns=edges))
        row['result']['cost_attribution'] = dict(schema='m1-cost-attribution-v2',scope='complete-hierarchy',enabled=True,phases=phases)
        rec = m.reconcile(row)
        self.assertEqual(rec['initialization']['exclusive_wall_ns'],100)
        self.assertEqual(rec['initialization']['unassigned_wall_ns'],row['initialization']['wall_ns']-100)
        bad=copy.deepcopy(row);bad['result']['cost_attribution']['phases'][0]['metrics'][0]['exclusive_wall_ns']=100
        with self.assertRaises(ValueError): m.reconcile(bad)
        bad=copy.deepcopy(row);bad['result']['cost_attribution']['schema']='unknown'
        with self.assertRaises(ValueError): m.reconcile(bad)
        bad=copy.deepcopy(row);bad['initialization']['wall_ns']=99
        with self.assertRaises(ValueError): m.reconcile(bad)

    def test_amdahl_analytic_controls(self):
        self.assertEqual(m.amdahl(0), 1)
        self.assertEqual(m.amdahl(.5), 2)
        self.assertAlmostEqual(m.amdahl(.5, 2), 4/3)
        self.assertIsNone(m.amdahl(1))
        with self.assertRaises(ValueError): m.amdahl(1.1)


if __name__ == '__main__': unittest.main()
