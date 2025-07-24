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

function do_shooting_sequence (attackers, defenders, distance)
  local blast_bonus = #(defenders.models) // 5
  --print("blast bonus: "..blast_bonus)
  for _,attacker in pairs(attackers.models) do
    --print("attacker: " .. fmt_table(attacker))
    for _,weapon in pairs(attacker.weapons) do
      --print("weapon: " .. fmt_table(weapon))
      if #(defenders.models) == 0 then
        return
      end
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
      local to_wound = base_to_wound(weapon.strength, defenders.models[1].toughness)
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

      for _=1,total_wounds do
        local ti = allocate_attack(defenders)
        local target = defenders.models[ti]
        if target then
          local save_roll = d6() - weapon.ap
          if save_roll < target.armor_save then
            local potential_damage = resolve_expression(weapon.damage)
            for _=1,potential_damage do
              local fnp_roll = d6()
              if fnp_roll < target.fnp then
                target.health = target.health - 1
              end
            end
            if target.health < 1 then
              table.remove(defenders.models, ti)
            end
          end
        end
      end
    end
  end
end


if not pcall(debug.getlocal, 4, 1) then
  -- in main script
  math.randomseed(os.time())
  for _=1,10 do
    math.random()
  end
  --
  local trials = 1000
  local survivors = 0
  for _=1,trials do
    local attackers = recursive_clone(space_marines.datasheets.intercessor_squad)
    local defenders = recursive_clone(attackers)
    do_shooting_sequence(attackers, defenders, 6)
    survivors = survivors + #(defenders.models)
  end
  print("average survivors: "..(survivors/trials))
end
