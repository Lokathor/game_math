
require "utils"

space_marines.weapons = {}
space_marines.models = {}
space_marines.datasheets = {}

space_marines.weapons.bolt_rifle_focused = Weapon{
  name = "Bolt Rifle (focused)",
  range = 24,
  num_attacks = 4,
  to_hit = 3,
  strength = 4,
  ap = 1,
  damage = 1,
  attrs = { assault=true, heavy=true },
}

space_marines.weapons.krak_grenade_launcher = Weapon{
  name = "Grenade Launcher (krak)",
  range = 24,
  num_attacks = 1,
  to_hit = 3,
  strength = 9,
  ap = 2,
  damage = "d3",
}

space_marines.weapons.bolt_pistol_default = Weapon{
  name = "Bolt Pistol",
  range = 12,
  num_attacks = 1,
  to_hit = 3,
  strength = 4,
  ap = 0,
  damage = 1,
  attrs = { pistol = true },
}

space_marines.weapons.heavy_bolt_pistol_default =  Weapon{
  name = "Heavy Bolt Pistol",
  range = 18,
  num_attacks = 1,
  to_hit = 3,
  strength = 4,
  ap = 1,
  damage = 1,
  attrs = { pistol = true },
}

space_marines.weapons.plasma_pistol_normal_default = Weapon{
  name = "Plasma Pistol (normal)",
  range = 12,
  num_attacks = 1,
  to_hit = 3,
  strength = 7,
  ap = 2,
  damage = 1,
  attrs = { pistol = true },
}

space_marines.weapons.close_combat_default = Weapon{
  name = "Close combat weapon",
  range= 1,
  num_attacks=3,
  to_hit = 3,
  strength=4,
  ap= 0,
  damage = 1,
}

space_marines.weapons.power_fist_3_3 = Weapon{
  name = "Power Fist",
  range= 1,
  num_attacks=3,
  to_hit = 3,
  strength=8,
  ap= 2,
  damage = 2,
}

space_marines.weapons.bladeguard_sword = Weapon{
  name = "Master-crafted Power Weapon",
  range = 1,
  num_attacks = 4,
  to_hit = 3,
  strength = 5,
  ap = 2,
  damage = 2,
}

space_marines.models.basic_intercessor = Model{
  name = "Intercessor",
  movement = 6,
  toughness = 4,
  armor_save = 3,
  wounds = 2,
  leadership = 6,
  oc = 2,
  weapons = {
    space_marines.weapons.bolt_rifle_focused,
    space_marines.weapons.bolt_pistol_default,
    space_marines.weapons.close_combat_default,
  },
}

space_marines.models.intercessor_krak = Model{
  name = "Intercessor",
  movement = 6,
  toughness = 4,
  armor_save = 3,
  wounds = 2,
  leadership = 6,
  oc = 2,
  weapons = {
    space_marines.weapons.bolt_rifle_focused,
    space_marines.weapons.krak_grenade_launcher,
    space_marines.weapons.bolt_pistol_default,
    space_marines.weapons.close_combat_default,
  }
},

space_marines.models.intercessor_fist_rifle = Model{
  name = "Intercessor Sergeant",
  movement = 6,
  toughness = 4,
  armor_save = 3,
  wounds = 2,
  leadership = 6,
  oc = 2,
  weapons = {
    space_marines.weapons.bolt_rifle_focused,
    space_marines.weapons.bolt_pistol_default,
    space_marines.weapons.power_fist_3_3,
  }
},

space_marines.datasheets.intercessor_squad = Unit{
  name = "Intercessors",
  models = {
    space_marines.models.basic_intercessor,
    space_marines.models.basic_intercessor,
    space_marines.models.basic_intercessor,
    space_marines.models.intercessor_krak,
    space_marines.models.intercessor_fist_rifle,
  }
}
