#[cfg(test)]
mod tests {
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
    fn test_unpack_uhyperint() {
        let buffer = ~[0u8,0u8,0u8,0u8,0u8,0u8,0u8,128u8,23u8,0u8,0u8];
        let mut x = xdr::Xdr::new(buffer);
        let v = x.unpack_uhyperint();
        assert!(v.is_none() == false);
        assert!(v.unwrap() == 128);
    }
    
    #[test]
    fn test_unpack_hyperint_positive() {
        let buffer = ~[0u8,0u8,0u8,0u8,0u8,0u8,0u8,246u8,23u8,0u8,0u8];
        let mut x = xdr::Xdr::new(buffer);
        let v = x.unpack_hyperint();
        assert!(v.is_none() == false);
        assert!(v.unwrap() == 246);
    }

    #[test]
    fn test_unpack_hyperint_negative() {
        let buffer = ~[255u8,255u8,255u8,255u8,255u8,255u8,255u8,231u8,23u8,0u8,0u8];
        let mut x = xdr::Xdr::new(buffer);
        let v = x.unpack_hyperint();
        assert!(v.is_none() == false);
        assert!(v.unwrap() == -25);
    }

    #[test]
    fn test_unpack_bool_true_value() {
        let buffer = ~[0u8,0u8,0u8,1u8,23u8,0u8,0u8];
        let mut x = xdr::Xdr::new(buffer);
        let v = x.unpack_bool();
        assert!(v.is_none() == false);
        assert!(v.unwrap() == true);
    }

    #[test]
    fn test_unpack_bool_false_value() {
        let buffer = ~[0u8,0u8,0u8,0u8,23u8,0u8,0u8];
        let mut x = xdr::Xdr::new(buffer);
        let v = x.unpack_bool();
        assert!(v.is_none() == false);
        assert!(v.unwrap() == false);
    }

    #[test]
    fn test_unpack_bool_invalid_value() {
        let buffer = ~[0u8,0u8,0u8,20u8,23u8,0u8,0u8];
        let mut x = xdr::Xdr::new(buffer);
        let v = x.unpack_bool();
        assert!(v.is_none());
    }

    #[test]
    fn test_unpack_fixed_bytes_unpadded() {
        let buffer = ~[1u8,2u8,3u8,4u8,0u8,0u8,0u8,25u8];
        let mut x = xdr::Xdr::new(buffer);
        let v = x.unpack_fixed_bytes(4);
        let next_val = x.unpack_uint();

        assert!(v.is_none() == false);
        let bytes = v.unwrap();
        assert!(bytes.len() == 4);
        assert!(bytes == ~[1u8,2u8,3u8,4u8]);

        assert!(next_val.is_none() == false);
        assert!(next_val.unwrap() == 25);
    }

    #[test]
    fn test_unpack_fixed_bytes_padded() {
        let buffer = ~[1u8,2u8,3u8,4u8,5u8,6u8,0u8,0u8,0u8,0u8,0u8,25u8];
        let mut x = xdr::Xdr::new(buffer);
        let v = x.unpack_fixed_bytes(6);
        let next_val = x.unpack_uint();

        assert!(v.is_none() == false);
        let bytes = v.unwrap();
        assert!(bytes.len() == 6);
        assert!(bytes == ~[1u8,2u8,3u8,4u8,5u8,6u8]);

        assert!(next_val.is_none() == false);
        assert!(next_val.unwrap() == 25);
    }

    #[test]
    fn test_unpack_fixed_bytes_padded_len_too_long() {
        let buffer = ~[1u8,2u8,3u8,4u8,5u8,6u8,7u8];
        let mut x = xdr::Xdr::new(buffer);
        let v = x.unpack_fixed_bytes(7);

        assert!(v.is_none());
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
