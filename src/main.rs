mod player;
mod strategy;
mod utils;

use std::collections::HashMap;

use crate::player::{Game, Player};
use crate::strategy::{
    EpsilonMistake, LookupStrategy, ProbabilityDistribution, RandomStrategy, Strategy, TitForTat,
    build_joint_memory1_rules,
};
use crate::utils::write_log_to_csv;
use itertools::iproduct;
use ndarray::Array1;

struct Config {
    n: usize,
    rounds: usize,
    rules: HashMap<String, ProbabilityDistribution>,
    epsilon: f64,
    payoff_matrix: Vec<Vec<i32>>,
    trial: usize,
}

fn play_game(c: Config) {
    let mut rng = rand::thread_rng();

    let mut players = Vec::<Player>::with_capacity(c.n);
    for ni in 0..c.n {
        players.push(Player {
            name: format!("Player {}", ni + 1),
            // LookupStrategy with epsilon mistakes
            strategy: Box::new(EpsilonMistake {
                epsilon: c.epsilon,
                inner: Box::new(LookupStrategy {
                    rules: c.rules.clone(),
                }),
            }),
            history: Vec::new(),
            action: "C".into(),
        });
    }

    let mut game = Game {
        trial: c.trial,
        players: players,
        payoff_matrix: c.payoff_matrix.clone(),
        rng: rng,
        epsilon: c.epsilon,
    };
    let (results, log) = game.simulate(c.rounds);

    // println!("Results over {} rounds:", c.rounds);
    // for (i, round) in results.iter().enumerate() {
    //     println!("Round {}: {:?}", i + 1, round);
    // }

    let file_name = format!("data/game_log_trial_{}_{:0.5}.csv", c.trial, c.epsilon);
    write_log_to_csv(&file_name, &log).expect("failed to write CSV");
}

fn main() {
    // Example joint memory-1 rules for Player 2:
    // P(C | CC), P(C | CD), P(C | DC), P(C | DD)

    let trials = 1000;
    let epsilons = Array1::linspace(0.0, 0.1, 5);
    for (trial, epsilon) in iproduct!(0..trials, epsilons.iter().copied()) {
        let p_cc = 1.0 - epsilon;
        let p_cd = epsilon;
        let p_dc = 1.0 - epsilon;
        let p_dd = epsilon;
        let rules = build_joint_memory1_rules(p_cc, p_cd, p_dc, p_dd);

        let config = Config {
            n: 2,
            rounds: 50,
            rules: rules.clone(),
            epsilon,
            payoff_matrix: vec![vec![3, 0], vec![5, 1]],
            trial,
        };
        play_game(config);
    }
}
