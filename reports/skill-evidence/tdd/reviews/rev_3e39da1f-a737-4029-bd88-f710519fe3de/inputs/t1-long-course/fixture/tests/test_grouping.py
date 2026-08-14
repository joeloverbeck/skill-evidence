import unittest

from routebook import group_routes


class GroupRoutesTests(unittest.TestCase):
    def test_legacy_groups_by_original_country(self):
        routes = [{"id": "r1", "city": "Madrid", "country": "es"}]
        self.assertEqual(group_routes(routes), {"es": routes})


if __name__ == "__main__":
    unittest.main()
