import unittest

from routebook import normalize_destination


class NormalizeDestinationTests(unittest.TestCase):
    def test_trims_surrounding_city_whitespace(self):
        self.assertEqual(normalize_destination("  Madrid  ", "ES"), ("Madrid", "ES"))

    def test_trims_surrounding_country_whitespace(self):
        self.assertEqual(normalize_destination("Madrid", "  ES  "), ("Madrid", "ES"))

    def test_collapses_internal_city_whitespace(self):
        self.assertEqual(normalize_destination("New   York", "US"), ("New York", "US"))

    def test_uppercases_two_letter_country_code(self):
        self.assertEqual(normalize_destination("Madrid", "es"), ("Madrid", "ES"))

    def test_rejects_blank_city(self):
        with self.assertRaisesRegex(ValueError, "city must not be blank"):
            normalize_destination(" \t ", "ES")


if __name__ == "__main__":
    unittest.main()
