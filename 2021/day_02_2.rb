# frozen_string_literal: true

def parse_action(action)
  action_name, action_value_string = action.split(' ')
  [action_name, action_value_string.to_i]
end

def call_action(name, value, aim)
  case name
  when 'up'
    [0, 0, -value]
  when 'down'
    [0, 0, value]
  when 'forward'
    [value, aim * value, 0]
  else
    raise 'Wrong action'
  end
end

# horizontal, depth, aim
start = [0, 0, 0]
input = File.readlines(ARGV[0])
input.each do |command|
  aim = start.last
  result = call_action(*parse_action(command), aim)
  start = start.zip(result).map(&:sum)
end

puts start[0] * start[1]
