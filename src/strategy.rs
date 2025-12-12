use crate::player::History;
use rand::Rng;
use std::collections::HashMap;

/// General strategy trait:
/// `history` = joint history of play as (my_action, opp_action) per past round.
pub trait Strategy {
    fn select_action(&mut self, history: &History, rng: &mut rand::rngs::ThreadRng) -> String;
}

/// Classic Tit-for-Tat:
/// - First round: Cooperate
/// - Thereafter: Play the opponent's last action
pub struct TitForTat;

pub struct AlwaysCooperate;

pub struct RandomStrategy {
    pub p_cooperate: f64, // probability to cooperate
}

/// Wrapper strategy that, with probability `epsilon`, flips the chosen action (C <-> D).
pub struct EpsilonMistake {
    pub epsilon: f64,
    pub inner: Box<dyn Strategy>,
}

pub type ProbabilityDistribution = HashMap<String, f64>;

/// Joint memory-1 lookup strategy:
/// - Key: state string like "CC", "CD", "DC", "DD" = (my_last, opp_last)
/// - Value: distribution over actions {"C": p, "D": 1-p}
pub struct LookupStrategy {
    pub rules: HashMap<String, ProbabilityDistribution>,
}

impl Strategy for LookupStrategy {
    fn select_action(&mut self, history: &History, rng: &mut rand::rngs::ThreadRng) -> String {
        // state: last (my, opp) pair encoded as "MO", e.g., "CD"
        let state_key = if let Some((my_last, opp_last)) = history.last() {
            format!("{}{}", my_last, opp_last)
        } else {
            // no history yet: default to C
            return "C".to_string();
        };

        if let Some(dist) = self.rules.get(&state_key) {
            let r: f64 = rng.random::<f64>();
            let mut acc = 0.0;
            for (action, p) in dist {
                acc += p;
                if r <= acc {
                    return action.clone();
                }
            }
        }
        // fallback
        "C".to_string()
    }
}

impl Strategy for TitForTat {
    fn select_action(&mut self, history: &History, _: &mut rand::rngs::ThreadRng) -> String {
        if history.is_empty() {
            "C".into()
        } else {
            let (_my_last, opp_last) = history.last().unwrap();
            opp_last.clone()
        }
    }
}

impl Strategy for AlwaysCooperate {
    fn select_action(&mut self, _: &History, _: &mut rand::rngs::ThreadRng) -> String {
        "C".into()
    }
}

impl Strategy for RandomStrategy {
    fn select_action(&mut self, _: &History, rng: &mut rand::rngs::ThreadRng) -> String {
        if rng.random::<f64>() <= self.p_cooperate {
            "C".into()
        } else {
            "D".into()
        }
    }
}

impl EpsilonMistake {
    fn flip_action(action: &str) -> String {
        match action {
            "C" => "D".to_string(),
            "D" => "C".to_string(),
            _ => action.to_string(), // unknown actions are left unchanged
        }
    }
}

impl Strategy for EpsilonMistake {
    fn select_action(&mut self, history: &History, rng: &mut rand::rngs::ThreadRng) -> String {
        // First, let the inner strategy choose an action
        let base_action = self.inner.select_action(history, rng);

        // Then, with probability epsilon, flip it
        let r: f64 = rng.random::<f64>();
        if r < self.epsilon {
            EpsilonMistake::flip_action(&base_action)
        } else {
            base_action
        }
    }
}

/// Helper to build rules from probabilities of cooperation after (CC, CD, DC, DD).
pub fn build_joint_memory1_rules(
    p_cc: f64,
    p_cd: f64,
    p_dc: f64,
    p_dd: f64,
) -> HashMap<String, ProbabilityDistribution> {
    let mut rules = HashMap::new();

    rules.insert(
        "CC".to_string(),
        HashMap::from([("C".to_string(), p_cc), ("D".to_string(), 1.0 - p_cc)]),
    );
    rules.insert(
        "CD".to_string(),
        HashMap::from([("C".to_string(), p_cd), ("D".to_string(), 1.0 - p_cd)]),
    );
    rules.insert(
        "DC".to_string(),
        HashMap::from([("C".to_string(), p_dc), ("D".to_string(), 1.0 - p_dc)]),
    );
    rules.insert(
        "DD".to_string(),
        HashMap::from([("C".to_string(), p_dd), ("D".to_string(), 1.0 - p_dd)]),
    );

    rules
}
