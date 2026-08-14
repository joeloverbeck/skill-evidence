import unittest

from routebook import manifest_stats


class ManifestStatsTests(unittest.TestCase):
    def test_reports_total_route_count(self):
        self.assertEqual(manifest_stats({"ES": [{}, {}]})["total"], 2)

    def test_reports_country_counts_in_alphabetical_order(self):
        stats = manifest_stats(
            {
                "US": [{}, {}],
                "ES": [{}],
                "PT": [{}, {}, {}],
            }
        )
        self.assertEqual(
            stats,
            {
                "total": 6,
                "countries": {"ES": 1, "PT": 3, "US": 2},
            },
        )
        self.assertEqual(list(stats["countries"]), ["ES", "PT", "US"])


if __name__ == "__main__":
    unittest.main()
