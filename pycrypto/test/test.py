
import pycrypto

print(pycrypto.hash256("Cargo.toml"))
print(pycrypto.hash160("Cargo.toml"))

try:
  print(pycrypto.hash256("nothere.txt"))
except Exception as e:
  print("[expected error]", e)
  pass

print(pycrypto.pubkey("../crypto/ec-priv.pem"))
try:
  print(pycrypto.pubkey("../crypto/ec-pub.pem"))
except Exception as e:
  print("[expected error]", e)
  pass

print(pycrypto.prvkey("../crypto/ec-priv.pem"))


pubkey = pycrypto.pubkey("../crypto/ec-priv.pem")["compressed hex"]
print(pubkey)
sig = pycrypto.sign("../crypto/ec-priv.pem", "../crypto/ec-priv.pem")
print(sig)
print(pycrypto.verify("../crypto/ec-priv.pem", pubkey, sig["signature"]))
