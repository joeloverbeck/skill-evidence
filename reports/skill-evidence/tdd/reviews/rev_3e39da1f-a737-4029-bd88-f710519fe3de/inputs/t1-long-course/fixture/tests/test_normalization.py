import unittest

from routebook import normalize_destination


class NormalizeDestinationTests(unittest.TestCase):
    def test_legacy_preserves_destination_spelling(self):
        self.assertEqual(normalize_destination("  New   York ", "us"), ("  New   York ", "us"))


if __name__ == "__main__":
    unittest.main()
