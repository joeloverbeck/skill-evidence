import unittest

from routebook import group_routes


class GroupRoutesTests(unittest.TestCase):
    def test_groups_by_normalized_country_code(self):
        routes = [{"id": "r1", "city": "Madrid", "country": "es"}]
        self.assertEqual(
            group_routes(routes),
            {"ES": [{"id": "r1", "city": "Madrid", "country": "ES"}]},
        )

    def test_sorts_each_group_by_normalized_city_then_route_id(self):
        routes = [
            {"id": "r2", "city": " Zurich ", "country": "ch"},
            {"id": "r9", "city": "Geneva", "country": "ch"},
            {"id": "r1", "city": "Geneva", "country": "ch"},
        ]
        self.assertEqual(
            [route["id"] for route in group_routes(routes)["CH"]],
            ["r1", "r9", "r2"],
        )

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
            {"id": "r1", "city": " Madrid ", "country": "es"},
        ]
        self.assertEqual(
            [route["id"] for route in group_routes(routes)["ES"]],
            ["r1", "r2"],
        )


if __name__ == "__main__":
    unittest.main()
