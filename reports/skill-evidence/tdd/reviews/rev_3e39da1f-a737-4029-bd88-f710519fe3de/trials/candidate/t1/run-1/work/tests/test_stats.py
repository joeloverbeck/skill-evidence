import unittest

from routebook import manifest_stats


class ManifestStatsTests(unittest.TestCase):
    def test_reports_total_route_count(self):
        self.assertEqual(manifest_stats({"ES": [{}, {}], "PT": [{}]})["total"], 3)

    def test_reports_per_country_counts_in_alphabetical_order(self):
        groups = {"US": [{}, {}], "ES": [{}], "FR": [{}, {}, {}]}

        stats = manifest_stats(groups)

        self.assertEqual(
            list(stats["countries"].items()),
            [("ES", 1), ("FR", 3), ("US", 2)],
        )


if __name__ == "__main__":
    unittest.main()
