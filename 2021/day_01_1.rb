# frozen_string_literal: true

input = File.readlines(ARGV[0]).map(&:to_i)

increased = 0
input.each_index do |depth, index|
  next if index.zero?

  increased += 1 if depth > input[index - 1]
end

puts increased
