-- https://en.wikipedia.org/wiki/Cycle_sort

local len = #array

function find_sorted_index(cycle_start, item)
  local sorted_index = cycle_start
  for i in range(cycle_start + 1, len) do
    if array[i] < item then sorted_index = sorted_index + 1 end
    array:wait(2)
  end

  return sorted_index
end

for cycle_start in range(0, len - 1) do
  array:set_color(cycle_start, 0, 0, 1, 0.5)

  local item = array[cycle_start]
  local sorted_index = find_sorted_index(cycle_start, item)

  if sorted_index == cycle_start then
    goto continue
  end

  while item == array[sorted_index] do
    sorted_index = sorted_index + 1
  end

  array[sorted_index], item = item, array[sorted_index]

  while sorted_index ~= cycle_start do
    array:set_color(sorted_index, 0, 0, 1, 0.5)
    sorted_index = cycle_start

    for i in range(cycle_start + 1, len) do
      if array[i] < item then sorted_index = sorted_index + 1 end
      array:wait(2)
    end

    while item == array[sorted_index] do
      sorted_index = sorted_index + 1
      array:wait(2)
    end

    array[sorted_index], item = item, array[sorted_index]
  end

  ::continue::
end
