#!/usr/bin/env lua

math.randomseed(os.time())
for _=1,10 do
  math.random()
end

function d6 ()
  return math.random(1,6)
end

print(d6())

