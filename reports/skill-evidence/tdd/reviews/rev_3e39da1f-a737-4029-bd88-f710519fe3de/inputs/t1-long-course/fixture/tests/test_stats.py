import unittest

from routebook import manifest_stats


class ManifestStatsTests(unittest.TestCase):
    def test_legacy_reports_total(self):
        self.assertEqual(manifest_stats({"ES": [{}, {}]}), {"total": 2})


if __name__ == "__main__":
    unittest.main()
