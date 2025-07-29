
function Weapon(args)
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

function Model(args)
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
  out.guns = args.guns or {}
  out.pistols = args.pistols or {}
  out.melee = args.melee or {}
  return out
end

function Unit(args)
  local out = {}
  out.ty = "unit"
  out.name = args.name or "NoName"
  out.models = args.models or {}
  out.starting_model_count = #(out.models)
  out.attrs = args.attrs or {}
  return out
end
