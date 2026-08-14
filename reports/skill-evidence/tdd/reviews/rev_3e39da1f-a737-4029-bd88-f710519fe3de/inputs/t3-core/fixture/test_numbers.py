import unittest


class ExistingTests(unittest.TestCase):
    def test_module_imports(self):
        import numbers  # noqa: F401


if __name__ == "__main__":
    unittest.main()
