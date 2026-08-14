import unittest

from receipt import Receipt


class ReceiptTests(unittest.TestCase):
    def test_public_dictionary_shape(self):
        self.assertEqual(Receipt("r1").to_dict(), {"receipt_id": "r1"})


if __name__ == "__main__":
    unittest.main()
