#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Complexity { Const, Log, Linear, NLogN, Quadratic, Cubic, Exp }

pub fn class_of(name: &str) -> Complexity {
    match name {
        "sha256" => Complexity::Linear,
        "merge_sort" => Complexity::NLogN,
        "binary_search" => Complexity::Log,
        "linear_search" => Complexity::Linear,
        "btreemap_insert" => Complexity::Log,
        "btreemap_get" => Complexity::Log,
        _ => Complexity::Const,
    }
}
