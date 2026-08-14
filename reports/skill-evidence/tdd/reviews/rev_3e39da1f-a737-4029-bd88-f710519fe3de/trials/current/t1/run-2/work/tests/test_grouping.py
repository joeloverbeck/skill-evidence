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
            {"id": "r2", "city": "Madrid", "country": "es"},
            {"id": "r3", "city": "  A   Coruna ", "country": "es"},
            {"id": "r1", "city": "Madrid", "country": "es"},
        ]
        self.assertEqual(
            group_routes(routes)["ES"],
            [
                {"id": "r3", "city": "A Coruna", "country": "ES"},
                {"id": "r1", "city": "Madrid", "country": "ES"},
                {"id": "r2", "city": "Madrid", "country": "ES"},
            ],
        )

    def test_rejects_duplicate_route_ids(self):
        routes = [
            {"id": "r1", "city": "Madrid", "country": "es"},
            {"id": "r1", "city": "Paris", "country": "fr"},
        ]
        with self.assertRaises(ValueError):
            group_routes(routes)

    def test_preserves_distinct_route_ids_for_same_city(self):
        routes = [
            {"id": "r2", "city": "Madrid", "country": "es"},
            {"id": "r1", "city": " Madrid ", "country": "ES"},
        ]
        self.assertEqual(
            group_routes(routes)["ES"],
            [
                {"id": "r1", "city": "Madrid", "country": "ES"},
                {"id": "r2", "city": "Madrid", "country": "ES"},
            ],
        )


if __name__ == "__main__":
    unittest.main()
