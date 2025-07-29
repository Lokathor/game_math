
function Weapon(args)
  local out = {}
  out.ty = "weapon"
  out.name = args.name or error("name")
  out.range = args.range or error("range")
  out.attacks = args.attacks or error("attacks")
  out.base_hit = args.base_hit or error("base_hit")
  out.strength = args.strength or error("strength")
  out.ap = args.ap or 0
  out.damage = args.damage or 1
  out.attrs = args.attrs or {}
  return out
end

function Model(args)
  local out = {}
  out.ty = "model"
  out.name = args.name or error("name")
  out.movement = args.movement or error("movement")
  out.toughness = args.toughness or error("toughness")
  out.armor = args.armor or error("armor")
  out.invuln = args.invuln
  out.fnp = args.fnp
  out.wounds = args.wounds or error("wounds")
  out.health = args.health or out.wounds
  out.leadership = args.leadership or error("leadership")
  out.oc = args.oc or error("oc")
  out.ranged = args.ranged or {}
  out.melee = args.melee or {}
  return out
end

function Unit(args)
  local out = {}
  out.ty = "unit"
  out.name = args.name or error("name")
  out.models = args.models or {}
  out.starting_model_count = #(out.models)
  out.attrs = args.attrs or {}
  return out
end
