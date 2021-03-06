#[cfg(test)]
mod tests {
    use xdr::{Xdr, XdrResult};

    #[test]
    fn unpack_u32_happy_case() {
        let buffer = ~[0u8,0u8,0u8,128u8,23u8,0u8,0u8];
        let mut x = Xdr::new(buffer);
        let v:XdrResult<u32> = x.unpack_primitive();
        assert!(v.is_err() == false);
        assert!(v.unwrap() == 128);
    }

    #[test]
    fn unpack_u32_buffer_too_short() {
        let buffer = ~[0u8,0u8,128u8];
        let mut x = Xdr::new(buffer);
        let v:XdrResult<u32> = x.unpack_primitive();
        assert!(v.is_err());
    }

    #[test]
    fn unpack_i32_positive() {
        let buffer = ~[0u8,0u8,0u8,246u8,23u8,0u8,0u8];
        let mut x = Xdr::new(buffer);
        let v:XdrResult<i32> = x.unpack_primitive();
        assert!(v.is_err() == false);
        assert!(v.unwrap() == 246);
    }

    #[test]
    fn unpack_i32_negative() {
        let buffer = ~[255u8,255u8,255u8,231u8,23u8,0u8,0u8];
        let mut x = Xdr::new(buffer);
        let v:XdrResult<i32> = x.unpack_primitive();
        assert!(v.is_err() == false);
        assert!(v.unwrap() == -25);
    }

    
    #[test]
    fn unpack_u64() {
        let buffer = ~[0u8,0u8,0u8,0u8,0u8,0u8,0u8,128u8,23u8,0u8,0u8];
        let mut x = Xdr::new(buffer);
        let v:XdrResult<u64> = x.unpack_primitive();
        assert!(v.is_err() == false);
        assert!(v.unwrap() == 128);
    }
    
    #[test]
    fn unpack_i64_positive() {
        let buffer = ~[0u8,0u8,0u8,0u8,0u8,0u8,0u8,246u8,23u8,0u8,0u8];
        let mut x = Xdr::new(buffer);
        let v:XdrResult<i64> = x.unpack_primitive();
        assert!(v.is_err() == false);
        assert!(v.unwrap() == 246);
    }

    #[test]
    fn unpack_i64_negative() {
        let buffer = ~[255u8,255u8,255u8,255u8,255u8,255u8,255u8,231u8,23u8,0u8,0u8];
        let mut x = Xdr::new(buffer);
        let v:XdrResult<i64> = x.unpack_primitive();
        assert!(v.is_err() == false);
        assert!(v.unwrap() == -25);
    }
    
    #[test]
    fn unpack_bool_true_value() {
        let buffer = ~[0u8,0u8,0u8,1u8,23u8,0u8,0u8];
        let mut x = Xdr::new(buffer);
        let v:XdrResult<bool> = x.unpack_primitive();
        assert!(v.is_err() == false);
        assert!(v.unwrap() == true);
    }

    #[test]
    fn unpack_bool_false_value() {
        let buffer = ~[0u8,0u8,0u8,0u8,23u8,0u8,0u8];
        let mut x = Xdr::new(buffer);
        let v:XdrResult<bool> = x.unpack_primitive();
        assert!(v.is_err() == false);
        assert!(v.unwrap() == false);
    }

    #[test]
    fn unpack_bool_invalid_value() {
        let buffer = ~[0u8,0u8,0u8,20u8,23u8,0u8,0u8];
        let mut x = Xdr::new(buffer);
        let v:XdrResult<bool> = x.unpack_primitive();
        assert!(v.is_err());
    }

    #[test]
    fn unpack_fixed_bytes_unpadded() {
        let buffer = ~[1u8,2u8,3u8,4u8,0u8,0u8,0u8,25u8];
        let mut x = Xdr::new(buffer);
        let v:XdrResult<~[u8]> = x.unpack_bytes(4);
        let next_val:XdrResult<u32> = x.unpack_primitive();

        assert!(v.is_err() == false);
        let bytes = v.unwrap();
        assert!(bytes.len() == 4);
        assert!(bytes == ~[1u8,2u8,3u8,4u8]);

        assert!(next_val.is_err() == false);
        assert!(next_val.unwrap() == 25);
    }

    #[test]
    fn unpack_fixed_bytes_padded() {
        let buffer = ~[1u8,2u8,3u8,4u8,5u8,6u8,0u8,0u8,0u8,0u8,0u8,25u8];
        let mut x = Xdr::new(buffer);
        let v:XdrResult<~[u8]> = x.unpack_bytes(6);
        let next_val:XdrResult<u32> = x.unpack_primitive();

        assert!(v.is_err() == false);
        let bytes = v.unwrap();
        assert!(bytes.len() == 6);
        assert!(bytes == ~[1u8,2u8,3u8,4u8,5u8,6u8]);

        assert!(next_val.is_err() == false);
        assert!(next_val.unwrap() == 25);
    }

    #[test]
    fn unpack_fixed_bytes_padded_len_too_long() {
        let buffer = ~[1u8,2u8,3u8,4u8,5u8,6u8,7u8];
        let mut x = Xdr::new(buffer);
        let v = x.unpack_bytes(7);

        assert!(v.is_err());
    }

    #[test]
    fn unpack_string_len_is_multiple_of_four() {
        let buffer = ~[0u8,0u8,0u8,4u8,82u8,85u8,83u8,84u8,0u8,0u8,0u8,25u8];
        let mut x = Xdr::new(buffer);
        let v = x.unpack_string();
        let next_val:XdrResult<u32> = x.unpack_primitive();
        assert!(v.is_err() == false);
        assert!(v.unwrap() == "RUST".to_owned());
        assert!(next_val.is_err() == false);
        assert!(next_val.unwrap() == 25);
    }

    #[test]
    fn unpack_string_len_is_not_multiple_of_four() {
        let buffer = ~[0u8,0u8,0u8,5u8,82u8,85u8,83u8,84u8,89u8,0u8,0u8,0u8,0u8,0u8,0u8,25u8];
        let mut x = Xdr::new(buffer);
        let v = x.unpack_string();
        let next_val:XdrResult<u32> = x.unpack_primitive();
        assert!(v.is_err() == false);
        assert!(v.unwrap() == "RUSTY".to_owned());
        assert!(next_val.is_err() == false);
        assert!(next_val.unwrap() == 25);
    }

    #[test]
    fn unpack_string_len_is_too_long() {
        let buffer = ~[0u8,0u8,0u8,45u8,82u8,85u8,83u8,84u8,89u8,0u8];
        let mut x = Xdr::new(buffer);
        let v = x.unpack_string();
        assert!(v.is_err());
    }

    #[test]
    fn unpack_array_correct_size() {
        let buffer = ~[0u8,0u8,0u8,1u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,3u8];
        let mut x = Xdr::new(buffer);
        let v:XdrResult<~[u32]> = x.unpack_array(3);
        assert!(v.is_err() == false);
        let array = v.unwrap();
        assert!(array == ~[1u32,2u32,3u32])
    }

    #[test]
    fn unpack_array_incorrect_size() {
        let buffer = ~[0u8,0u8,0u8,1u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,3u8];
        let mut x = Xdr::new(buffer);
        let v:XdrResult<~[u32]> = x.unpack_array(4);
        assert!(v.is_err());
    }

    #[test]
    fn unpack_varlen_array_correct_size() {
        let buffer = ~[0u8,0u8,0u8,3u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,3u8];
        let mut x = Xdr::new(buffer);
        let v:XdrResult<~[u32]> = x.unpack_varlen_array();
        assert!(v.is_err() == false);
        let array = v.unwrap();
        assert!(array == ~[1u32,2u32,3u32])
    }

    #[test]
    fn unpack_varlen_array_incorrect_size() {
        let buffer = ~[0u8,0u8,0u8,10u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,3u8];
        let mut x = Xdr::new(buffer);
        let v:XdrResult<~[u32]> = x.unpack_varlen_array();
        assert!(v.is_err());
    }
}
