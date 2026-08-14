import unittest

from routebook import render_manifest


class RenderManifestTests(unittest.TestCase):
    def test_emits_countries_and_routes_in_stable_line_order(self):
        groups = {
            "US": [{"id": "r3", "city": "New York", "country": "US"}],
            "ES": [
                {"id": "r1", "city": "Madrid", "country": "ES"},
                {"id": "r2", "city": "Seville", "country": "ES"},
            ],
        }

        self.assertEqual(
            render_manifest(groups).splitlines(),
            [
                "ES:",
                "  r1: Madrid",
                "  r2: Seville",
                "US:",
                "  r3: New York",
            ],
        )

    def test_ends_with_exactly_one_newline_without_trailing_horizontal_whitespace(self):
        groups = {
            "ES": [{"id": "r1", "city": "Madrid \t", "country": "ES"}],
        }

        self.assertEqual(render_manifest(groups), "ES:\n  r1: Madrid\n")

    def test_cleans_physical_lines_for_direct_render_input(self):
        groups = {
            "ES": [{"id": "r1", "city": "Madrid \t\n", "country": "ES"}],
        }

        self.assertEqual(render_manifest(groups), "ES:\n  r1: Madrid\n")


if __name__ == "__main__":
    unittest.main()
