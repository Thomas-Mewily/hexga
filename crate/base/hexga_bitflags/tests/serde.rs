#[cfg(test)]
mod tests
{
    use hexga_bitflags::*;
    use postcard;
    use serde::{Deserialize, Serialize};
    use serde_json;

    #[bit_index]
    #[repr(u8)]
    enum TestFlags
    {
        A,
        B,
        C = 2,
        E = TestFlags::A | Self::B,
    }

    #[test]
    fn test_json_serialization()
    {
        let flags = TestFlags::A | TestFlags::C;

        // Test JSON serialization (text format)
        let json = serde_json::to_string(&flags).unwrap();
        println!("JSON: {}", json);

        // Test JSON deserialization
        let deserialized: TestFlagsFlags = serde_json::from_str(&json).unwrap();
        assert_eq!(flags, deserialized);
        assert_eq!(json, r#"{"a":true,"b":false,"c":true}"#)
    }

    #[test]
    fn test_binary_serialization()
    {
        let flags = TestFlags::A | TestFlags::C;

        // Test binary serialization
        let binary = postcard::to_stdvec(&flags).unwrap();
        println!("Binary: {:?}", binary);

        // Test binary deserialization
        let deserialized: TestFlagsFlags = postcard::from_bytes(&binary).unwrap();
        assert_eq!(flags, deserialized);
    }

    #[test]
    fn test_empty_flags()
    {
        let flags = TestFlagsFlags::EMPTY;

        // Test JSON
        let json = serde_json::to_string(&flags).unwrap();
        let deserialized: TestFlagsFlags = serde_json::from_str(&json).unwrap();
        assert_eq!(flags, deserialized);

        // Test binary
        let binary = postcard::to_stdvec(&flags).unwrap();
        let deserialized: TestFlagsFlags = postcard::from_bytes(&binary).unwrap();
        assert_eq!(flags, deserialized);
    }

    #[test]
    fn test_single_flag()
    {
        let flags = TestFlags::B.flags();

        // Test JSON
        let json = serde_json::to_string(&flags).unwrap();
        let deserialized: TestFlagsFlags = serde_json::from_str(&json).unwrap();
        assert_eq!(flags, deserialized);

        // Test binary
        let binary = postcard::to_stdvec(&flags).unwrap();
        let deserialized: TestFlagsFlags = postcard::from_bytes(&binary).unwrap();
        println!("Bin {:?}", binary);
        assert_eq!(flags, deserialized);
        assert_eq!(binary, &[2]);
    }
}
