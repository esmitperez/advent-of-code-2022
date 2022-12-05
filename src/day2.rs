#[derive(PartialEq, Eq, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Eq, Debug)]
enum RoundResult {
    Lose,
    Win,
    Draw,
    Undetermined,
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    shape: Shape,
    value: u8,
}

trait Playable {
    fn play(&mut self);
}

#[derive(PartialEq, Eq, Debug)]
pub struct Round {
    source: String,
    mine: Hand,
    theirs: Hand,
    result: RoundResult,
    points: u8,
}

impl Playable for Round {
    fn play(&mut self) {
        let result = determine_round_result(self);
        self.result = result;

        let points = calculate_points(self);
        self.points = points;
    }
}

fn parse_round(source: String) -> Round {
    use RoundResult::*;

    let hand_shapes: Vec<&str> = source.split(" ").collect();

    let theirs = determine_shape(hand_shapes[0]);
    let mine = determine_shape(hand_shapes[1]);

    Round {
        source,
        mine,
        theirs,
        result: Undetermined,
        points: 0,
    }
}

fn determine_shape(letter: &str) -> Hand {
    use Shape::*;
    return match letter {
        "A" | "X" => Hand {
            shape: Rock,
            value: 1,
        },
        "B" | "Y" => Hand {
            shape: Paper,
            value: 2,
        },
        "C" | "Z" => Hand {
            shape: Scissors,
            value: 3,
        },
        _ => panic!("All letters expected to match"),
    };
}

fn calculate_points(round: &Round) -> u8 {
    use RoundResult::*;

    match round.result {
        Win => 6 + round.mine.value,
        Draw => 3 + round.mine.value,
        Lose => round.mine.value,
        Undetermined => panic!("Should never reach here. Round hasn't been played and is Undetermined"),
    }
}


fn determine_round_result(round: &Round) -> RoundResult {
    use RoundResult::*;
    use Shape::*;

    let mine = &round.mine.shape;
    let theirs = &round.theirs.shape;

    match mine {
        Rock => {
            return match theirs {
                Paper => Lose,
                Scissors => Win,
                Rock => Draw,
            }
        }
        Paper => {
            return match theirs {
                Paper => Draw,
                Scissors => Lose,
                Rock => Win,
            }
        }
        Scissors => {
            return match theirs {
                Paper => Win,
                Scissors => Draw,
                Rock => Lose,
            }
        }
    };
}

pub fn parse(contents: String) -> Vec<Round> {
    let mut rounds = Vec::new();

    let lines = contents.split("\n");
    for l in lines {
        let round = parse_round(l.to_string());
        rounds.push(round);
    }

    rounds
}

pub fn play_tournament(rounds: &mut Vec<Round> ) -> u16{
    
    // find how to .map this
    let points = rounds.iter_mut().fold(0,|acc, r| {
        r.play();
        dbg!(&r);
        acc + u16::from(r.points)
    });

    println!("Points {:?}", points);

    u16::from(points)
}

#[cfg(test)]
mod tests {
    use super::RoundResult::*;
    use super::Shape::*;
    use super::*;
    const EXPECTED: &str = "A Y\nB X\nC Z";

    #[test]
    fn determine_shape_works() {
        let result = determine_shape("A");
        dbg!(&result);
        assert_eq!(
            result,
            Hand {
                shape: Rock,
                value: 1
            }
        );

        let result = determine_shape("X");
        dbg!(&result);
        assert_eq!(
            result,
            Hand {
                shape: Rock,
                value: 1
            }
        );

        let result = determine_shape("C");
        dbg!(&result);
        assert_eq!(
            result,
            Hand {
                shape: Scissors,
                value: 3
            }
        );
    }

    #[test]
    fn parse_round_works() {
        let result = parse_round("A X".to_owned());
        dbg!(&result);
        assert_eq!(result.mine.shape, Rock);
        assert_eq!(result.theirs.shape, Rock);
    }

    #[test]
    fn determine_round_result_works() {
        let round = parse_round("A X".to_owned());
        let result = determine_round_result(&round);
        dbg!(&result);
        assert_eq!(result, Draw);

        let round = parse_round("A Y".to_owned());
        let result = determine_round_result(&round);
        dbg!(&result);
        assert_eq!(result, Win);

        let round = parse_round("A Z".to_owned());
        let result = determine_round_result(&round);
        dbg!(&result);
        assert_eq!(result, Lose);

        let round = parse_round("B X".to_owned());
        let result = determine_round_result(&round);
        dbg!(&result);
        assert_eq!(result, Lose);
    }

    #[test]
    fn calculate_points_works() {
        let mut round = parse_round("A Y".to_owned());
        round.play();

        let result = round.points;
        dbg!(&result);
        assert_eq!(result, 8);

        let mut round = parse_round("B X".to_owned());
        round.play();

        let result = round.points;
        dbg!(&result);
        assert_eq!(result, 1);

        let mut round = parse_round("C Z".to_owned());
        round.play();

        let result = round.points;
        dbg!(&result);
        assert_eq!(result, 6);
    }

    #[test]
    fn parse_works(){
        let result = parse(EXPECTED.to_owned());
        dbg!(&result);
        assert_eq!(result.len(), 3)
    }

    #[test]
    fn play_tournament_works(){
        let tournament_total = play_tournament(&mut parse(EXPECTED.to_owned()));
        dbg!(&tournament_total);
        assert_eq!(tournament_total, 15)
    }
}
