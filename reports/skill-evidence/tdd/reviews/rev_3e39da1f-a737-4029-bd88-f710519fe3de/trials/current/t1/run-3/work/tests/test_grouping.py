import unittest

from routebook import group_routes


class GroupRoutesTests(unittest.TestCase):
    def test_groups_routes_by_normalized_country_code(self):
        routes = [
            {"id": "r1", "city": " Madrid ", "country": "es"},
            {"id": "r2", "city": "Porto", "country": "pt"},
        ]
        self.assertEqual(
            group_routes(routes),
            {
                "ES": [{"id": "r1", "city": "Madrid", "country": "ES"}],
                "PT": [{"id": "r2", "city": "Porto", "country": "PT"}],
            },
        )

    def test_sorts_each_group_by_normalized_city_then_route_id(self):
        routes = [
            {"id": "r2", "city": " Rome ", "country": "it"},
            {"id": "r1", "city": "Rome", "country": "IT"},
            {"id": "r9", "city": "  Milan", "country": "it"},
        ]
        self.assertEqual(
            [route["id"] for route in group_routes(routes)["IT"]],
            ["r9", "r1", "r2"],
        )

    def test_rejects_duplicate_route_ids_across_countries(self):
        routes = [
            {"id": "r1", "city": "Madrid", "country": "ES"},
            {"id": "r1", "city": "Porto", "country": "PT"},
        ]
        with self.assertRaisesRegex(ValueError, "duplicate route id: r1"):
            group_routes(routes)


if __name__ == "__main__":
    unittest.main()
