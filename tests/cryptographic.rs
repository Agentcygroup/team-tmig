use tmig::boundary::cryptographic::{classify, CryptoBound};

#[test]
fn secure_marginal_broken() {
    assert_eq!(classify(256, true), CryptoBound::Secure);
    assert_eq!(classify(128, false), CryptoBound::Marginal);
    assert_eq!(classify(64, false), CryptoBound::Broken);
    assert_eq!(classify(0, false), CryptoBound::Unknown);
}
