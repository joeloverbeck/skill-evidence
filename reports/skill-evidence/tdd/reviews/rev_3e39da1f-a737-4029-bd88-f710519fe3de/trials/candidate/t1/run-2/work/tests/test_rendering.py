import unittest

from routebook import render_manifest


class RenderManifestTests(unittest.TestCase):
    def test_emits_countries_and_routes_in_stable_order(self):
        groups = {
            "US": [{"id": "r3", "city": "Boston", "country": "US"}],
            "ES": [
                {"id": "r2", "city": "Seville", "country": "ES"},
                {"id": "r1", "city": "Madrid", "country": "ES"},
            ],
        }
        self.assertEqual(
            render_manifest(groups).splitlines(),
            ["ES:", "  r1: Madrid", "  r2: Seville", "US:", "  r3: Boston"],
        )

    def test_ends_in_one_newline_without_trailing_horizontal_whitespace(self):
        groups = {"ES": [{"id": "r1", "city": "Madrid \t", "country": "ES"}]}
        self.assertEqual(render_manifest(groups), "ES:\n  r1: Madrid\n")


if __name__ == "__main__":
    unittest.main()
