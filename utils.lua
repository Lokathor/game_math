
function recursive_clone(obj, seen)
  if type(obj) ~= 'table' then return obj end
  if seen and seen[obj] then return seen[obj] end
  local s = seen or {}
  local res = {}
  s[obj] = res
  for k, v in pairs(obj) do
    res[recursive_clone(k, s)] = recursive_clone(v, s)
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

function d6 ()
  return math.random(1,6)
end

function d3 ()
  return math.random(1,3)
end

function resolve_expression (x)
  if type(x) == "number" then
    return x
  elseif type(x) == "function" then
    return x()
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

function base_to_wound (strength, toughness)
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

function Weapon (args)
  local out = {}
  out.ty = "weapon"
  out.name = args.name or "NoName"
  out.range = args.range or 0
  out.num_attacks = args.num_attacks or 1
  out.to_hit = args.to_hit or 7
  out.strength = args.strength or 1
  out.ap = args.ap or 0
  out.damage = args.damage or 1
  out.attrs = args.attrs or {}
  return out
end

function Model (args)
  local out = {}
  out.ty = "model"
  out.name = args.name or "NoName"
  out.movement = args.movement or 6
  out.toughness = args.toughness or 4
  out.armor_save = args.armor_save or 7
  out.invuln_save = args.invuln_save or 7
  out.fnp = args.fnp or 7
  out.wounds = args.wounds or 1
  out.health = args.health or out.wounds
  out.leadership = args.leadership or 13
  out.oc = args.oc or 0
  out.weapons = args.weapons or {}
  return out
end

function Unit (args)
  local out = {}
  out.ty = "unit"
  out.name = args.name or "NoName"
  out.models = args.models or {}
  out.starting_model_count = #(out.models)
  out.attrs = args.attrs or {}
  return out
end
