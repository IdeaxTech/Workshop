import java.util.Random;
import java.util.Scanner;

public class Janken {
    public static void main(String[] args) {
        String[] choices = {"Rock", "Paper", "Scissors"};
        Scanner scanner = new Scanner(System.in);
        System.out.print("Enter 0 (Rock), 1 (Paper), 2 (Scissors): ");
        int userChoice = scanner.nextInt();
        if (userChoice < 0 || userChoice > 2) {
            System.out.println("Invalid input. Please enter 0, 1, or 2.");
            scanner.close();
            return;
        }
        int computerChoice = new Random().nextInt(3);
        System.out.println("You: " + choices[userChoice]);
        System.out.println("Computer: " + choices[computerChoice]);
        
        if (userChoice == computerChoice) {
            System.out.println("It's a tie!");
        } else if ((userChoice == 0 && computerChoice == 2) ||
                   (userChoice == 1 && computerChoice == 0) ||
                   (userChoice == 2 && computerChoice == 1)) {
            System.out.println("You win!");
        } else {
            System.out.println("You lose!");
        }
        scanner.close();
    }
}
