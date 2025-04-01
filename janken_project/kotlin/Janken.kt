fun main() {
  println("Enter 0 (Rock), 1 (Paper), 2 (Scissors): ")
  val userInput = readLine()
  val userChoice = userInput?.toIntOrNull()

  if (userChoice == null || userChoice !in 0..2) {
    println("Invalid input. Please enter 0, 1, or 2.")
    return
  }

  val computerChoice = (0..2).random()
  println("You: ${userChoice}")
  println("Computer: ${computerChoice}")

  when {
    userChoice == computerChoice -> println("It's a draw!")
    (userChoice == 0 && computerChoice == 2) || (userChoice == 1 && computerChoice == 0) || (userChoice == 2 && computerChoice == 1) -> println("You win!")
    else -> println("You lose!")
  }
}