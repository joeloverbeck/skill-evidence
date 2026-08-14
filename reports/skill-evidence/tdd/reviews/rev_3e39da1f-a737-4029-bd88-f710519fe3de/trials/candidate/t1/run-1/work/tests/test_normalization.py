import unittest

from routebook import normalize_destination


class NormalizeDestinationTests(unittest.TestCase):
    def test_trims_surrounding_city_whitespace(self):
        self.assertEqual(normalize_destination("  Madrid \t", "es")[0], "Madrid")

    def test_collapses_internal_city_whitespace(self):
        self.assertEqual(normalize_destination("New \t  York", "us")[0], "New York")

    def test_uppercases_two_letter_country_code(self):
        self.assertEqual(normalize_destination("Oslo", "no"), ("Oslo", "NO"))

    def test_trims_country_whitespace_before_uppercasing(self):
        self.assertEqual(normalize_destination("Madrid", " es ")[1], "ES")

    def test_rejects_blank_city(self):
        with self.assertRaises(ValueError):
            normalize_destination(" \t ", "ES")


if __name__ == "__main__":
    unittest.main()
