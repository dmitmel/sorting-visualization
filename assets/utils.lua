function range(from, to)
  local i = from

  function range_iter()
    if i < to then
      j, i = i, i + 1
      return j
    end
  end

  return range_iter
end
