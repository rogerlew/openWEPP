"""Disposable administrative tests; no real model experiments."""
import copy
import json
import os
from pathlib import Path
import shutil
import stat
import subprocess
import tempfile
import unittest
import re
import tomllib
from unittest import mock

import context_report as context
import evidence_bundle as eb


class CaptureTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix='openwepp-custody-')
        self.addCleanup(self.temp.cleanup)
        self.base = Path(self.temp.name)
        self.repo = self.base/'original'
        self.repo.mkdir()
        eb.git(self.repo,'init','--quiet')
        for name,data in {'main':'base\n','gone':'delete\n','old':'rename\n',
                          'run.sh':'#!/bin/sh\ncat "$1"\n'}.items():
            (self.repo/name).write_text(data)
        eb.git(self.repo,'add','--','main','gone','old','run.sh')
        eb.git(self.repo,'-c','user.name=Fixture','-c','user.email=fixture@invalid',
               '-c','commit.gpgsign=false','commit','--quiet','-m','base')
        (self.repo/'main').write_text('staged\n')
        eb.git(self.repo,'add','--','main')
        (self.repo/'main').write_text('unstaged\n')
        eb.git(self.repo,'mv','old','renamed')
        eb.git(self.repo,'rm','--quiet','gone')
        (self.repo/'run.sh').chmod(0o755)
        (self.repo/'extra').write_text('untracked\n')
        (self.repo/'safe-link').symlink_to('main')
        (self.repo/'fixture').write_text('restored fixture\n')
        (self.repo/'protocol').write_text('run executable with fixture; expect exact output\n')
        (self.repo/'result').write_text('restored fixture\n')
        self.spec=dict(files=dict(source=['main','old','renamed','gone','run.sh','extra','safe-link'],
                                  executable=['run.sh'],fixture=['fixture'],protocol=['protocol'],result=['result']),
                       build_command=['cp','run.sh','measured-run.sh'],toolchain='POSIX /bin/sh; no compilation',
                       commands=[['sh','run.sh','fixture']],environment=['LANG'],
                       external_dependencies=[{'path':'/bin/sh','identity':'host shell; fixture portability only'}],
                       limitations='declared source only, no rebuild or full history claim',
                       retention='owner retains through disposition and explicit audit release')
        self.bundle=self.base/'bundle'

    def capture(self):
        return eb.capture(self.repo,self.spec,self.bundle)

    def mutate_manifest(self, fn):
        p=self.bundle/'manifest.json'
        m=json.loads(p.read_text()); fn(m)
        data=eb.canonical(m); p.write_bytes(data)
        (self.bundle/'manifest.sha256').write_text(eb.digest(data)+'\n')

    def test_independent_source_and_index_restoration(self):
        self.capture()
        # Original checkout and its Git objects cease to exist before restore.
        shutil.rmtree(self.repo)
        dest=self.base/'restored'
        eb.restore(self.bundle,dest)
        self.assertEqual((dest/'source/main').read_bytes(),b'unstaged\n')
        self.assertEqual(eb.git(dest/'source','show',':main'),b'staged\n')
        self.assertFalse((dest/'source/gone').exists())
        self.assertFalse((dest/'source/old').exists())
        self.assertEqual((dest/'source/renamed').read_bytes(),b'rename\n')
        self.assertEqual((dest/'source/extra').read_bytes(),b'untracked\n')
        self.assertEqual(os.readlink(dest/'source/safe-link'),'main')
        self.assertEqual(stat.S_IMODE((dest/'source/run.sh').stat().st_mode),0o755)
        self.assertIn(b'100644',eb.git(dest/'source','ls-files','--stage','run.sh'))
        actual=subprocess.check_output([str(dest/'executable/run.sh'),str(dest/'fixture/fixture')])
        self.assertEqual(actual,(dest/'result/result').read_bytes())

    def test_missing_input(self):
        self.spec['files']['fixture']=['missing']
        with self.assertRaises(ValueError): self.capture()
        self.assertFalse(self.bundle.exists())

    def test_destination_existing_and_overlap(self):
        self.bundle.mkdir()
        with self.assertRaises(ValueError): self.capture()
        with self.assertRaises(ValueError): eb.capture(self.repo,self.spec,self.repo/'bundle')

    def test_dotdot_destination_overlap(self):
        (self.base/'other').mkdir()
        with self.assertRaises(ValueError):
            eb.capture(self.repo,self.spec,self.base/'other/../original/bundle')
        self.assertFalse((self.repo/'bundle').exists())

    def test_chained_symlink_escape(self):
        (self.repo/'alias').symlink_to('.')
        (self.repo/'escape').symlink_to('alias/../fixture/fixture')
        self.spec['files']['source'] += ['alias','escape']
        with self.assertRaises(ValueError): self.capture()
        self.assertFalse(self.bundle.exists())

    def test_staged_chained_symlink_escape(self):
        link=self.repo/'safe-link'
        link.unlink(); link.symlink_to('alias/../fixture/fixture')
        eb.git(self.repo,'add','--','safe-link')
        link.unlink(); link.symlink_to('main')
        with self.assertRaises(ValueError): self.capture()

    def test_corruption_and_truncation(self):
        m=self.capture(); p=self.bundle/'blobs'/m['entries'][0]['sha256']
        p.write_bytes(b'corrupt')
        with self.assertRaises(ValueError): eb.verify(self.bundle)
        (self.bundle/'manifest.json').write_bytes(b'{')
        with self.assertRaises(ValueError): eb.verify(self.bundle)

    def test_archive_path_attacks(self):
        self.capture()
        original=(self.bundle/'manifest.json').read_bytes()
        for unsafe in ('../escape','/absolute','a/../../escape','a\\b','.git/config','a//b'):
            (self.bundle/'manifest.json').write_bytes(original)
            self.mutate_manifest(lambda m: m['entries'][0].update(path=unsafe))
            with self.assertRaises(ValueError): eb.restore(self.bundle,self.base/'restored')
            self.assertFalse((self.base/'restored').exists())

    def test_external_symlink_and_symlink_parent(self):
        (self.repo/'safe-link').unlink(); (self.repo/'safe-link').symlink_to('/etc/passwd')
        with self.assertRaises(ValueError): self.capture()
        (self.repo/'safe-link').unlink(); (self.repo/'safe-link').symlink_to('../outside')
        with self.assertRaises(ValueError): self.capture()
        alias=self.base/'alias'; alias.symlink_to(self.base,target_is_directory=True)
        with self.assertRaises(ValueError): eb.capture(self.repo,self.spec,alias/'capture')

    def test_sensitive_and_env_inputs(self):
        self.spec['environment']=['ACCESS_TOKEN']
        with self.assertRaises(ValueError): self.capture()
        self.spec['environment']=[]; self.spec['files']['fixture']=['.env']
        (self.repo/'.env').write_text('not a credential; negative fixture')
        with self.assertRaises(ValueError): self.capture()

    def test_source_changes_during_capture(self):
        original=eb.entry; count=0
        def changing(path, logical):
            nonlocal count
            result=original(path,logical); count+=1
            if count==1: (self.repo/'main').write_text('changed while capturing\n')
            return result
        with mock.patch.object(eb,'entry',side_effect=changing):
            with self.assertRaises(ValueError): self.capture()
        self.assertFalse(self.bundle.exists())

    def test_unsupported_member_and_missing_blob(self):
        self.capture()
        self.mutate_manifest(lambda m:m['entries'][0].update(kind='device'))
        with self.assertRaises(ValueError): eb.verify(self.bundle)

    def test_missing_blob(self):
        self.capture()
        next((self.bundle/'blobs').iterdir()).unlink()
        with self.assertRaises(ValueError): eb.verify(self.bundle)

    def test_symlink_blob_and_manifest(self):
        self.capture()
        p=next((self.bundle/'blobs').iterdir()); p.unlink(); p.symlink_to('/etc/passwd')
        with self.assertRaises(ValueError): eb.verify(self.bundle)
        p=self.bundle/'manifest.json'; p.unlink(); p.symlink_to('/etc/passwd')
        with self.assertRaises(ValueError): eb.verify(self.bundle)

    def test_staged_symlink_restored_without_original(self):
        eb.git(self.repo,'add','--','safe-link')
        self.capture(); shutil.rmtree(self.repo)
        dest=self.base/'restored'; eb.restore(self.bundle,dest)
        self.assertEqual(eb.git(dest/'source','show',':safe-link'),b'main')
        self.assertIn(b'120000',eb.git(dest/'source','ls-files','--stage','safe-link'))

    def test_index_mutation(self):
        original=eb.entry; once=False
        def changing(path,logical):
            nonlocal once
            result=original(path,logical)
            if not once:
                once=True; eb.git(self.repo,'add','--','main')
            return result
        with mock.patch.object(eb,'entry',side_effect=changing):
            with self.assertRaises(ValueError): self.capture()

    def test_duplicate_and_path_collision_manifest(self):
        self.capture()
        self.mutate_manifest(lambda m:m['entries'].append(dict(m['entries'][0])))
        with self.assertRaises(ValueError): eb.verify(self.bundle)

    def test_restore_destination_conflict(self):
        self.capture(); dest=self.base/'restored'; dest.mkdir()
        with self.assertRaises(ValueError): eb.restore(self.bundle,dest)


class IdentityTests(unittest.TestCase):
    def test_different_revision_bytes_are_not_deduplicated(self):
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory); (root/'a').write_bytes(b'new\n')
            spec={'readings':{'old':dict(path='a',reason='old',revision='old'),
                              'new':dict(path='a',reason='new')},
                  'roles':{'review':{'bootstrap':['old','new'],'expansion':[]}}}
            with mock.patch.object(context.subprocess,'check_output',return_value=b'old\n'):
                result=context.report(root,spec)['roles']['review']
            self.assertEqual(result['bootstrap']['unique_bytes'],8)

    def test_mutation_memberships_do_not_classify_semantics(self):
        with tempfile.TemporaryDirectory(prefix='openwepp-identity-') as directory:
            root=Path(directory)
            membership=dict(experiment=['source','executable','fixture','protocol'],
                            evidence_claim=['claim.md','criteria.md','result','reconstruct.py'],
                            publication=['report.md'])
            for names in membership.values():
                for name in names: (root/name).write_text('original\n')
            base=eb.identity(root,membership)
            for name,expected in [('report.md','publication'),('claim.md','evidence_claim'),
                                  ('criteria.md','evidence_claim'),('result','evidence_claim'),
                                  ('reconstruct.py','evidence_claim'),('fixture','experiment'),
                                  ('source','experiment'),('executable','experiment'),('protocol','experiment')]:
                (root/name).write_text('changed\n')
                now=eb.identity(root,membership)
                self.assertEqual([k for k in base if base[k]!=now[k]],[expected])
                (root/name).write_text('original\n')
            # Broken reference, verdict, or typo in the SAME report all change
            # publication identity. Judgment/matrix, not hashing, decides reuse.
            for text in ('typo corrected','[authority](missing.md)','Verdict: FAIL'):
                (root/'report.md').write_text(text)
                self.assertNotEqual(eb.identity(root,membership)['publication'],base['publication'])

    def test_reading_ranges_repeat_and_recursive_dependencies(self):
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory); (root/'a').write_bytes(b'one\ntwo\n')
            spec={'readings':{'a':dict(path='a',lines=[1,1],reason='automatic',exposures=2,requires=['b']),
                              'b':dict(path='a',reason='mandatory whole file')},
                  'roles':{'author':{'bootstrap':['a'],'expansion':['b']}}}
            result=context.report(root,spec)['roles']['author']
            self.assertEqual(result['bootstrap']['unique_bytes'],8)
            self.assertEqual(result['bootstrap']['exposure_bytes'],16)
            self.assertEqual(result['combined_unique_bytes'],8)
            spec['readings']['b']['requires']=['a']
            with self.assertRaises(ValueError): context.report(root,spec)


class GovernanceTests(unittest.TestCase):
    def test_changed_markdown_links_and_anchors_resolve(self):
        root=Path(__file__).resolve().parents[2]
        owned=json.loads((root/'docs/work-packages/20260906-agent-context-and-evidence-efficiency-001/artifacts/owned-files.json').read_text())
        checked=0
        for name in owned['paths']:
            p=root/name
            if p.suffix!='.md' or not p.exists(): continue
            # Catalog history is unchanged except explicitly added current links;
            # avoid making unrelated historical link debt this package's scope.
            text=p.read_text()
            if name=='docs/work-packages/README.md': text=text.split('## 20260904-',1)[0]
            if name=='CLAUDE.md': continue  # inserted routing uses repository paths
            for target in re.findall(r'\]\(([^)]+)\)',text):
                if '://' in target or '<' in target or '{{' in target: continue
                path,_,anchor=target.partition('#')
                q=(p.parent/path) if path else p
                self.assertTrue(q.exists(),f'{name}: {target}')
                if anchor and q.is_file() and q.suffix=='.md':
                    headings=re.findall(r'^#+ (.+)$',q.read_text(),re.M)
                    slugs=[re.sub(r'[^\w\- ]','',h.lower()).replace(' ','-') for h in headings]
                    self.assertIn(anchor,slugs,f'{name}: {target}')
                checked+=1
        self.assertGreater(checked,0)

    def test_roles_and_science_routes_preserve_obligations(self):
        root=Path(__file__).resolve().parents[2]
        common=(root/'docs/work-packages/AGENTS.md').read_text()
        for file in ('role-authoring.md','role-implementation.md','role-review.md','role-verification.md','role-runner.md','science-obligations.md','specialized-workflows.md'):
            self.assertIn(file,common); self.assertTrue((root/'docs/work-packages'/file).is_file())
        science=(root/'docs/work-packages/science-obligations.md').read_text()
        for obligation in ('operand-lineage','independent reconstruction','real closure','anti-tautology','real downstream consumer','ASSUMED_FOR_EXECUTION'):
            self.assertIn(obligation,science)
        strategy=(root/'docs/standards/testing-and-gate-strategy.md').read_text()
        self.assertIn('A0/A1/A3 remain non-deferrable',strategy)
        self.assertIn('exact clean commit',strategy)
        self.assertIn('unknown impact',strategy.lower())
        for path in (root/'.codex/agents').glob('*.toml'):
            cfg=tomllib.loads(path.read_text())
            self.assertIn(cfg['model_reasoning_effort'],('low','medium','high'))
            self.assertNotIn('You may modify .codex',cfg.get('developer_instructions',''))
        config=tomllib.loads((root/'.codex/config.toml').read_text())
        self.assertIn('package_verifier',config['agents'])


if __name__=='__main__': unittest.main()
