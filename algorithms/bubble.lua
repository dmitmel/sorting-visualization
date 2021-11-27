-- https://en.wikipedia.org/wiki/Bubble_sort

local len = #array

for i in range(0, len - 1) do
  local last = len - i - 1
  array:set_color(last, 0, 1, 0, 0.8)

  for j in range(0, last) do
    array:set_color(j, 0, 1, 0, 0.8)

    if array[j] > array[j + 1] then array:swap(j, j + 1) end

    array:wait(5)
    array:reset_color(j)
  end

  array:reset_color(last)
  error('error')
end
