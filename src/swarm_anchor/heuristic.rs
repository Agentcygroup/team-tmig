#[derive(Clone, Debug)]
pub struct Heuristic { pub id: String, pub label: String }
pub fn all_heuristics() -> Vec<Heuristic> {
    ["hill-climb", "simulated-annealing", "tabu", "genetic", "pso", "aco", "mcts", "ucb1", "thompson", "sgd", "adam", "pca", "ica", "entropy", "kl"]
        .iter().enumerate().map(|(i, k)| Heuristic { id: format!("{}.{}", k, i), label: format!("{} h", k) }).collect()
}
