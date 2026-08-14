import unittest

from routebook import render_manifest


class RenderManifestTests(unittest.TestCase):
    def test_emits_stable_country_and_route_lines(self):
        groups = {
            "ES": [
                {"id": "r3", "city": "Seville", "country": "ES"},
                {"id": "r2", "city": "Madrid", "country": "ES"},
                {"id": "r1", "city": "Madrid", "country": "ES"},
            ],
            "DE": [{"id": "r4", "city": "Berlin", "country": "DE"}],
        }

        self.assertEqual(
            render_manifest(groups).splitlines(),
            ["DE:", "  r4: Berlin", "ES:", "  r1: Madrid", "  r2: Madrid", "  r3: Seville"],
        )

    def test_ends_with_exactly_one_newline_and_no_trailing_horizontal_whitespace(self):
        groups = {"ES": [{"id": "r1", "city": "Madrid", "country": "ES"}]}

        rendered = render_manifest(groups)

        self.assertTrue(rendered.endswith("\n"))
        self.assertFalse(rendered.endswith("\n\n"))
        self.assertTrue(all(line == line.rstrip(" \t") for line in rendered.splitlines()))

    def test_removes_trailing_horizontal_whitespace_from_direct_input(self):
        groups = {"ES": [{"id": "r1", "city": "Madrid \t", "country": "ES"}]}

        rendered = render_manifest(groups)

        self.assertTrue(all(line == line.rstrip(" \t") for line in rendered.splitlines()))


if __name__ == "__main__":
    unittest.main()
