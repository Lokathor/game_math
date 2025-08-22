
function deep_clone(obj, seen)
  if type(obj) ~= "table" then return obj end
  if seen and seen[obj] then return seen[obj] end
  local s = seen or {}
  local res = {}
  s[obj] = res
  for k, v in pairs(obj) do
    res[deep_clone(k, s)] = deep_clone(v, s)
  end
  return setmetatable(res, getmetatable(obj))
end

function fmt_table(o)
  if type(o) == 'table' then
    local s = '{ '
    for k,v in pairs(o) do
      if type(k) ~= 'number' then k = '"'..k..'"' end
      s = s .. '['..k..'] = ' .. fmt_table(v) .. ','
    end
    return s .. '} '
  else
    return tostring(o)
  end
end

function d6()
  return math.random(1,6)
end

function d3()
  return math.random(1,3)
end

function resolve_expression(x)
  if type(x) == "number" then
    return x
  elseif x == "d6" or x == "1d6" then
    return d6()
  elseif x == "d6+1" or x == "1d6+1" then
    return d6()+1
  elseif x=="d3" or x=="1d3" then
    return d3()
  elseif x=="d3+1" or x=="1d3+1" then
    return d3()+1
  elseif x=="2d6" then
    return d6()+d6()
  elseif x=="2d3" then
      return d3()+d3()
  else
    error("unknown expression:"..tostring(x))
  end
end

function max_expression(x)
  if type(x) == "number" then
    return x
  elseif x=="d6" or x=="1d6" then
    return 6
  elseif x=="d6+1" or x=="1d6+1" then
    return 7
  elseif x=="d3" or x=="1d3" then
    return 3
  elseif x=="d3+1" or x=="1d3+1" then
    return 4
  elseif x=="2d6" then
    return 12
  elseif x=="2d3" then
    return 6
  else
    error("unknown expression:"..tostring(x))
  end
end

-- true when left should sort earlier
function cmp_weapon(left,right)
  -- always sort highest damage first
  local ldam = max_expression(left.damage)
  local rdam = max_expression(right.damage)
  if ldam > rdam then
    return true
  elseif ldam < rdam then
    return false
  end
  -- fallback: just sort by name
  return left.name < right.name
end

function base_to_wound(strength, toughness)
  if strength >= (toughness*2) then
    return 2
  elseif strength > toughness then
    return 3
  elseif strength == toughness then
    return 4
  elseif strength <= (toughness//2) then
    return 6
  else
    return 5
  end
end
