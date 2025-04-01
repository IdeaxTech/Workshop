import 'dart:math';

import 'package:args/args.dart';

void main(List<String> args) {
  final ArgParser parser = ArgParser();

  parser.addOption('choice', abbr: 'c', help: 'Enter 0 (Rock), 1 (Paper), or 2 (Scissors):');
  final ArgResults results = parser.parse(args);

  int userChoice = int.parse(results['choice'] as String);

  final random = Random();

  if (userChoice < 0 || userChoice > 2) {
    print('Invalid input. Please enter 0, 1, or 2.');
    return;
  }

  int computerChoice = random.nextInt(3);

  print('You: $userChoice');
  print('Computer: $computerChoice');

  if (userChoice == computerChoice) {
    print('It\'s is tie!');
  } else if ((userChoice == 0 && computerChoice == 2) ||
             (userChoice == 1 && computerChoice == 0) ||
             (userChoice == 2 && computerChoice == 1)) {
    print('You win!');
  } else {
    print('You lose!');
  }
}