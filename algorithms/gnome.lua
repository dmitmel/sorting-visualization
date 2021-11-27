-- https://en.wikipedia.org/wiki/Gnome_sort

local len = #array
local i = 0

while i < len do
  array:set_color(i, 0, 1, 0, 0.8)
  array:wait(5)
  array:reset_color(i)

  if i == 0 or array[i] >= array[i - 1] then
    i = i + 1
  else
    array:swap(i, i - 1)
    i = i - 1
  end
end
