use crate::content::registry::ContentKind;

pub fn classify(n: &str, ext: &str) -> ContentKind {
    let n = n.to_lowercase();
    let ext = ext.to_lowercase();
    if n.contains("sig") || ext == "sig" {
        return ContentKind::Sig;
    }
    if n.contains("key") || ext == "key" {
        return ContentKind::Key;
    }
    if n.contains("cert") || ext == "crt" {
        return ContentKind::Cert;
    }
    if n.contains("sbom") {
        return ContentKind::Sbom;
    }
    if ext == "asm" || ext == "s" {
        return ContentKind::Asm;
    }
    ContentKind::Unknown
}
