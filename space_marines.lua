
require "types"

space_marines = {}
space_marines.weapons = {}
space_marines.models = {}
space_marines.datasheets = {}

space_marines.weapons.intercessor_bolt_rifle_4 = Weapon{
  name = "Bolt Rifle (4 shot)",
  range = 24,
  attacks = 4,
  base_hit = 3,
  strength = 4,
  ap = 1,
  damage = 1,
  abilities = { assault=true, heavy=true },
}

space_marines.weapons.bolt_pistol = Weapon{
  name = "Bolt Pistol",
  range = 12,
  attacks = 1,
  base_hit = 3,
  strength = 4,
  ap = 1,
  damage = 1,
  abilities = { pistol=true },
}

space_marines.weapons.close_combat = Weapon{
  name = "Close Combat Weapon",
  range = 1,
  attacks = 3,
  base_hit = 3,
  strength = 4,
  ap = 0,
  damage = 1,
}

space_marines.models.basic_intercessor = Model{
  name = "Intercessor",
  movement = 6,
  toughness = 4,
  armor = 3,
  wounds = 2,
  leadership = 6,
  oc = 2,
  weapons = {
    space_marines.weapons.intercessor_bolt_rifle_4,
    space_marines.weapons.bolt_pistol,
    space_marines.weapons.close_combat,
  }
}

space_marines.datasheets.five_basic_intercessors = Unit{
  name = "Intercessors (5 basic)",
  models = {
    space_marines.models.basic_intercessor,
    space_marines.models.basic_intercessor,
    space_marines.models.basic_intercessor,
    space_marines.models.basic_intercessor,
    space_marines.models.basic_intercessor,
  }
}
