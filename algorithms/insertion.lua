-- https://en.wikipedia.org/wiki/Insertion_sort

local len = #array

for i in range(1, len) do
  array:set_color(i, 0, 1, 0, 0.8)
  array:wait(7)

  local j = i
  while j > 0 and array[j - 1] > array[j] do
    array:swap(j, j - 1)
    j = j - 1

    array:set_color(j, 0, 1, 0, 0.8)
    array:wait(7)
    array:reset_color(j)
  end

  array:reset_color(i)
end
