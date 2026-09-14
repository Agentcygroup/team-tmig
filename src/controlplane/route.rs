use crate::hitl::consequence::Consequence;

pub fn route(c: Consequence) -> (u8, u8) {
    match c {
        Consequence::None => (0, 0),
        Consequence::Low => (0, 1),
        Consequence::Moderate => (0, 2),
        Consequence::High => (0, 3),
        Consequence::Legal => (0, 4),
        Consequence::Irreversible => (1, 4),
    }
}
