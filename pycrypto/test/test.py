
import pycrypto

print(pycrypto.hash256("Cargo.toml"))
print(pycrypto.hash160("Cargo.toml"))

try:
  print(pycrypto.hash256("nothere.txt"))
except Exception as e:
  print("[expected error]", e)
  pass


