#!/usr/bin/env lua
-- License: CC0-1.0

require "utils"
require "space_marines"

function allocate_attack (defenders)
  -- TODO: pick a defender properly
  for k,v in pairs(defenders.models) do
    return k
  end
end

function cmp_weapon (left, right)
  -- sorting non-number damage values is weird.
  if type(left.damage) ~= "number" then
    if type(right.damage) ~= "number" then
      return left.name < right.name
    else
      return true
    end
  end
  if type(right.damage) ~= "number" then
    return false
  end
  -- now it's just numbers
  if left.damage > right.damage then
    return true
  elseif left.damage < right.damage then
    return false
  else
    return left.name < right.name
  end
end

function do_shooting_sequence (attackers, defenders, distance)
  print("shooting sequence!")
  -- determine per-shoot-sequence values now
  local blast_bonus = #(defenders.models) // 5
  -- Determine what weapons can fire
  local weps = {}
  for _,attacker in pairs(attackers.models) do
    if distance <= 1 then
      -- melee shooting
      for _,pistol in pairs(attacker.pistols) do
        table.insert(weps,pistol)
      end
      -- TODO: Big Guns Never Tire
    else
      for _,gun in pairs(attacker.guns) do
        if gun.range >= distance then
          table.insert(weps,gun)
        end
      end
    end
  end
  table.sort(weps, cmp_weapon)
  -- devastating damage floats until the end
  local devastating_damage = {}
  for _,weapon in pairs(weps) do
    if #(defenders.models) == 0 then
      return
    end
    print("shooting: "..weapon.name)
    -- TODO: shoot the weapon
  end
  -- TODO: allocate floating devastating damage
end

if not pcall(debug.getlocal, 4, 1) then
  -- in main script
  math.randomseed(os.time())
  for _=1,10 do
    math.random()
  end
  --
  local trials = 1
  local survivors = 0
  for _=1,trials do
    local attackers = recursive_clone(space_marines.datasheets.intercessor_squad)
    local defenders = recursive_clone(attackers)
    do_shooting_sequence(attackers, defenders, 26)
    survivors = survivors + #(defenders.models)
  end
  print("average survivors: "..(survivors/trials))
end
