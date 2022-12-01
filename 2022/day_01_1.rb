# frozen_string_literal: true

input = File.readlines(ARGV[0], chomp: true)
input << ""

calories = []
calories_count = 0

input.each do |line|
  if line.empty?
    calories << calories_count
    calories_count = 0
    next
  end

  calories_count += line.to_i
end

p(calories.max)
