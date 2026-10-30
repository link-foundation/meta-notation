"""Check the real publishing workflow without contacting crates.io.

Run: python -m unittest discover -s experiments -p 'test_*.py' -v
Requires PyYAML (see .github/workflows/ci.yml).
"""

import os
from pathlib import Path
import subprocess
import tempfile
import unittest

import yaml

ROOT = Path(__file__).resolve().parents[1]


class PublishWorkflowTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        workflow = yaml.safe_load((ROOT / '.github/workflows/publish-rust.yml').read_text())
        cls.job = workflow['jobs']['publish-rust']
        cls.auth = next(step for step in cls.job['steps'] if step.get('id') == 'prm_auth')
        cls.publish = next(step for step in cls.job['steps'] if step.get('name') == 'Publish to crates.io')

    def test_authentication_can_fall_back(self):
        self.assertEqual(self.job['permissions']['id-token'], 'write')
        self.assertTrue(self.auth.get('continue-on-error'))
        self.assertRegex(self.auth['uses'], r'^rust-lang/crates-io-auth-action@[0-9a-f]{40}$')
        self.assertLess(self.job['steps'].index(self.auth), self.job['steps'].index(self.publish))

    def test_token_priority(self):
        # Evaluate the workflow's OR expression using fake credentials only.
        expression = self.publish['env']['CARGO_REGISTRY_TOKEN']
        terms = [term.strip() for term in expression.removeprefix('${{').removesuffix('}}').split('||')]
        self.assertEqual(terms, ['steps.prm_auth.outputs.token', 'secrets.CARGO_REGISTRY_TOKEN', 'secrets.CARGO_TOKEN'])
        for values, expected in [
            (('oidc', 'registry', 'legacy'), 'oidc'),
            (('', 'registry', 'legacy'), 'registry'),
            (('', '', 'legacy'), 'legacy'),
            (('', '', ''), ''),
        ]:
            with self.subTest(values=values):
                context = dict(zip(terms, values))
                self.assertEqual(next((context[term] for term in terms if context[term]), ''), expected)

    def run_publish(self, token, cargo_exit=0):
        with tempfile.TemporaryDirectory() as directory:
            cargo = Path(directory) / 'cargo'
            cargo.write_text('#!/bin/sh\n[ "$#" = 1 ] && [ "$1" = publish ] || exit 99\n'
                             '[ "$PWD" = "$EXPECTED_DIRECTORY" ] || exit 98\n'
                             '[ "$CARGO_REGISTRY_TOKEN" = "fake-token" ] || exit 97\n'
                             'echo cargo-called\nexit "$CARGO_EXIT"\n')
            cargo.chmod(0o755)
            return subprocess.run(
                ['bash', '--noprofile', '--norc', '-eo', 'pipefail', '-c', self.publish['run']],
                cwd=ROOT / self.publish['working-directory'],
                env={**os.environ, 'PATH': directory + os.pathsep + os.environ['PATH'],
                     'CARGO_REGISTRY_TOKEN': token, 'CARGO_EXIT': str(cargo_exit),
                     'EXPECTED_DIRECTORY': str(ROOT / 'rust')},
                capture_output=True, text=True, timeout=10,
            )

    def test_publish_with_token(self):
        result = self.run_publish('fake-token')
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn('cargo-called', result.stdout)

    def test_publish_failure_fails_job(self):
        self.assertFalse(self.publish.get('continue-on-error', False))
        self.assertEqual(self.run_publish('fake-token', cargo_exit=42).returncode, 42)

    def test_missing_credentials_fail_before_cargo(self):
        result = self.run_publish('')
        self.assertNotEqual(result.returncode, 0)
        self.assertNotIn('cargo-called', result.stdout)
        self.assertIn('::error::', result.stdout)
        self.assertIn('CARGO_REGISTRY_TOKEN', result.stdout)
        self.assertIn('CARGO_TOKEN', result.stdout)


if __name__ == '__main__':
    unittest.main()
