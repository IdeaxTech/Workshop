use rand::Rng;

#[derive(Debug, PartialEq, Eq)]
enum Hand {
    Rock,
    Scissors,
    Paper,
}

fn main() {
    println!("Enter 0 (Rock), 1 (Paper), 2 (Scissors), -1 (End): ");

    // 乱数用の準備
    let mut rng = rand::rng();

    loop {
        // 入力受付
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let user_hand_num = input.trim().parse::<i8>().unwrap();
        let user_hand = match user_hand_num {
            0 => Hand::Rock,
            1 => Hand::Paper,
            2 => Hand::Scissors,
            -1 => {
                println!("Bay!");
                break;
            }
            _ => {
                println!("Invalid input. Please enter 0, 1, or 2.");
                continue;
            }
        };

        // コンピュータの手をランダムに決定
        let computer_hand_num = rng.random_range(0..3);
        let computer_hand = match computer_hand_num {
            0 => Hand::Rock,
            1 => Hand::Paper,
            2 => Hand::Scissors,
            _ => unreachable!(),
        };

        // 勝敗判定
        println!("You: {:?}", user_hand);
        println!("Computer: {:?}", computer_hand);
        match (user_hand, computer_hand) {
            (Hand::Rock, Hand::Scissors)
            | (Hand::Scissors, Hand::Paper)
            | (Hand::Paper, Hand::Rock) => println!("You win!"),
            (Hand::Rock, Hand::Paper)
            | (Hand::Scissors, Hand::Rock)
            | (Hand::Paper, Hand::Scissors) => println!("You lose!"),
            (user, computer) if user == computer => {
                println!("It's a tie!");
            }
            _ => {
                println!("Invalid game state.");
            }
        }
    }
}
