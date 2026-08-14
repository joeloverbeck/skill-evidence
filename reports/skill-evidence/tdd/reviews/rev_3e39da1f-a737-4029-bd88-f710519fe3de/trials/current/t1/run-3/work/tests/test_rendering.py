import unittest

from routebook import render_manifest


class RenderManifestTests(unittest.TestCase):
    def test_emits_countries_and_routes_in_stable_order(self):
        groups = {
            "PT": [{"id": "r3", "city": "Porto", "country": "PT"}],
            "ES": [
                {"id": "r2", "city": "Seville", "country": "ES"},
                {"id": "r1", "city": "Madrid", "country": "ES"},
            ],
        }
        self.assertEqual(
            render_manifest(groups).splitlines(),
            [
                "ES:",
                "  r1: Madrid",
                "  r2: Seville",
                "PT:",
                "  r3: Porto",
            ],
        )

    def test_ends_in_one_newline_without_trailing_horizontal_whitespace(self):
        output = render_manifest(
            {"ES": [{"id": "r1", "city": "Madrid", "country": "ES"}]}
        )
        self.assertTrue(output.endswith("\n"))
        self.assertFalse(output.endswith("\n\n"))
        self.assertTrue(all(line == line.rstrip(" \t") for line in output.splitlines()))

    def test_removes_trailing_horizontal_whitespace_from_rendered_fields(self):
        output = render_manifest(
            {"ES": [{"id": "r1", "city": "Madrid \t", "country": "ES"}]}
        )
        self.assertEqual(output, "ES:\n  r1: Madrid\n")


if __name__ == "__main__":
    unittest.main()
