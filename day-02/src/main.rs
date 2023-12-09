#[derive(Debug, PartialEq, Clone, Copy)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Hand::Rock,
            "B" | "Y" => Hand::Paper,
            "C" | "Z" => Hand::Scissors,
            _ => panic!("Invalid hand!"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Prevision {
    opponent: Hand,
    outcome: Outcome,
}

#[derive(Debug, Clone, Copy)]
struct Round {
    opponent: Hand,
    me: Hand,
}

impl From<Prevision> for Round {
    fn from(prevision: Prevision) -> Self {
        let me = match prevision.outcome {
            Outcome::Draw => prevision.opponent,
            Outcome::Win => match prevision.opponent {
                Hand::Scissors => Hand::Rock,
                Hand::Paper => Hand::Scissors,
                Hand::Rock => Hand::Paper,
            },
            Outcome::Lost => match prevision.opponent {
                Hand::Scissors => Hand::Paper,
                Hand::Paper => Hand::Rock,
                Hand::Rock => Hand::Scissors,
            },
        };

        Round {
            me,
            opponent: prevision.opponent,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Lost = 0,
    Draw = 3,
    Win = 6,
}

impl From<Round> for Outcome {
    fn from(round: Round) -> Self {
        if round.me == round.opponent {
            return Outcome::Draw;
        }

        if (round.me == Hand::Paper && round.opponent == Hand::Rock)
            || (round.me == Hand::Scissors && round.opponent == Hand::Paper)
            || (round.me == Hand::Rock && round.opponent == Hand::Scissors)
        {
            return Outcome::Win;
        }

        Outcome::Lost
    }
}
impl From<&str> for Outcome {
    fn from(value: &str) -> Self {
        match value {
            "X" => Outcome::Lost,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Invalid Outcome!"),
        }
    }
}

fn main() -> std::io::Result<()> {
    let input = include_str!("../input1.txt");

    // Part 1 tournament
    //let tournament = input.lines().map(|x| x.split(" ").map(|y| y.into()).collect::<Vec<Hand>>())
    //                                            .map(|x| Round{opponent: x[0], me: x[1]})
    //                                            .collect::<Vec<_>>();

    // Part 2 tournament
    let tournament = input
        .lines()
        .map(|x| x.split(' ').collect::<Vec<&str>>())
        .map(|x| Prevision {
            opponent: x[0].into(),
            outcome: x[1].into(),
        })
        .map(Round::from)
        .collect::<Vec<Round>>();

    println!("{:?}", tournament);

    let outcomes: Vec<Outcome> = tournament.iter().map(|&round| round.into()).collect();

    println!("{:?}", outcomes);

    let my_hands: Vec<Hand> = tournament.iter().map(|round| round.me).collect();

    println!("{:?}", my_hands);

    let outcomes_score = outcomes.iter().map(|&x| x as i32).sum::<i32>();
    let my_hands_score = my_hands.iter().map(|&x| x as i32).sum::<i32>();

    println!("Outcomes score : {}", outcomes_score);
    println!("My hands score : {}", my_hands_score);
    println!("Total score : {}", outcomes_score + my_hands_score);

    Ok(())
}
