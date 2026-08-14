import unittest

from numbers import clamp


class ExistingTests(unittest.TestCase):
    def test_module_imports(self):
        import numbers  # noqa: F401

    def test_clamp_returns_minimum_when_value_is_below_minimum(self):
        result = clamp(-2, 3, 9)

        self.assertEqual(3, result)
        self.assertNotEqual(-2, result)


if __name__ == "__main__":
    unittest.main()
