import unittest

from routebook import render_manifest


class RenderManifestTests(unittest.TestCase):
    def test_legacy_uses_debug_representation(self):
        groups = {"ES": [{"id": "r1", "city": "Madrid", "country": "ES"}]}
        self.assertEqual(render_manifest(groups), repr(groups))


if __name__ == "__main__":
    unittest.main()
