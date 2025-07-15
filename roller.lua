#!/usr/bin/env lua
-- License: CC0-1.0

function d6 ()
  return math.random(1,6)
end

function d3 ()
  return math.random(1,3)
end

function Weapon (args)
  local out = {}
  out.range = args.range or 0
  out.num_atks = args.num_atks or 1
  out.to_hit = args.to_hit or 7
  out.strength = args.strength or 1
  out.ap = args.ap or 0
  out.damage = args.damage or 1
  out.attrs = args.attrs or {}
  return out
end

INTERCESSOR_BOLT_RIFLE = Weapon({})

[[ Weapon Abilities
Assault
Rapid Fire
Ignores Cover
Twin-linked
Pistol
Torrent
Lethal Hits
Lance
Indirect Fire
Precision
Blast
Melta
Heavy
Hazardous
Devastating Wounds
Sustained Hits
Extra Attacks
CritHit-
Anti-
]]

[[ Defender Attributes
Movement
Toughness
ArmorSave
InvulnSave
FeelNoPain
Health
Leadership
OC
Stealth
LoneOperative
Cover
StartingModels
StartingHealthPerModel
]]

if not pcall(debug.getlocal, 4, 1) then
  -- in main script
  math.randomseed(os.time())
  for _=1,10 do
    math.random()
  end
  --
  print(INTERCESSOR_BOLT_RIFLE)
end
