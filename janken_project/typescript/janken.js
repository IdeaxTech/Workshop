var readline = require('readline');
var choices = ["Rock", "Paper", "Scissors"];
var rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});
rl.question("Enter 0 (Rock), 1 (Paper), 2 (Scissors): ", function (answer) {
    var userChoice = parseInt(answer);
    if (isNaN(userChoice) || userChoice < 0 || userChoice > 2) {
        console.log("Invalid input. Please enter 0, 1, or 2.");
        rl.close();
        return;
    }
    var computerChoice = Math.floor(Math.random() * 3);
    console.log("You: ".concat(choices[userChoice]));
    console.log("Computer: ".concat(choices[computerChoice]));
    if (userChoice === computerChoice) {
        console.log("It's a tie!");
    }
    else if ((userChoice === 0 && computerChoice === 2) ||
        (userChoice === 1 && computerChoice === 0) ||
        (userChoice === 2 && computerChoice === 1)) {
        console.log("You win!");
    }
    else {
        console.log("You lose!");
    }
    rl.close();
});
