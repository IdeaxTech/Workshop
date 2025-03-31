#include <iostream>
#include <cstdlib>
#include <ctime>

using namespace std;

string getHand(int num) {
    if (num == 0) return "Rock";
    if (num == 1) return "Paper";
    return "Scissors";
}

int main() {
    srand(time(0));
    int userChoice;
    cout << "Enter 0 (Rock), 1 (Paper), 2 (Scissors): ";
    cin >> userChoice;
    if (userChoice < 0 || userChoice > 2) {
        cout << "Invalid input. Please enter 0, 1, or 2.\n";
        return 1;
    }
    int computerChoice = rand() % 3;
    
    cout << "You: " << getHand(userChoice) << "\n";
    cout << "Computer: " << getHand(computerChoice) << "\n";
    
    if (userChoice == computerChoice) {
        cout << "It's a tie!\n";
    } else if ((userChoice == 0 && computerChoice == 2) ||
               (userChoice == 1 && computerChoice == 0) ||
               (userChoice == 2 && computerChoice == 1)) {
        cout << "You win!\n";
    } else {
        cout << "You lose!\n";
    }
    return 0;
}