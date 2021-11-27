-- https://en.wikipedia.org/wiki/Quicksort

function sort_slice(low, high)
  if low < high then
    local pivot = partition(low, high)

    for i in range(low, pivot) do
      array:set_color(i, 0, 1, 0, 0.3)
    end
    array:set_color(pivot, 1, 0, 0, 1)
    for i in range(pivot + 1, high + 1) do
      array:set_color(i, 0, 0, 1, 0.3)
    end

    sort_slice(low, pivot - 1)
    sort_slice(pivot + 1, high)

    for i in range(low, pivot) do
      array:reset_color(i)
    end
    array:reset_color(pivot)
    for i in range(pivot + 1, high + 1) do
      array:reset_color(i)
    end
  end
end

function partition(low, high)
  local pivot = array[high]
  local i = low
  for j in range(low, high) do
    if array[j] <= pivot then
      array:swap(i, j)
      i = i + 1
    end
    array:wait(15)
  end
  array:swap(i, high)
  return i
end

sort_slice(0, #array - 1)
