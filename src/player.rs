use crate::strategy::Strategy;
use crate::utils::RoundRecord;

/// History of play for a player: one (my_action, opp_action) pair per past round.
pub type History = Vec<(String, String)>;

pub struct Player {
    pub name: String,
    pub strategy: Box<dyn Strategy>,
    /// Joint history: (my_action, opp_action) for each previous round
    pub history: History,
    /// Current action this round
    pub action: String,
}

pub struct Game {
    pub trial: usize,
    pub players: Vec<Player>,
    /// payoff_matrix[a_idx][b_idx] where 0 = C, 1 = D
    pub payoff_matrix: Vec<Vec<i32>>,
    pub rng: rand::rngs::ThreadRng,
    pub epsilon: f64,
}

impl Game {
    /// simulate t rounds, returning payoff vector per round AND a flat log of records
    pub fn simulate(&mut self, t: usize) -> (Vec<Vec<i32>>, Vec<RoundRecord>) {
        let mut results = vec![vec![0; self.players.len()]; t];
        let mut log: Vec<RoundRecord> = Vec::with_capacity(t * self.players.len());

        println!("{}", self.epsilon);

        for round in 0..t {
            let payoffs = self.interact();
            results[round] = payoffs.clone();

            // log one record per player
            for (i, player) in self.players.iter().enumerate() {
                log.push(RoundRecord {
                    trial: self.trial,
                    round: round + 1, // 1-based
                    player: player.name.clone(),
                    action: player.action.clone(),
                    payoff: payoffs[i],
                    epsilon: self.epsilon,
                });
            }
        }

        (results, log)
    }

    /// One round of interaction (2 players assumed)
    pub fn interact(&mut self) -> Vec<i32> {
        // 1. Snapshot histories for decision-making
        let histories: Vec<History> = self.players.iter().map(|p| p.history.clone()).collect();

        // 2. Each player chooses a new action based on the joint history
        let mut new_actions: Vec<String> = Vec::with_capacity(self.players.len());
        for i in 0..self.players.len() {
            let action = self.players[i]
                .strategy
                .select_action(&histories[i], &mut self.rng);
            new_actions.push(action);
        }

        // 3. Commit new actions and update histories with (my, opp) for this round
        for i in 0..self.players.len() {
            let opp = 1 - i; // assumes 2 players
            let my_a = new_actions[i].clone();
            let opp_a = new_actions[opp].clone();
            self.players[i].action = my_a.clone();
            self.players[i].history.push((my_a, opp_a));
        }

        // 4. Compute payoffs (2-player Prisoner's Dilemma)
        let idx0 = if new_actions[0] == "C" { 0 } else { 1 };
        let idx1 = if new_actions[1] == "C" { 0 } else { 1 };

        vec![
            self.payoff_matrix[idx0][idx1],
            self.payoff_matrix[idx1][idx0],
        ]
    }
}
