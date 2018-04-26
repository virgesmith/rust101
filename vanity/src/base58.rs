use openssl::bn::{BigNum,BigNumContext};

pub static DIGITS_BTC: &'static str = 
  "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

// struct Base58 {

// }

//impl Base58 {
  pub fn from_bytes(b: Vec<u8>) -> String {
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
      buf.push(DIGITS_BTC.as_bytes()[j] as char);
      i += 1;
    }
    // add leading zeros
    while i < addr_len {
      buf.push(DIGITS_BTC.as_bytes()[0] as char);
      i += 1;
    }
    let str: String = buf.into_iter().rev().collect();   
    str
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

// TODO implement
  // fn to_bytes(s: &str) -> Vec<u8> {
  //   let mut data: Vec<u8> = Vec{};

  //   data
  // }



// bytes fromBase58StringImpl(const std::string& s, const bytes& lookup, char zero)
//   {
//     std::string::const_iterator it = s.begin();
//     // Skip leading spaces.
//     while (*it && isspace(*it))
//     {
//       ++it;
//     }
//     // Skip and count leading zeros ('1' in BTC or 'r' in XRP)s.
//     int zeros = 0;
//     while (*it == zero)
//     {
//       ++zeros;
//       ++it;
//     }
//     // Allocate enough space in big-endian base256 representation.
//     bytes b256(s.size() * 733 / 1000 + 1); // log(58) / log(256), rounded up.
//     // Process the characters.
//     while (*it && !isspace(*it))
//     {
//       // Decode base58 character
// 	    // Apply "b256 = b256 * 58 + *it".
//       int carry = lookup[*it];
//       if (carry == 255)
//       {
//         throw std::runtime_error("Invalid character in base58 encoded string");
//       }
// 	    for (bytes::reverse_iterator it = b256.rbegin(); it != b256.rend(); it++)
// 	    {
// 	      carry += 58 * (*it);
// 	      *it = carry % 256;
// 	      carry /= 256;
// 	    }
// 	    assert(carry == 0);
// 	    ++it;
//     }
//     // Skip trailing spaces.
//     while (isspace(*it))
//     {
//       ++it;
//     }
    
//     if (*it != 0)
//     {
//       throw std::runtime_error("parsing base58 encoded string");
//     }
//     // Skip leading zeroes in b256.
//     bytes::iterator bit = b256.begin();
//     while (bit != b256.end() && *bit == 0)
//     {
//       ++bit;
//     }
//     // Copy result into output vector.
//     bytes b(zeros, 0);
//     b.reserve(zeros + (b256.end() - bit));
//     while (bit != b256.end())
//     {
//       b.push_back(*(bit++));
//     }
//     return b;
//   }
