# frozen_string_literal: true

def parse_action(action)
  action_name, action_value_string = action.split(' ')
  [action_name, action_value_string.to_i]
end

def call_action(name, value)
  case name
  when 'up'
    [0, -value]
  when 'down'
    [0, value]
  when 'forward'
    [value, 0]
  else
    raise 'Wrong action'
  end
end

start = [0, 0]
input = File.readlines(ARGV[0])
input.each do |command|
  result = call_action(*parse_action(command))
  start = start.zip(result).map(&:sum)
end

puts start.first * start.last
