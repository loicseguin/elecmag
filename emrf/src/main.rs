use std::env;
use std::io;

#[derive(Debug)]
enum Level {
    E, // Excellent
    S, // Satisfaisant
    R, // pRogression
    F, // Fragmentaire
}

impl Level {
    fn score(&self) -> f32 {
        match self {
            Level::E => 1.00,
            Level::S => 0.75,
            Level::R => 0.40,
            Level::F => 0.00,
        }
    }
}

/// Characters are associated with levels, association is case insensitive. Some synonyms are
/// recognized:
///   - E => Excellent
///   - S, M => Satisfactory (Meets expectation in original paper)
///   - R, P => in Progression (Revision needed in original paper)
///   - F => Fragmentary
/// If the character is not recognized, return an Err.
fn char_to_level(c: &char) -> Result<Level, String> {
    match c.to_ascii_uppercase() {
        'E' => Ok(Level::E),
        'S' => Ok(Level::S),
        'M' => Ok(Level::S),
        'R' => Ok(Level::R),
        'P' => Ok(Level::R),
        'F' => Ok(Level::F),
        _ => Err(format!("incorrect level: {}", c)),
    }
}

/// For a given set of criteria with weights, compute the total score for a given evaluation.
///
/// For instance, an evaluation of ERSS for a set of four criteria with weights 0.2, 0.3, 0.4, 0.1
/// would lead to a total score of 0.2 * E.score() + 0.3 * R.score() + 0.4 * S.score() + 0.1 *
/// S.score().
fn levels_to_score(eval: &[Level], weights: &Vec<f32>) -> f32 {
    eval.iter().zip(weights).map(|(x, y)| x.score() * y).sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let max_score: f32 = if args.len() >= 2 {
        args[1]
            .parse()
            .expect("Max score should be a floating point number")
    } else {
        100.0
    };

    let mut evaluation = String::new();
    let weights = vec![0.2, 0.3, 0.3, 0.2];

    while io::stdin()
        .read_line(&mut evaluation)
        .expect("Failed to read line")
        != 0
    {
        // Use take to get rid of the trailing newline character
        let evaluation: Result<Vec<Level>, String> = std::mem::take(&mut evaluation)
            .trim()
            .chars()
            .map(|x| char_to_level(&x))
            .collect();
        match evaluation {
            Ok(evaluation) => {
                if evaluation.len() != 4 {
                    eprintln!(
                        "incorrect number of levels {} (should be 4)",
                        evaluation.len()
                    );
                } else {
                    println!(
                        "{:?}: {:?}",
                        evaluation,
                        max_score * levels_to_score(&evaluation, &weights)
                    );
                }
            }
            Err(evaluation) => eprintln!("{evaluation}"),
        }
    }
}
