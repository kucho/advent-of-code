# frozen_string_literal: true

input = File.readlines(ARGV[0]).map(&:to_i)
sums = (0..input.size).map { |index| input.slice(index, 3).sum }
increased = 0

sums.each_with_index do |depth, index|
  next if index.zero?

  increased += 1 if depth > sums[index - 1]
end

puts increased
