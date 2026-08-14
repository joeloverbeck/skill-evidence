import unittest

from routebook import render_manifest


class RenderManifestTests(unittest.TestCase):
    def test_emits_stable_country_lines_followed_by_routes(self):
        groups = {
            "FR": [{"id": "r3", "city": "Paris", "country": "FR"}],
            "ES": [
                {"id": "r2", "city": "Seville", "country": "ES"},
                {"id": "r1", "city": "Madrid", "country": "ES"},
            ],
        }
        self.assertEqual(
            render_manifest(groups).splitlines(),
            ["ES", "  r1: Madrid", "  r2: Seville", "FR", "  r3: Paris"],
        )

    def test_ends_in_exactly_one_newline_without_trailing_horizontal_whitespace(self):
        groups = {"ES": [{"id": "r1", "city": "Madrid", "country": "ES"}]}
        rendered = render_manifest(groups)
        self.assertTrue(rendered.endswith("\n"))
        self.assertFalse(rendered.endswith("\n\n"))
        self.assertTrue(
            all(line.rstrip(" \t") == line for line in rendered.splitlines())
        )

    def test_strips_trailing_horizontal_whitespace_from_rendered_fields(self):
        groups = {
            "ES ": [{"id": "r1 ", "city": "Madrid\t ", "country": "ES"}],
        }
        rendered = render_manifest(groups)
        self.assertTrue(
            all(line.rstrip(" \t") == line for line in rendered.splitlines())
        )


if __name__ == "__main__":
    unittest.main()
