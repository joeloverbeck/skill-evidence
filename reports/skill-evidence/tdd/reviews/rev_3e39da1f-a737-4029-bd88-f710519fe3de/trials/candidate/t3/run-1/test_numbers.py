import unittest

from numbers import clamp


class ExistingTests(unittest.TestCase):
    def test_module_imports(self):
        import numbers  # noqa: F401


class ClampTests(unittest.TestCase):
    def test_value_below_minimum_returns_minimum(self):
        result = clamp(2, 5, 9)

        self.assertEqual(result, 5)
        self.assertNotEqual(result, 2)


if __name__ == "__main__":
    unittest.main()
