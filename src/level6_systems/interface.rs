#[derive(Clone, Debug)]
pub struct Interface {
    pub name: String,
    pub producer: String,
    pub consumer: String,
    pub payload: String,
}

pub fn all_interfaces() -> Vec<Interface> {
    vec![
        Interface {
            name: "controlplane::handle".into(),
            producer: "controlplane".into(),
            consumer: "hitl".into(),
            payload: "ActionProof -> Attestation".into(),
        },
        Interface {
            name: "router::route".into(),
            producer: "router".into(),
            consumer: "collective".into(),
            payload: "Consequence -> (RouteTarget, Attestation)".into(),
        },
    ]
}
