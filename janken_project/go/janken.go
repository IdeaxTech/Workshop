package main

import (
    "fmt"
    "math/rand"
    "time"
)

func main() {
    choices := []string{"Rock", "Paper", "Scissors"}
    var userChoice int
    fmt.Print("Enter 0 (Rock), 1 (Paper), 2 (Scissors): ")
    fmt.Scan(&userChoice)
    if userChoice < 0 || userChoice > 2 {
        fmt.Println("Invalid input. Please enter 0, 1, or 2.")
        return
    }
    rand.Seed(time.Now().UnixNano())
    computerChoice := rand.Intn(3)
    fmt.Println("You:", choices[userChoice])
    fmt.Println("Computer:", choices[computerChoice])
    
    if userChoice == computerChoice {
        fmt.Println("It's a tie!")
    } else if (userChoice == 0 && computerChoice == 2) ||
              (userChoice == 1 && computerChoice == 0) ||
              (userChoice == 2 && computerChoice == 1) {
        fmt.Println("You win!")
    } else {
        fmt.Println("You lose!")
    }
}