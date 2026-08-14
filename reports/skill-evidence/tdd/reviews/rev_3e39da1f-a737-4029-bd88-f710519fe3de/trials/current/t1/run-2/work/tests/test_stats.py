import unittest

from routebook import manifest_stats


class ManifestStatsTests(unittest.TestCase):
    def test_reports_total_route_count(self):
        self.assertEqual(manifest_stats({"ES": [{}, {}]})["total"], 2)

    def test_reports_country_counts_in_alphabetical_order(self):
        stats = manifest_stats({"FR": [{}], "ES": [{}, {}]})
        self.assertEqual(list(stats["countries"].items()), [("ES", 2), ("FR", 1)])


if __name__ == "__main__":
    unittest.main()
