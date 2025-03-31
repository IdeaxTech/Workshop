import random

def get_hand(num):
    return ["Rock", "Paper", "Scissors"][num]

user_choice = int(input("Enter 0 (Rock), 1 (Paper), 2 (Scissors): "))
if user_choice not in [0, 1, 2]:
    print("Invalid input. Please enter 0, 1, or 2.")
else:
    computer_choice = random.randint(0, 2)
    print(f"You: {get_hand(user_choice)}")
    print(f"Computer: {get_hand(computer_choice)}")
    
    if user_choice == computer_choice:
        print("It's a tie!")
    elif (user_choice == 0 and computer_choice == 2) or \
        (user_choice == 1 and computer_choice == 0) or \
        (user_choice == 2 and computer_choice == 1):
        print("You win!")
    else:
        print("You lose!")