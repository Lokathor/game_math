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

function queue_shooting(model, distance, weps_to_shoot, hazard_checks)
  local has_non_pistols = false
  for _,weapon in pairs(model.weapons) do
    has_non_pistols = has_non_pistols or not weapon.abilities.pistol
  end
  local block_pistols = has_non_pistols and not model.keywords.vehicle
  for _,weapon in pairs(model.weapons) do
    if weapon.range >= distance then
      if distance > 1 or (weapon.abilities.pistol and not block_pistols) then
        table.insert(weps_to_shoot, weapon)
        if weapon.abilities.hazardous then
          table.insert(hazard_checks, model)
        end
      end
    end
  end
end

function do_shooting_sequence (attackers, defenders, distance)
  print("shooting sequence!")
  -- determine per-shoot-sequence values now
  local blast_bonus = #(defenders.models) // 5
  -- Determine what weapons can fire
  local weps_to_shoot = {}
  local hazardous_checks = {}
  for _,model in pairs(attackers.models) do
    queue_shooting(model, distance, weps_to_shoot, hazardous_checks)
  end
  table.sort(weps_to_shoot, cmp_weapon)
  local devastating_damage_queue = {}
  for _,weapon in pairs(weps_to_shoot) do
    if #(defenders.models) == 0 then
      return
    end
    print("shooting: "..weapon.name)
    -- TODO: shoot the weapon
  end
  -- TODO: allocate floating devastating damage
  -- TODO: hazard checks
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
    local attackers = deep_clone(space_marines.datasheets.five_basic_intercessors)
    local defenders = deep_clone(attackers)
    do_shooting_sequence(attackers, defenders, 6)
    survivors = survivors + #(defenders.models)
  end
  print("average survivors: "..(survivors/trials))
end
