
require "utils"

INTERCESSOR_FOCUSED_BOLT_RIFLE = Weapon{
  name = "Bolt Rifle (focused)",
  range = 24,
  num_attacks = 4,
  to_hit = 3,
  strength = 4,
  ap = 1,
  damage = 1,
  attrs = { assault=true, heavy=true },
}

INTERCESSOR_FRAG_GRENADE_LAUNCHER = Weapon{
  name = "Frag Grenade Launcher",
  range = 24,
  num_attacks = "d3",
  to_hit = 3,
  strength = 4,
  ap = 0,
  damage = 1,
  attrs = { blast = true },
}

INTERCESSOR_KRAK_GRENADE_LAUNCHER = Weapon{
  name = "Krak Grenade Launcher",
  range = 24,
  num_attacks = 1,
  to_hit = 3,
  strength = 9,
  ap = 2,
  damage = "d3",
}

PLASMA_PISTOL_NORMAL = Weapon{
  name = "Plasma Pistol (normal)",
  range = 12,
  num_attacks = 1,
  to_hit = 3,
  strength = 7,
  ap = 1,
  damage = 1,
}

BASIC_INTERCESSOR = Model{
  name = "Intercessor",
  movement = 6,
  toughness = 4,
  armor_save = 3,
  wounds = 2,
  leadership = 6,
  oc = 2,
  weapons = {INTERCESSOR_FOCUSED_BOLT_RIFLE},
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
BASIC_INTERCESSOR_SQUAD.models[4].weapons[2] = recursive_clone(INTERCESSOR_KRAK_GRENADE_LAUNCHER)
BASIC_INTERCESSOR_SQUAD.models[5].weapons[1] = recursive_clone(PLASMA_PISTOL_NORMAL)
