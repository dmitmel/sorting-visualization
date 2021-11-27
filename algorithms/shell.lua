-- https://en.wikipedia.org/wiki/Shell_sort

local len = #array

local gap = 1

while gap < (len >> 1) do
    gap = (gap << 1) + 1
end

while gap >= 1 do
  local i = gap
  while i < len do
    local k = i - gap
    local j = i
    while (j >= gap) and (array[j] < array[k]) do
      array:swap(j, k)
      array:wait(10)
      j = k
      k = k - gap
    end
    array:wait(10)
    i = i + 1
  end
  gap = gap >> 1
end
