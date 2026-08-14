import unittest

from numbers import clamp


class ExistingTests(unittest.TestCase):
    def test_module_imports(self):
        import numbers  # noqa: F401


class ClampTests(unittest.TestCase):
    def test_value_below_minimum_returns_minimum(self):
        self.assertEqual(clamp(-5, 0, 10), 0)


if __name__ == "__main__":
    unittest.main()
