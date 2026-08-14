import unittest

from routebook import manifest_stats


class ManifestStatsTests(unittest.TestCase):
    def test_reports_total_route_count(self):
        groups = {"ES": [{}, {}], "PT": [{}]}

        self.assertEqual(manifest_stats(groups)["total"], 3)

    def test_reports_country_counts_in_alphabetical_order(self):
        groups = {"ES": [{}, {}], "DE": [{}], "PT": [{}, {}, {}]}

        stats = manifest_stats(groups)

        self.assertEqual(list(stats["countries"].items()), [("DE", 1), ("ES", 2), ("PT", 3)])
        self.assertNotEqual(list(stats["countries"].items()), [("ES", 2), ("DE", 1), ("PT", 3)])


if __name__ == "__main__":
    unittest.main()
