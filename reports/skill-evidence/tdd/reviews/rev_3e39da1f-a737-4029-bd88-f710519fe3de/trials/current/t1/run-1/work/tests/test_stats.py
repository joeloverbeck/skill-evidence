import unittest

from routebook import manifest_stats


class ManifestStatsTests(unittest.TestCase):
    def test_reports_total_route_count(self):
        stats = manifest_stats({"ES": [{}, {}], "FR": [{}]})

        self.assertEqual(stats["total"], 3)

    def test_reports_per_country_counts_in_alphabetical_order(self):
        stats = manifest_stats({"FR": [{}], "ES": [{}, {}]})

        self.assertEqual(list(stats["countries"].items()), [("ES", 2), ("FR", 1)])


if __name__ == "__main__":
    unittest.main()
