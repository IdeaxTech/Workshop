def get_hand(num)
  ["Rock", "Paper", "Scissors"][num]
end

puts "Enter 0 (Rock), 1 (Paper), 2 (Scissors): "
user_choice = gets.to_i

if user_choice < 0 || user_choice > 2
  puts "Invalid input. Please enter 0, 1, or 2."
  exit
end

computer_choice = rand(3)

puts "You: #{get_hand(user_choice)}"
puts "Computer: #{get_hand(computer_choice)}"

if user_choice == computer_choice
  puts "It's a tie!"
elsif (user_choice == 0 && computer_choice == 2) ||
      (user_choice == 1 && computer_choice == 0) ||
      (user_choice == 2 && computer_choice == 1)
  puts "You win!"
else
  puts "You lose!"
end
