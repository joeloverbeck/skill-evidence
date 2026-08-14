import unittest

from routebook import manifest_stats


class ManifestStatsTests(unittest.TestCase):
    def test_reports_total_route_count(self):
        self.assertEqual(manifest_stats({"ES": [{}, {}]})["total"], 2)

    def test_reports_country_counts_in_alphabetical_order(self):
        groups = {"US": [{"id": "r3"}], "ES": [{"id": "r1"}, {"id": "r2"}]}
        self.assertEqual(
            list(manifest_stats(groups)["countries"].items()),
            [("ES", 2), ("US", 1)],
        )


if __name__ == "__main__":
    unittest.main()
