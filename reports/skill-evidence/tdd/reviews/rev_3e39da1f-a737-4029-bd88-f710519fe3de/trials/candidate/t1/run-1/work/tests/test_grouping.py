import unittest

from routebook import group_routes


class GroupRoutesTests(unittest.TestCase):
    def test_groups_by_normalized_country_code(self):
        routes = [
            {"id": "r1", "city": "Madrid", "country": "es"},
            {"id": "r2", "city": "Barcelona", "country": "ES"},
        ]

        grouped = group_routes(routes)

        self.assertEqual(
            {
                country: {route["id"] for route in country_routes}
                for country, country_routes in grouped.items()
            },
            {"ES": {"r1", "r2"}},
        )

    def test_sorts_each_group_by_normalized_city_then_route_id(self):
        routes = [
            {"id": "r2", "city": "New   York", "country": "US"},
            {"id": "r3", "city": " Zurich ", "country": "US"},
            {"id": "r1", "city": " New York ", "country": "US"},
        ]

        grouped = group_routes(routes)

        self.assertEqual([route["id"] for route in grouped["US"]], ["r1", "r2", "r3"])

    def test_rejects_exact_duplicate_route_ids(self):
        routes = [
            {"id": "shared", "city": "Madrid", "country": "ES"},
            {"id": "shared", "city": "Lisbon", "country": "PT"},
        ]

        with self.assertRaises(ValueError):
            group_routes(routes)

    def test_preserves_distinct_route_ids_for_the_same_city(self):
        routes = [
            {"id": "r2", "city": "Rome", "country": "IT"},
            {"id": "r1", "city": "Rome", "country": "IT"},
        ]

        grouped = group_routes(routes)

        self.assertEqual({route["id"] for route in grouped["IT"]}, {"r1", "r2"})


if __name__ == "__main__":
    unittest.main()
