import unittest
import pycrypto

class TestPyCrypto(unittest.TestCase):

  def test_hash(self):
    self.assertEqual(pycrypto.hash256("../crypto/ec-priv.pem"), "ae77017a14b0cfe03c375024618179b45a056f66e56c8f2020be3f21e2ef2737")
    self.assertEqual(pycrypto.hash160("../crypto/ec-priv.pem"), "d809411dc3e0db7e12390a80c6dead25052b6069")

    with self.assertRaises(Exception):
      pycrypto.hash256("nothere.txt")

  def test_pubkey(self):
    result = pycrypto.pubkey("../crypto/ec-priv.pem")

    self.assertEqual(result['uncompressed hex'], '04f6755afd57b6da43e8eec8144b5efe63f902ccc1980461fc66435671f54bea02147c8f924a1e7cbe66e6cdf06532136351d886468094a93f89e994fa8ebbd080')
    self.assertEqual(result['uncompressed base64'], 'BPZ1Wv1XttpD6O7IFEte/mP5AszBmARh/GZDVnH1S+oCFHyPkkoefL5m5s3wZTITY1HYhkaAlKk/iemU+o670IA=')
    self.assertEqual(result['uncompressed raw'], '[4, 246, 117, 90, 253, 87, 182, 218, 67, 232, 238, 200, 20, 75, 94, 254, 99, 249, 2, 204, 193, 152, 4, 97, 252, 102, 67, 86, 113, 245, 75, 234, 2, 20, 124, 143, 146, 74, 30, 124, 190, 102, 230, 205, 240, 101, 50, 19, 99, 81, 216, 134, 70, 128, 148, 169, 63, 137, 233, 148, 250, 142, 187, 208, 128]')
    self.assertEqual(result['compressed base64'], 'AvZ1Wv1XttpD6O7IFEte/mP5AszBmARh/GZDVnH1S+oC') 
    self.assertEqual(result['BTC p2pkh'], '1GGZnReKybChriBrvxEDWsQqQJBLQHvRzW')
    self.assertEqual(result['compressed raw'], '[2, 246, 117, 90, 253, 87, 182, 218, 67, 232, 238, 200, 20, 75, 94, 254, 99, 249, 2, 204, 193, 152, 4, 97, 252, 102, 67, 86, 113, 245, 75, 234, 2]')
    self.assertEqual(result['compressed hex'], '02f6755afd57b6da43e8eec8144b5efe63f902ccc1980461fc66435671f54bea02')

    with self.assertRaises(Exception):
      pycrypto.pubkey("../crypto/ec-pub.pem")

    with self.assertRaises(Exception):
      pycrypto.pubkey("notfound.pem")

  def test_prvkey(self):
    result: dict = pycrypto.prvkey("../crypto/ec-priv.pem")

    self.assertEqual(result['hex'], '94199c35c8848e03e9cb4380ef712bc077a5991fa0bbf2c4a40b0353e3ad6c27') 
    self.assertEqual(result['BTC wif'], 'L2Bbdwmcs188qfBWjhGi95P6sxVeGbvS1zQsnvpcAc4h1864jJXD') 
    self.assertEqual(result['base64'], 'lBmcNciEjgPpy0OA73ErwHelmR+gu/LEpAsDU+OtbCc=')
    self.assertEqual(result['raw'], '[148, 25, 156, 53, 200, 132, 142, 3, 233, 203, 67, 128, 239, 113, 43, 192, 119, 165, 153, 31, 160, 187, 242, 196, 164, 11, 3, 83, 227, 173, 108, 39]')

    with self.assertRaises(Exception):
      pycrypto.prvkey("../crypto/ec-pub.pem")

    with self.assertRaises(Exception):
      pycrypto.prvkey("notfound.pem")

  def test_dsa(self):
    pubkey = pycrypto.pubkey("../crypto/ec-priv.pem")["compressed hex"]
    sig = pycrypto.sign("../crypto/ec-priv.pem", "../crypto/ec-priv.pem")
    self.assertTrue(pycrypto.verify("../crypto/ec-priv.pem", pubkey, sig["signature"]))
    self.assertFalse(pycrypto.verify("../crypto/ec-pub.pem", pubkey, sig["signature"]))

  def test_vanity(self):
    result = pycrypto.vanity("AB", 4)
    self.assertIn("wif", result)
    self.assertIn("tries", result)
    self.assertIn("hex", result)
    self.assertEqual(result['p2pkh'][:3], "1AB")

    with self.assertRaises(Exception):
      pycrypto.vanity("Invalid.", 1)

    with self.assertRaises(ValueError):
      pycrypto.vanity("AB.", 1000)

if __name__ == "__main__":
  unittest.main()
