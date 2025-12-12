use std::error::Error;
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
pub struct RoundRecord {
    pub trial: usize,
    pub round: usize,
    pub player: String,
    pub action: String,
    pub payoff: i32,
    pub epsilon: f64,
}

pub fn write_log_to_csv(path: &str, log: &[RoundRecord]) -> Result<(), Box<dyn Error>> {
    let file = File::create(path)?;
    let mut wtr = csv::Writer::from_writer(file);

    // header will be inferred from struct field names if we use serialize,
    // but let's write manually for explicit control:
    wtr.write_record(&["trial", "round", "player", "action", "payoff", "epsilon"])?;

    for rec in log {
        wtr.write_record(&[
            rec.trial.to_string(),
            rec.round.to_string(),
            rec.player.clone(),
            rec.action.clone(),
            rec.payoff.to_string(),
            rec.epsilon.to_string(),
        ])?;
    }

    wtr.flush()?;
    Ok(())
}
