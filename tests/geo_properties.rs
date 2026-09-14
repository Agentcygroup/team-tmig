use proptest::prelude::*;
use tmig::geo::area_code::AreaCode;
use tmig::geo::boundary::Boundary11;
use tmig::geo::zip::Zip;

proptest! {
    #[test]
    fn zip_roundtrip(z in 1000u32..=99950u32, p in 0u32..=9999u32) {
        let zc = Zip::new(z).unwrap().with_plus4(p).unwrap();
        prop_assert_eq!(zc.zip5, z);
        prop_assert_eq!(zc.plus4, Some(p));
    }

    #[test]
    fn area_code_nxx(n in 200u16..=999u16) {
        prop_assert!(AreaCode::new(n).is_some());
    }

    #[test]
    fn boundary_roundtrip(s in 1u8..=78u8, c in 0u16..=999u16, t in 0u32..=999999u32) {
        if let Some(b) = Boundary11::new(s, c, t) {
            let str = b.as_string();
            prop_assert_eq!(Boundary11::parse(&str), Some(b));
        }
    }
}
