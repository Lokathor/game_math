#!/usr/bin/env lua
-- License: CC0-1.0

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

function Weapon (args)
  local out = {}
  out.ty = "weapon"
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
  out.models = args.models or {}
  out.starting_model_count = #(out.models)
  return out
end

function do_shooting_sequence (attackers, defenders)
  local blast_bonus = #(defenders.models) // 5
  --print("blast bonus: "..blast_bonus)
  for _,attacker in pairs(attackers.models) do
    --print("attacker: " .. fmt_table(attacker))
    for _,weapon in pairs(attacker.weapons) do
      --print("weapon: " .. fmt_table(weapon))
      local num_attacks
      if type(weapon.num_attacks) == 'function' then
        num_attacks = weapon.num_attacks()
      else
        num_attacks = weapon.num_attacks
      end
      if weapon.attrs.blast then
        num_attacks = num_attacks + blast_bonus
      end

      local normal_hits = 0
      local critial_hits = 0
      for _=1,num_attacks do
        local hit_roll = d6()
        if hit_roll == 6 then
          -- TODO: ability to crit on 5s
          critial_hits = critial_hits + 1
        elseif hit_roll >= weapon.to_hit then
          normal_hits = normal_hits + 1
        end
        -- TODO: hit rerolls
      end
      
      print("normal_hits: "..normal_hits)
      print("critial_hits: "..critial_hits)
    end
  end
end

INTERCESSOR_BOLT_RIFLE = Weapon{
  range = 24,
  num_attacks = 4, -- assume focused fire
  to_hit = 3,
  strength = 4,
  ap = 1,
  damage = 1,
  attrs = { assault=true, heavy=true },
}

BASIC_INTERCESSOR = Model{
  movement = 6,
  toughness = 4,
  armor_save = 3,
  wounds = 2,
  leadership = 6,
  oc = 1,
  weapons = {INTERCESSOR_BOLT_RIFLE},
}

BASIC_INTERCESSOR_SQUAD = Unit{
  models = {
    recursive_clone(BASIC_INTERCESSOR),
    recursive_clone(BASIC_INTERCESSOR),
    recursive_clone(BASIC_INTERCESSOR),
    recursive_clone(BASIC_INTERCESSOR),
    recursive_clone(BASIC_INTERCESSOR),
  }
}

if not pcall(debug.getlocal, 4, 1) then
  -- in main script
  math.randomseed(os.time())
  for _=1,10 do
    math.random()
  end
  --
  local attackers = recursive_clone(BASIC_INTERCESSOR_SQUAD)
  local defenders = recursive_clone(BASIC_INTERCESSOR_SQUAD)
  do_shooting_sequence(attackers, defenders)
  print(#(defenders.models) .. " models survived the attack.");
end
