def parse_intcode(arr)
  cursor = 0

  until  arr[cursor] == 99
    a = arr[arr[cursor + 1]]
    b = arr[arr[cursor + 2]]
    target = arr[cursor + 3]

    case arr[cursor]
    when 1
      arr[target] = a + b
    when 2
      arr[target] = a * b
    else
      raise 'Error: invalid opcode'
    end

    cursor += 4
   end
  arr
end

input = [1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 10, 19, 1, 19, 6, 23, 2, 23, 13, 27, 1, 27, 5, 31, 2, 31, 10, 35, 1, 9, 35, 39, 1, 39, 9, 43, 2, 9, 43, 47, 1, 5, 47, 51, 2, 13, 51, 55, 1, 55, 9, 59, 2, 6, 59, 63, 1, 63, 5, 67, 1, 10, 67, 71, 1, 71, 10, 75, 2, 75, 13, 79, 2, 79, 13, 83, 1, 5, 83, 87, 1, 87, 6, 91, 2, 91, 13, 95, 1, 5, 95, 99, 1, 99, 2, 103, 1, 103, 6, 0, 99, 2, 14, 0, 0]

def find_noun_verb(arr, result)
  pool = (0..99).to_a.combination(2).to_a
  base = arr[0]
  noun = 0
  verb = 0
  until base == result
    new_arr = arr.dup
    noun, verb = pool.pop
    new_arr[1] = noun
    new_arr[2] = verb
    parse_intcode(new_arr)
    base = new_arr[0]
  end
  [noun, verb]
end

noun, verb = find_noun_verb(input, 19_690_720)
p noun
p verb
