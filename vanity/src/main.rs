extern crate openssl;

use openssl::ec::{EcKey,EcGroup,PointConversionForm};
use openssl::nid::Nid;
use openssl::bn::{BigNum,BigNumContext};
use openssl::sha;
use openssl::hash::{hash, MessageDigest};

//const SIZE: usize = 32;

// returns Vec so can be prepended
pub fn hash160(data: &[u8]) -> Vec<u8> {
  let data1 = sha::sha256(data);
  let res = hash(MessageDigest::ripemd160(), &data1).unwrap().to_vec(); //to_vec().clone_into_array()
  // let mut a = [0; 20];
  // for i in 0..a.len() {
  //     // Panics if not enough input
  //     a[i] = res[i];
  // }
  // a
  res
}

pub fn hash256(data: &[u8]) -> [u8; 32] {
  sha::sha256(&sha::sha256(&data[..]))
}


fn main() {
  openssl::init();
  let _vanity = String::from("1RUST");
  let group = EcGroup::from_curve_name(Nid::SECP256K1).unwrap(); 
  let mut ctx = BigNumContext::new().unwrap();

  let key = EcKey::generate(&group).unwrap();
  assert!(key.check_key().unwrap() == ()); // returns Result<(), ErrorStack>
  let bytes = key.public_key().to_bytes(&group, PointConversionForm::COMPRESSED, &mut ctx).unwrap();
  //println!("pubkey {:?}", bytes);

  let mut data = hash160(&bytes);
  //let mut data = [0; 20].to_vec();
  
  // prepend network byte
  data.insert(0, 0);
  //let check = sha::sha256(&sha::sha256(&data1[..]));
  let check = hash256(&data);
  // append checksum...
  let addr = [&data[..], &check[0..4]].concat();
  //Base58_fromBytes(hash.to_vec());
  base58_from_bytes(addr);
}


// zero PKH: 111111111111111111114oLvT2
// 1GGZnReKybChriBrvxEDWsQqQJBLQHvRzW
// priv [0x00, 0x94, 0x19, 0x9c, 0x35, 0xc8, 0x84, 0x8e, 0x03, 0xe9, 0xcb, 0x43, 0x80, 0xef, 0x71, 0x2b, 0xc0, 0x77, 0xa5, 0x99, 0x1f, 0xa0, 0xbb, 0xf2, 0xc4, 0xa4, 0x0b, 0x03, 0x53, 0xe3, 0xad, 0x6c, 0x27]
// pub [0x04, 0xf6, 0x75, 0x5a, 0xfd, 0x57, 0xb6, 0xda, 0x43, 0xe8, 0xee, 0xc8, 0x14, 0x4b, 0x5e, 0xfe, 0x63, 0xf9, 0x02, 0xcc, 0xc1, 0x98, 0x04, 0x61, 0xfc, 0x66, 0x43, 0x56, 0x71, 0xf5, 0x4b, 0xea, 0x02, 0x14, 0x7c, 0x8f, 0x92, 0x4a, 0x1e, 0x7c, 0xbe, 0x66, 0xe6, 0xcd, 0xf0, 0x65, 0x32, 0x13, 0x63, 0x51, 0xd8, 0x86, 0x46, 0x80, 0x94, 0xa9, 0x3f, 0x89, 0xe9, 0x94, 0xfa, 0x8e, 0xbb, 0xd0, 0x80]
// pub [0x02, 0xf6, 0x75, 0x5a, 0xfd, 0x57, 0xb6, 0xda, 0x43, 0xe8, 0xee, 0xc8, 0x14, 0x4b, 0x5e, 0xfe, 0x63, 0xf9, 0x02, 0xcc, 0xc1, 0x98, 0x04, 0x61, 0xfc, 0x66, 0x43, 0x56, 0x71, 0xf5, 0x4b, 0xea, 0x02]

static BASE58_DIGITS_BTC: &'static str = 
  "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

// struct Base58 {

// }

//impl Base58 {
  fn base58_from_bytes(b: Vec<u8>) {
    if b.len() == 0 {
      () //String::from("")
    }
    let mut ctx = BigNumContext::new().unwrap();
    let base = BigNum::from_u32(58).unwrap(); 
    let zero = BigNum::from_u32(0).unwrap(); 
    let mut r = BigNum::new().unwrap();
    let mut x = BigNum::from_slice(&b).unwrap();

    let addr_len = b.len() * 138 / 100;

    let mut buf : Vec<char> = Vec::with_capacity(64);
    let mut i = 0;
    while x != zero {
      let mut tmpx = BigNum::new().unwrap();
      let _ = tmpx.div_rem(&mut r, &x, &base, &mut ctx);
      x = tmpx;
      let j = r.mod_word(58).unwrap() as usize;
      buf.push(BASE58_DIGITS_BTC.as_bytes()[j] as char);
      i += 1;
    }
    // add leading zeros
    while i < addr_len {
      buf.push(BASE58_DIGITS_BTC.as_bytes()[0] as char);
      i += 1;
    }
    let str: String = buf.into_iter().rev().collect();   
    println!("1111111111111111111114oLvT2\n{}", str);
    //1111111111111111111114oLvT2
  }
//}

// std::string toBase58StringImpl(const bytes& b, const char* const alphabet)
//   {
//     if (b.empty())
//       return "";

//     size_t len = b.size();
//     size_t str_len;

//     BN_CTX *ctx;
//     BIGNUM base, x, r;
//     int i;
    
//     str_len = len * 138 / 100 + 2;
//     std::string str(str_len, 0);

//     ctx = BN_CTX_new();
//     BN_CTX_start(ctx);

//     BN_init(&base);
//     BN_init(&x);
//     BN_init(&r);
//     BN_set_word(&base, 58);
//     BN_bin2bn(&b[0], len, &x);
    
//     i = 0;
//     while (!BN_is_zero(&x)) 
//     {
//       BN_div(&x, &r, &x, &base, ctx);
//       str[i] = alphabet[BN_get_word(&r)];
//       ++i;
//     }
//     for (size_t j = 0; j < len; ++j) 
//     {
//       if (b[j] != 0x00) 
//       {
//           break;
//       }
//       str[i] = alphabet[0];
//       ++i;
//     }
     
//     BN_clear_free(&r);
//     BN_clear_free(&x);
//     BN_free(&base);
//     BN_CTX_end(ctx);
//     BN_CTX_free(ctx);
    
//     // Trailing zeros need to be avoided
//     std::string::reverse_iterator it = str.rbegin();
//     while (*it == 0) 
//     {
//       ++it;
//     }
//     return std::string(it, str.rend());
//   }  



