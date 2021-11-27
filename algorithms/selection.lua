-- https://en.wikipedia.org/wiki/Selection_sort

local len = #array

for i in range(0, len - 1) do
  array:set_color(i, 0, 1, 0, 0.7)

  local min_i = i
  for j in range(i + 1, len) do
    if array[j] < array[min_i] then
      if min_i ~= i then array:reset_color(min_i) end

      min_i = j
      array:set_color(min_i, 0, 1, 0, 0.7)
    end

    array:wait(5)
  end

  array:wait(20)

  array:swap(i, min_i)
  array:reset_color(i)
  array:reset_color(min_i)
end
