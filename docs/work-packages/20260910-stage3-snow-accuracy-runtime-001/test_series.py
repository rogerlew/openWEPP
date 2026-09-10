import contextlib
import io
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch
import series

class SeriesFailureTest(unittest.TestCase):
    def test_early_failure_without_observation_retains_entire_schedule(self):
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory);binary=root/'binary';binary.write_bytes(b'fixture identity')
            output=root/'series';calls=[]
            def failed(binary,directory,policy,case,physical,ofes):
                directory.mkdir();calls.append((policy,ofes))
                if len(calls)%2==0:(directory/'observations.json').write_text('[]')
                return {'execution_valid':False,'runner_wall_s':None}
            with patch.object(series,'collect',side_effect=failed), patch('sys.argv',['series',str(binary),str(output),'--candidate','B01','--source',str(binary),'--ofes','10']),contextlib.redirect_stdout(io.StringIO()):
                series.main()
            result=json.loads((output/'summary.json').read_text())
            self.assertEqual(len(calls),16)
            self.assertTrue(all(ofes==10 for _,ofes in calls))
            self.assertEqual(json.loads((output/'protocol.json').read_text())['ofes'],10)
            self.assertEqual(result['failure_count'],16)
            self.assertEqual(result['warmup_failure_count'],4)
            self.assertFalse(result['candidates']['B01']['all_pairs_valid'])
            self.assertNotIn('geometric_mean_ratio',result['candidates']['B01'])
