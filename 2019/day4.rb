def find_my_password(min, max)
  match = []

  (min...max).to_a.each do |n|
    digits = n.to_s.split('').map { |s| s.to_i }
    next if digits.uniq.length == digits.length

    sequence_len = 0
    consecutive = false
    digits.each_with_index do |d, i|
      next if i == 0

      if digits[i - 1] == d
        sequence_len += 1
      else
        consecutive = true if sequence_len == 1
        sequence_len = 0
      end
    end

    next unless sequence_len == 1 || consecutive

    growing = true
    digits.each_with_index do |d, i|
      next if i == 0

      if digits[i - 1] > d
        growing = false
        break
      end
    end
    next unless growing

    match << n
  end

  match.length
end

p find_my_password(234_208, 765_869)
