import unittest

from routebook import render_manifest


class RenderManifestTests(unittest.TestCase):
    def test_emits_stable_country_lines_followed_by_their_routes(self):
        groups = {
            "FR": [{"id": "r3", "city": "Paris", "country": "FR"}],
            "ES": [
                {"id": "r2", "city": "Seville", "country": "ES"},
                {"id": "r1", "city": "Madrid", "country": "ES"},
            ],
        }
        self.assertEqual(
            render_manifest(groups).splitlines(),
            ["ES:", "  r1: Madrid", "  r2: Seville", "FR:", "  r3: Paris"],
        )

    def test_ends_with_exactly_one_newline_without_trailing_horizontal_whitespace(self):
        groups = {"ES": [{"id": "r1", "city": "Madrid", "country": "ES"}]}

        self.assertEqual(render_manifest(groups), "ES:\n  r1: Madrid\n")

    def test_removes_trailing_horizontal_whitespace_from_each_line(self):
        groups = {"ES": [{"id": "r1", "city": "Madrid \t", "country": "ES"}]}

        self.assertEqual(render_manifest(groups), "ES:\n  r1: Madrid\n")


if __name__ == "__main__":
    unittest.main()
