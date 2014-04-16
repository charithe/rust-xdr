#![crate_id = "xdr#0.1"]
#![crate_type = "lib"]

//! Implementation of unpacking routines for External Data Representation (XDR) format.
//! Follows the RFC at https://tools.ietf.org/html/rfc4506 
//! Copyright 2014 Charith Ellawala: charith {at} lucideelectricdreams {dot} com

pub mod xdr {
    use std::str;

    static BYTE_LEN : uint = 8;
    static INT_BYTES : uint = 4;
    static UINT_BYTES : uint  = 4;

    pub struct Xdr<'r> {
        buffer: &'r[u8],
        curr_pos: uint,
        size: uint
    }

    impl <'r> Xdr<'r> {
        pub fn new(data : &'r[u8]) -> Xdr<'r> {
            Xdr { buffer: data, curr_pos: 0, size: data.len() }
        }
        pub fn unpack_uint(&mut self) -> Option<u32> {
            self.unpack_int_type(UINT_BYTES)
        }

        pub fn unpack_int(&mut self) -> Option<i32> {
            self.unpack_int_type(INT_BYTES)
        }

        fn unpack_int_type<T: Int+FromPrimitive>(&mut self, num_bytes: uint) -> Option<T> {
            if self.curr_pos + num_bytes <= self.size {
                let slice = self.buffer.slice(self.curr_pos, self.curr_pos + num_bytes);
                self.curr_pos += num_bytes;
                let mut shift_amt:T = FromPrimitive::from_uint((num_bytes - 1) * BYTE_LEN).unwrap();
                let mut ret_val:T = FromPrimitive::from_uint(0).unwrap();;
                for i in range(0,num_bytes) {
                    let mask:T = FromPrimitive::from_u8(slice[i]).unwrap();
                    ret_val = ret_val | (mask << shift_amt); 
                    shift_amt = shift_amt - FromPrimitive::from_uint(BYTE_LEN).unwrap();
                }
                Some(ret_val)
            }
            else {
                None
            }
        }

        pub fn unpack_string(&mut self) -> Option<~str> {
            let str_len = self.unpack_uint();
            if str_len.is_none() {
                return None
            }

            let slen = str_len.unwrap() as uint;
            if self.curr_pos + slen > self.size {
                return None
            }

            let slice = self.buffer.slice(self.curr_pos, self.curr_pos + slen);

            // calculate the padding amount
            let tmp = slen % 4;
            let padding = if tmp > 0 { 4 - tmp } else { 0 };
            self.curr_pos += slen + padding;

            str::from_utf8(slice).and_then(|s| Some(s.to_owned())) 
        }

    }
}

#[cfg(test)]
mod xdr_test {
    use xdr;

    #[test]
    fn test_unpack_uint_happy_case() {
        let buffer = ~[0u8,0u8,0u8,128u8,23u8,0u8,0u8];
        let mut x = xdr::Xdr::new(buffer);
        let v = x.unpack_uint();
        assert!(v.is_none() == false);
        assert!(v.unwrap() == 128);
    }

    #[test]
    fn test_unpack_uint_buffer_too_short() {
        let buffer = ~[0u8,0u8,128u8];
        let mut x = xdr::Xdr::new(buffer);
        let v = x.unpack_uint();
        assert!(v.is_none());
    }

    #[test]
    fn test_unpack_int_positive() {
        let buffer = ~[0u8,0u8,0u8,246u8,23u8,0u8,0u8];
        let mut x = xdr::Xdr::new(buffer);
        let v = x.unpack_int();
        assert!(v.is_none() == false);
        assert!(v.unwrap() == 246);
    }

    #[test]
    fn test_unpack_int_negative() {
        let buffer = ~[255u8,255u8,255u8,231u8,23u8,0u8,0u8];
        let mut x = xdr::Xdr::new(buffer);
        let v = x.unpack_int();
        assert!(v.is_none() == false);
        assert!(v.unwrap() == -25);
    }

    #[test]
    fn test_unpack_string_len_is_multiple_of_four() {
        let buffer = ~[0u8,0u8,0u8,4u8,82u8,85u8,83u8,84u8,0u8,0u8,0u8,25u8];
        let mut x = xdr::Xdr::new(buffer);
        let v = x.unpack_string();
        let next_val = x.unpack_uint();
        assert!(v.is_none() == false);
        assert!(v.unwrap() == ~"RUST");
        assert!(next_val.is_none() == false);
        assert!(next_val.unwrap() == 25);
    }

    #[test]
    fn test_unpack_string_len_is_not_multiple_of_four() {
        let buffer = ~[0u8,0u8,0u8,5u8,82u8,85u8,83u8,84u8,89u8,0u8,0u8,0u8,0u8,0u8,0u8,25u8];
        let mut x = xdr::Xdr::new(buffer);
        let v = x.unpack_string();
        let next_val = x.unpack_uint();
        assert!(v.is_none() == false);
        assert!(v.unwrap() == ~"RUSTY");
        assert!(next_val.is_none() == false);
        assert!(next_val.unwrap() == 25);
    }

    #[test]
    fn test_unpack_string_len_is_too_long() {
        let buffer = ~[0u8,0u8,0u8,45u8,82u8,85u8,83u8,84u8,89u8,0u8];
        let mut x = xdr::Xdr::new(buffer);
        let v = x.unpack_string();
        assert!(v.is_none());
    }
}

