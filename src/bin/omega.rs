use tmig::payload::{verify_all, Pack, Registry};
use tmig::resilience::{Capacity, Quorum, Recovery};

fn main() {
    let r = Registry::omega();
    let required = r.required().len();
    let optional = r.optional().len();
    let (pass, fail) = verify_all(&r, b"seed-omega");
    let pack = Pack::from_registry(&r, b"seed-omega");
    let cap = Capacity::omega();
    let quo = Quorum::new(9);
    let rec = Recovery::omega();

    println!("payload_total\t{}", r.len());
    println!("payload_required\t{}", required);
    println!("payload_optional\t{}", optional);
    println!("payload_verify\t{}_pass_{}_fail", pass, fail);
    println!("pack_entries\t{}", pack.len());
    println!("pack_bytes\t{}", pack.total_bytes());
    println!("capacity_replicas\t{}", cap.replicas);
    println!("capacity_shards\t{}", cap.shards);
    println!("capacity_zones\t{}", cap.geographic_zones);
    println!("capacity_domains\t{}", cap.failure_domains);
    println!("capacity_survives\t{}", cap.can_survive(8));
    println!("quorum_members\t{}", quo.members);
    println!("quorum_required\t{}", quo.required);
    println!("quorum_tolerates\t{}", quo.tolerates());
    println!("rto_seconds\t{}", rec.rto_seconds);
    println!("rpo_seconds\t{}", rec.rpo_seconds);
    println!("auto_recovery\t{}", rec.auto);
}
