-- https://en.wikipedia.org/wiki/Merge_sort

function merge_sort(left, right)
  if left < right then
    local middle = (left + right) / 2
    array:set_color(middle, 1.0, 0.0, 0.0, 1.0)

    for i in range(left, middle) do
      array:set_color(i, 0.0, 1.0, 0.0, 0.3)
    end
    merge_sort(left, middle)

    for i in range(middle + 1, right + 1) do
      array:set_color(i, 0.0, 0.0, 1.0, 0.3)
    end
    merge_sort(middle + 1, right)

    merge(left, middle, right)

    for i in range(left, right + 1) do
      array:reset_color(i)
    end
  end
end

function merge(left, middle, right)
  local left_size = middle - left + 1
  local right_size = right - middle

  local left_array = sub_array(left, left_size)
  local right_array = sub_array(middle + 1, right_size)

  local i = 0
  local j = 0
  local k = left
  local prev_k = k

  while i < left_size and j < right_size do
    array:reset_color(prev_k)
    array:set_color(k, 1.0, 0.0, 0.0, 1.0)
    prev_k = k

    if left_array[i+1] <= right_array[j+1] then
      array[k] = left_array[i+1]
      i = i + 1
    else
      array[k] = right_array[j+1]
      j = j + 1
    end
    k = k + 1
    array:wait(20)
  end

  array:reset_color(prev_k)

  while i < left_size do
    array[k] = left_array[i+1]
    i = i + 1
    k = k + 1
    array:wait(20)
  end

  while j < right_size do
    array[k] = right_array[j+1]
    j = j + 1
    k = k + 1
    array:wait(20)
  end
end

function sub_array(begin, size)
  local arr = {}
  for i in range(0, size) do
    arr[i+1] = array[begin + i]
    array:wait(10)
  end
  return arr
end

merge_sort(0, #array - 1)
