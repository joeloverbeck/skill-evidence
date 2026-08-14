import unittest

from routebook import group_routes


class GroupRoutesTests(unittest.TestCase):
    def test_groups_routes_by_normalized_country_code(self):
        routes = [
            {"id": "r1", "city": " Madrid ", "country": "es"},
            {"id": "r2", "city": "Seville", "country": "ES"},
        ]

        self.assertEqual(
            group_routes(routes),
            {
                "ES": [
                    {"id": "r1", "city": "Madrid", "country": "ES"},
                    {"id": "r2", "city": "Seville", "country": "ES"},
                ]
            },
        )

    def test_sorts_each_group_by_normalized_city_then_route_id(self):
        routes = [
            {"id": "a", "city": "Zurich", "country": "ch"},
            {"id": "c", "city": "Amsterdam", "country": "CH"},
            {"id": "b", "city": "  Amsterdam ", "country": "ch"},
        ]

        grouped = group_routes(routes)

        self.assertEqual([route["id"] for route in grouped["CH"]], ["b", "c", "a"])
        self.assertNotEqual([route["id"] for route in grouped["CH"]], ["a", "b", "c"])

    def test_rejects_duplicate_route_ids(self):
        routes = [
            {"id": "r1", "city": "Madrid", "country": "ES"},
            {"id": "r1", "city": "Lisbon", "country": "PT"},
        ]

        with self.assertRaises(ValueError):
            group_routes(routes)

    def test_preserves_distinct_route_ids_for_same_city(self):
        routes = [
            {"id": "r2", "city": "Madrid", "country": "ES"},
            {"id": "r1", "city": "Madrid", "country": "ES"},
        ]

        grouped = group_routes(routes)

        self.assertEqual([route["id"] for route in grouped["ES"]], ["r1", "r2"])


if __name__ == "__main__":
    unittest.main()
