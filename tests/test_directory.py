import importlib.machinery
import importlib.util
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

loader = importlib.machinery.SourceFileLoader('directory', str(Path(__file__).parents[1] / 'helper/monolito-directory-helper'))
spec = importlib.util.spec_from_loader(loader.name, loader)
m = importlib.util.module_from_spec(spec)
loader.exec_module(m)


def request(**overrides):
    return dict(domain='example.com', administrator='admin', password='test-only',
                unqualified_names=False, allow_weak_crypto=False, **overrides)


class DirectoryTests(unittest.TestCase):
    def test_dns_rejects_shell_options_and_invalid_labels(self):
        for bad in ['--help', 'a..com', '-a.com', 'a-.com', 'a.com\n', 'a.com;id', 'a/com', 'é.com']:
            self.assertFalse(m.valid_domain(bad), bad)
        self.assertTrue(m.valid_domain('ad-1.example.com'))

    def test_request_is_strict(self):
        self.assertEqual(m.validate(request())['domain'], 'example.com')
        for key, value in [('administrator', '--help'), ('password', 'a\nb'), ('allow_weak_crypto', 'false')]:
            r = request()
            r[key] = value
            with self.assertRaises(ValueError):
                m.validate(r)
        with self.assertRaises(ValueError):
            m.validate({**request(), 'command': 'id'})

    def test_sssd_changes_only_target_domain(self):
        original = '[sssd]\nservices = nss, pam\n[domain/example.com]\nid_provider = ad\n[domain/other.com]\nuse_fully_qualified_names = true\n'
        text = m.sssd_config(original, 'EXAMPLE.COM', True)
        self.assertIn('id_provider = ad', text)
        self.assertIn('use_fully_qualified_names = false', text)
        self.assertIn('[domain/other.com]\nuse_fully_qualified_names = true', text)
        self.assertEqual(text, m.sssd_config(text, 'example.com', True))
        self.assertNotIn('= false', m.sssd_config(text, 'example.com', False))

    def test_missing_domain_refuses_write(self):
        with self.assertRaises(ValueError):
            m.sssd_config('[sssd]\n', 'example.com', True)

    def test_kerberos_preserves_realms_and_is_repeatable(self):
        text = '[libdefaults]\n default_realm = EXAMPLE.COM\n allow_weak_crypto = false\n[realms]\n EXAMPLE.COM = {\n  kdc = dc.example.com\n }\n'
        updated = m.kerberos_config(text)
        self.assertIn('allow_weak_crypto = true', updated)
        self.assertEqual(updated, m.kerberos_config(updated))
        self.assertEqual(updated.split('[realms]')[1], text.split('[realms]')[1])
        with self.assertRaises(ValueError):
            m.kerberos_config('[libdefaults]\n[libdefaults]\n')

    def test_failed_join_restores_kerberos(self):
        with tempfile.TemporaryDirectory() as d:
            krb = Path(d) / 'krb5.conf'
            krb.write_text('[libdefaults]\n default_realm = EXAMPLE.COM\n')
            original = krb.read_text()
            def cmd(args, **kwargs):
                if args[1] == 'list':
                    return type('Output', (), {'stdout': b''})()
                raise RuntimeError('join failed')
            r = request()
            r['allow_weak_crypto'] = True
            with patch.object(m, 'KRB5', krb), patch.object(m, 'BACKUP', Path(d)/'backups'), patch.object(m, 'safe_read', return_value=(original, 0o644)), patch.object(m.shutil, 'which', return_value='/usr/bin/tool'), patch.object(m, 'command', side_effect=cmd):
                # Run this test as root in the isolated development container.
                if m.os.geteuid() != 0:
                    self.skipTest('root-owned temporary backup validation requires container root')
                self.assertEqual(m.join_locked(r), 1)
                self.assertEqual(krb.read_text(), original)
                self.assertEqual(r['password'], '')

    def test_partial_success_is_reported_without_leaving_domain(self):
        with tempfile.TemporaryDirectory() as d:
            calls = []
            def cmd(args, **kwargs):
                calls.append(args)
                return type('Output', (), {'stdout': b''})()
            with patch.object(m, 'BACKUP', Path(d)/'backups'), patch.object(m, 'safe_read', side_effect=OSError('missing sssd')), patch.object(m.shutil, 'which', return_value='/usr/bin/tool'), patch.object(m, 'command', side_effect=cmd):
                if m.os.geteuid() != 0:
                    self.skipTest('root-owned temporary backup validation requires container root')
                self.assertEqual(m.join_locked(request()), 10)
            self.assertTrue(any('join' in c for c in calls))
            self.assertFalse(any('leave' in c for c in calls))
            self.assertFalse(any('test-only' in str(c) for c in calls))


if __name__ == '__main__':
    unittest.main()
