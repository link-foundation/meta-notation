"""Exercise npm publishing configuration and commands without registry writes."""

import os
from pathlib import Path
import subprocess
import tempfile
import unittest

import yaml

ROOT = Path(__file__).resolve().parents[1]


class NpmPublishWorkflowTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.workflow = yaml.load(
            (ROOT / '.github/workflows/publish-npm.yml').read_text(), Loader=yaml.BaseLoader)
        cls.job = cls.workflow['jobs']['publish-npm']
        cls.steps = cls.job['steps']

    def test_oidc_permissions_and_supported_tools(self):
        self.assertEqual(self.job.get('permissions'), {'contents': 'read', 'id-token': 'write'})
        setup = next(step for step in self.steps if 'actions/setup-node@' in step.get('uses', ''))
        self.assertEqual(setup['with']['node-version'], '24.x')
        upgrade = next(step for step in self.steps if step.get('name') == 'Install npm CLI')
        self.assertEqual(upgrade['run'], 'npm install --global npm@11.20.0')
        publish = next(step for step in self.steps if step.get('name') == 'Publish to npm')
        self.assertLess(self.steps.index(upgrade), self.steps.index(publish))

    def test_releases_default_to_oidc_and_fallback_is_explicit(self):
        self.assertEqual(self.workflow['on']['release']['types'], ['published'])
        flag = self.workflow['on']['workflow_dispatch']['inputs']['use_token']
        self.assertEqual(flag['type'], 'boolean')
        self.assertEqual(flag['default'], 'false')
        publish = next(step for step in self.steps if step.get('name') == 'Publish to npm')
        fallback = next(step for step in self.steps if step.get('name') == 'Publish with token fallback')
        self.assertEqual(publish['if'], '${{ !inputs.use_token }}')
        self.assertEqual(fallback['if'], "${{ github.event_name == 'workflow_dispatch' && inputs.use_token }}")
        self.assertNotIn('env', publish)
        self.assertEqual(fallback['env'], {'NODE_AUTH_TOKEN': '${{ secrets.NPM_TOKEN }}'})
        for step in self.steps:
            if step != fallback:
                self.assertNotIn('secrets.NPM_TOKEN', str(step))
        self.assertNotEqual(self.job.get('continue-on-error'), 'true')

    def run_publish(self, step, token='', exit_code=0):
        with tempfile.TemporaryDirectory() as directory:
            npm = Path(directory) / 'npm'
            npm.write_text('#!/bin/sh\n[ "$#" = 1 ] && [ "$1" = publish ] || exit 99\n'
                           '[ "$PWD" = "$EXPECTED_DIRECTORY" ] || exit 98\n'
                           'echo npm-called\nexit "$NPM_EXIT"\n')
            npm.chmod(0o755)
            env = {key: value for key, value in os.environ.items()
                   if key not in ('NODE_AUTH_TOKEN', 'NPM_TOKEN')}
            return subprocess.run(
                ['bash', '--noprofile', '--norc', '-eo', 'pipefail', '-c', step['run']],
                cwd=ROOT / step['working-directory'],
                env={**env, 'PATH': directory + os.pathsep + os.environ['PATH'],
                     'NODE_AUTH_TOKEN': token, 'NPM_EXIT': str(exit_code),
                     'EXPECTED_DIRECTORY': str(ROOT / 'js')},
                capture_output=True, text=True, timeout=10,
            )

    def test_oidc_publish_needs_no_token_and_propagates_errors(self):
        step = next(step for step in self.steps if step.get('name') == 'Publish to npm')
        self.assertNotEqual(step.get('continue-on-error'), 'true')
        self.assertEqual(self.run_publish(step).returncode, 0)
        self.assertEqual(self.run_publish(step, exit_code=42).returncode, 42)

    def test_fallback_requires_token_and_propagates_errors(self):
        step = next(step for step in self.steps if step.get('name') == 'Publish with token fallback')
        self.assertNotEqual(step.get('continue-on-error'), 'true')
        missing = self.run_publish(step)
        self.assertNotEqual(missing.returncode, 0)
        self.assertNotIn('npm-called', missing.stdout)
        self.assertIn('::error::', missing.stdout)
        self.assertEqual(self.run_publish(step, token='fake-token').returncode, 0)
        self.assertEqual(self.run_publish(step, token='fake-token', exit_code=42).returncode, 42)


if __name__ == '__main__':
    unittest.main()
