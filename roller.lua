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
  return out
end

function allocate_attack (defenders)
  -- TODO: pick a defender properly
  for k,v in pairs(defenders.models) do
    return k
  end
end

function do_shooting_sequence (attackers, defenders)
  local blast_bonus = #(defenders.models) // 5
  --print("blast bonus: "..blast_bonus)
  for _,attacker in pairs(attackers.models) do
    --print("attacker: " .. fmt_table(attacker))
    for _,weapon in pairs(attacker.weapons) do
      --print("weapon: " .. fmt_table(weapon))
      local num_attacks = resolve_expression(weapon.num_attacks)
      if weapon.attrs.blast then
        num_attacks = num_attacks + blast_bonus
      end

      local normal_hits = 0
      local critical_hits = 0
      local to_hit = weapon.to_hit
      local to_crit_hit = 6 -- TODO: crit hit 5s
      for _=1,num_attacks do
        local hit_roll = d6()
        if hit_roll >= to_crit_hit then
          critical_hits = critical_hits + 1
        elseif hit_roll >= to_hit then
          normal_hits = normal_hits + 1
        end
        -- TODO: hit rerolls
      end
      local wound_rolls = normal_hits + critical_hits

      local normal_wounds = 0
      local critical_wounds = 0
      local to_wound = 4
      local to_crit_wound = 6 -- TODO: anti
      for _=1,wound_rolls do
        local wound_roll = d6()
        if wound_roll >= to_crit_wound then
          critical_wound = critical_wounds + 1
        elseif wound_roll >= to_wound then
          normal_wounds = normal_wounds + 1
        end
        -- TODO: wound rerolls
      end
      local total_wounds = normal_wounds + critical_wounds

      local damage = 0
      for _=1,total_wounds do
        local ti = allocate_attack(defenders)
        local target = defenders.models[ti]
        if target then
          local save_roll = d6()
          if save_roll < target.armor_save then
            for _=1,weapon.damage do
              local fnp_roll = d6()
              if fnp_roll < target.fnp then
                damage = damage + 1
                target.health = target.health - 1
              end
            end
          end
          print("target.health: "..target.health)
          if target.health < 1 then
            defenders.models[ti] = nil
          end
        end
      end
      
      print("damage: "..damage)
    end
  end
end

INTERCESSOR_BOLT_RIFLE = Weapon{
  name = "Bolt Rifle (focused)",
  range = 24,
  num_attacks = 4, -- assume focused fire
  to_hit = 3,
  strength = 4,
  ap = 1,
  damage = 1,
  attrs = { assault=true, heavy=true },
}

BASIC_INTERCESSOR = Model{
  name = "Intercessor",
  movement = 6,
  toughness = 4,
  armor_save = 3,
  wounds = 2,
  leadership = 6,
  oc = 1,
  weapons = {INTERCESSOR_BOLT_RIFLE},
}

BASIC_INTERCESSOR_SQUAD = Unit{
  name = "Intercessors",
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
