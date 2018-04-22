extern crate openssl;

use openssl::ec::{EcKey,EcGroup,PointConversionForm};
use openssl::nid::Nid;
use openssl::bn::{BigNum,BigNumContext};


const SIZE: usize = 32;

fn main() {
  openssl::init();
  let vanity = String::from("1RUST");
  let group = EcGroup::from_curve_name(Nid::SECP256K1).unwrap(); 
  let mut ctx = BigNumContext::new().unwrap();

  let key = EcKey::generate(&group).unwrap();
  assert!(key.check_key().unwrap() == ()); // returns Result<(), ErrorStack>
  let bytes = key.public_key().to_bytes(&group, PointConversionForm::COMPRESSED, &mut ctx).unwrap();
  println!("pubkey {:?}", bytes);

  // TODO pre/append network/checksum...

  Base58_fromBytes(bytes)
}


static BASE58_DIGITS_BTC: &'static str = 
  "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

struct Base58 {

}

//impl Base58 {
  fn Base58_fromBytes(b: Vec<u8>) {
    //  if b.empty() {
    //    String::from("")
    //  }
    let mut ctx = BigNumContext::new().unwrap();
    let base = BigNum::from_u32(58).unwrap(); 
    let zero = BigNum::from_u32(0).unwrap(); 
    let mut r = BigNum::new().unwrap();
    let mut x = BigNum::from_slice(&b).unwrap();

    let mut i = 0;
    while x != zero {
      let mut tmpx = BigNum::new().unwrap();
      // BN_div(&x, &r, &x, &base, ctx);
      tmpx.div_rem(&mut r, &x, &base, &mut ctx);
      x = tmpx;
      // print r
      let j = r.mod_word(58).unwrap() as usize;
      //println!("{:?}", j);
      println!("{}", BASE58_DIGITS_BTC.as_bytes()[j] as char);
    }
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



  
  
//   let mut buf = [0; SIZE];
//   rand_bytes(&mut buf).unwrap();

//   // EC_KEY_new_by_curve_name(NID_secp256k1);
//   let prv_key = EcKey::from_curve_name(Nid::SECP256K1).unwrap(); 
  
//   // create an EcKey from the binary form of a EcPoint
//   let bignum = BigNum::from_slice(&buf).unwrap();
//   println!("rand {:?}", bignum);

// //   EC_KEY_set_private_key(ecKey, &priv);
 
//   // EC_KEY_get0_group(ecKey);
//   let group = prv_key.group();

//   let mut point = EcPoint::new(&group).unwrap();
//   let prv_key = EcKey::from_private_components(&group, &bignum, &point).unwrap(); 

//   let mut ctx = BigNumContext::new().unwrap();
//   let mut pubpt = EcPoint::new(&group).unwrap();
//   point.mul(&group, &pubpt, &bignum, &ctx).unwrap();
//   pubpt.mul(&group, &point, &bignum, &ctx).unwrap();
//   println!("prvkey check {:?}", prv_key.check_key());
//   println!("pubkey {:?}", prv_key.public_key().
//     to_bytes(&group, PointConversionForm::COMPRESSED, &mut ctx).unwrap().clone());

//   let pub_key = EcKey::from_public_key(&group, &pubpt).unwrap();
//   println!("pubkey check {:?}", pub_key.check_key());
//   println!("pubkey {:?}", pub_key.public_key().
//     to_bytes(&group, PointConversionForm::COMPRESSED, &mut ctx).unwrap().clone());



