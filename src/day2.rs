use std::marker::Copy;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Shape {
    Hidden,
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
    fn play(&mut self) -> &mut Round;
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum ParsingStrategyType {
    DefaultStrategy,
    CorrectStrategy,
}

trait DeterminesHand {
    fn determine_hand(&self, letter: &str, shape: Shape) -> Hand;
}

trait ParsingStrategy {
    // fn determine_hand(&self, letter: &str) -> Hand;
}

#[derive(PartialEq, Eq, Debug)]
struct DefaultParsingStrategy {}

#[derive(PartialEq, Eq, Debug)]
struct CorrectParsingStrategy {}

impl DeterminesHand for DefaultParsingStrategy {
    fn determine_hand(&self, letter: &str, shape: Shape) -> Hand {
        println!("Running using DEFAULT strategy");

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
}
impl DeterminesHand for CorrectParsingStrategy {
    fn determine_hand(&self, letter: &str, shape: Shape) -> Hand {
        use Shape::*;

        println!("Running using CORRECT strategy");
        match letter {
            "X" => {
                // I lose to theirs
                match shape {
                    Rock => Hand {
                        shape: Scissors,
                        value: 3,
                    },
                    Paper => Hand {
                        shape: Rock,
                        value: 1,
                    },
                    Scissors => Hand {
                        shape: Paper,
                        value: 2,
                    },
                    Hidden => panic!("Should not be Hidden"),
                }
            }
            "Y" => {
                // draw
                match shape {
                    Rock => Hand {
                        shape: Rock,
                        value: 1,
                    },
                    Paper => Hand {
                        shape: Paper,
                        value: 2,
                    },
                    Scissors => Hand {
                        shape: Scissors,
                        value: 3,
                    },
                    Hidden => panic!("Should not be Hidden"),
                }
            }
            "Z" => {
                // I win to theirs 
                match shape {
                    Rock => Hand {
                        shape: Paper,
                        value: 2,
                    },
                    Paper => Hand {
                        shape: Scissors,
                        value: 3,
                    },
                    Scissors => Hand {
                        shape: Rock,
                        value: 1,
                    },
                    Hidden => panic!("Should not be Hidden"),
                }
            }
            _ => Hand {
                shape: Shape::Hidden,
                value: 0,
            }, // todo
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Round {
    source: String,
    strategy: ParsingStrategyType,
    mine: Hand,
    theirs: Hand,
    result: RoundResult,
    points: u8,
}

impl Playable for Round {
    fn play(&mut self) -> &mut Round {
        let result = determine_round_result(self);
        self.result = result;

        let points = calculate_points(self);
        self.points = points;

        self
    }
}

trait Determinable {
    fn determine_shapes(&mut self) -> &mut Round;
}

impl Determinable for Round {
    fn determine_shapes(&mut self) -> &mut Round {
        use ParsingStrategyType::*;

        let hand_shapes: Vec<&str> = self.source.split(" ").collect();

        self.theirs = xdetermine_hand(hand_shapes[0]);
        self.mine = match self.strategy {
            DefaultStrategy => {
                DefaultParsingStrategy{}.determine_hand(hand_shapes[1], self.theirs.shape)
            }
            CorrectStrategy => {
                CorrectParsingStrategy{}.determine_hand(hand_shapes[1], self.theirs.shape)
            }
        };

        self
    }
}

fn parse_round(source: String, parsing_strategy: ParsingStrategyType) -> Round {
    use RoundResult::*;
    use Shape::*;

    let mut r = Round {
        source,
        strategy: parsing_strategy,
        mine: Hand {
            shape: Hidden,
            value: 0,
        },
        theirs: Hand {
            shape: Hidden,
            value: 0,
        },
        result: Undetermined,
        points: 0,
    };

    r.determine_shapes();

    r
}

fn xdetermine_hand(letter: &str) -> Hand {
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
        Undetermined => {
            panic!("Should never reach here. Round hasn't been played and is Undetermined")
        }
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
                Hidden => Undetermined,
            }
        }
        Paper => {
            return match theirs {
                Paper => Draw,
                Scissors => Lose,
                Rock => Win,
                Hidden => Undetermined,
            }
        }
        Scissors => {
            return match theirs {
                Paper => Win,
                Scissors => Draw,
                Rock => Lose,
                Hidden => Undetermined,
            }
        }
        Hidden => Undetermined,
    }
}

pub fn parse(contents: String, parsing_type: ParsingStrategyType) -> Vec<Round> {
    use ParsingStrategyType::*;

    let mut rounds = Vec::new();

    let lines = contents.split("\n");
    for l in lines {
        let round = parse_round(l.to_string(), parsing_type);
        rounds.push(round);
    }

    rounds
}

pub fn play_tournament(rounds: &mut Vec<Round>) -> u16 {
    // find how to .map this
    let points = rounds.iter_mut().fold(0, |acc, r| {
        r.play();
        // dbg!(&r);
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
        let result = DefaultParsingStrategy {}.determine_hand("A", Shape::Hidden);
        dbg!(&result);
        assert_eq!(
            result,
            Hand {
                shape: Rock,
                value: 1
            }
        );

        let result = DefaultParsingStrategy {}.determine_hand("X", Shape::Hidden);
        dbg!(&result);
        assert_eq!(
            result,
            Hand {
                shape: Rock,
                value: 1
            }
        );

        let result = DefaultParsingStrategy {}.determine_hand("C", Shape::Hidden);
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
        let result = parse_round("A X".to_owned(), ParsingStrategyType::DefaultStrategy);
        dbg!(&result);
        assert_eq!(result.mine.shape, Rock);
        assert_eq!(result.theirs.shape, Rock);
    }

    #[test]
    fn determine_round_result_works() {
        let round = parse_round("A X".to_owned(), ParsingStrategyType::DefaultStrategy);
        let result = determine_round_result(&round);
        dbg!(&result);
        assert_eq!(result, Draw);

        let round = parse_round("A Y".to_owned(), ParsingStrategyType::DefaultStrategy);
        let result = determine_round_result(&round);
        dbg!(&result);
        assert_eq!(result, Win);

        let round = parse_round("A Z".to_owned(), ParsingStrategyType::DefaultStrategy);
        let result = determine_round_result(&round);
        dbg!(&result);
        assert_eq!(result, Lose);

        let round = parse_round("B X".to_owned(), ParsingStrategyType::DefaultStrategy);
        let result = determine_round_result(&round);
        dbg!(&result);
        assert_eq!(result, Lose);
    }

    #[test]
    fn calculate_points_works() {
        let mut round = parse_round("A Y".to_owned(), ParsingStrategyType::DefaultStrategy);
        round.play();

        let result = round.points;
        dbg!(&result);
        assert_eq!(result, 8);

        let mut round = parse_round("B X".to_owned(), ParsingStrategyType::DefaultStrategy);
        round.play();

        let result = round.points;
        dbg!(&result);
        assert_eq!(result, 1);

        let mut round = parse_round("C Z".to_owned(), ParsingStrategyType::DefaultStrategy);
        round.play();

        let result = round.points;
        dbg!(&result);
        assert_eq!(result, 6);
    }

    #[test]
    fn parse_works() {
        let result = parse(EXPECTED.to_owned(), ParsingStrategyType::DefaultStrategy);
        dbg!(&result);
        assert_eq!(result.len(), 3)
    }

    #[test]
    fn play_tournament_works() {
        let tournament_total = play_tournament(&mut parse(
            EXPECTED.to_owned(),
            ParsingStrategyType::DefaultStrategy,
        ));
        dbg!(&tournament_total);
        assert_eq!(tournament_total, 15);

        let tournament_total = play_tournament(&mut parse(
            EXPECTED.to_owned(),
            ParsingStrategyType::CorrectStrategy,
        ));
        dbg!(&tournament_total);
        assert_eq!(tournament_total, 12)
    }
}
