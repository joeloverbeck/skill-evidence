import unittest

from routebook import group_routes


class GroupRoutesTests(unittest.TestCase):
    def test_groups_routes_by_normalized_country_code(self):
        routes = [{"id": "r1", "city": "  Madrid ", "country": "es"}]
        self.assertEqual(
            group_routes(routes),
            {"ES": [{"id": "r1", "city": "Madrid", "country": "ES"}]},
        )

    def test_sorts_each_group_by_normalized_city_then_route_id(self):
        routes = [
            {"id": "r3", "city": " Zurich", "country": "ch"},
            {"id": "r2", "city": "Bern", "country": "CH"},
            {"id": "r1", "city": "  Bern ", "country": "ch"},
        ]
        self.assertEqual(
            [route["id"] for route in group_routes(routes)["CH"]],
            ["r1", "r2", "r3"],
        )

    def test_rejects_duplicate_route_ids(self):
        routes = [
            {"id": "r1", "city": "Madrid", "country": "ES"},
            {"id": "r1", "city": "Paris", "country": "FR"},
        ]
        with self.assertRaisesRegex(ValueError, "duplicate route id: r1"):
            group_routes(routes)

    def test_preserves_distinct_route_ids_for_same_normalized_city(self):
        routes = [
            {"id": "r2", "city": "New   York", "country": "US"},
            {"id": "r1", "city": " New York ", "country": "us"},
        ]
        self.assertEqual(
            [route["id"] for route in group_routes(routes)["US"]],
            ["r1", "r2"],
        )


if __name__ == "__main__":
    unittest.main()
