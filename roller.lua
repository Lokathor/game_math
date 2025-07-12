#!/usr/bin/env lua

function d6 ()
  return math.random(1,6)
end

if not pcall(debug.getlocal, 4, 1) then
  -- in main script
  math.randomseed(os.time())
  for _=1,10 do
    math.random()
  end
  print(d6())
end
