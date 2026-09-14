use crate::content::index::ContentIndex;

pub struct Render;

impl Render {
    pub fn tsv(idx: &ContentIndex) -> String {
        let mut out = String::from("kind\tcount\n");
        for (k, v) in &idx.by_kind { out.push_str(&format!("{:?}\t{}\n", k, v.len())); }
        out.push_str(&format!("TOTAL\t{}\n", idx.len()));
        out
    }
    pub fn json(idx: &ContentIndex) -> String {
        let items: Vec<String> = idx.by_kind.iter()
            .map(|(k, v)| format!("\"{:?}\":{}", k, v.len()))
            .collect();
        format!("{{\"kinds\":{{{}}},\"total\":{},\"bytes\":{}}}",
            items.join(","), idx.len(), idx.total_bytes)
    }
}
