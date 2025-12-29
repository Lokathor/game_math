use randomize::Gen32;

pub mod units;
pub use units::*;

#[allow(unused)]
pub fn do_shooting(
  g: &mut impl Gen32, attacker: &mut Unit, defender: &mut Unit, distance: u8,
  ctx: Context,
) {
  let mut eagle_hit_reroll =
    attacker.models[0].rules.contains(&ModelRule::EagleOptics);
  let mut eagle_wound_reroll =
    attacker.models[0].rules.contains(&ModelRule::EagleOptics);
  let mut eagle_damage_reroll =
    attacker.models[0].rules.contains(&ModelRule::EagleOptics);

  // TODO: Dark Pact
  // TODO: Terminator Despoilers

  let mut shooting_weapons = vec![];
  // gather weapons that will shoot.
  for model in attacker.models.iter() {
    for gun in model.guns.iter() {
      if gun.range >= distance {
        shooting_weapons.push(gun.clone());
      }
    }
  }
  shooting_weapons.sort();
  shooting_weapons.reverse();

  let mut devastating = Vec::new();
  // shoot the weapons
  for gun in shooting_weapons.iter() {
    let weapon_is_lethal_hits = gun.rules.contains(&WeaponRule::LethalHits);
    let weapon_is_devastating_wounds =
      gun.rules.contains(&WeaponRule::DevastatingWounds);
    let opt_sustained_hits = gun
      .rules
      .iter()
      .filter_map(|r| match r {
        WeaponRule::SustainedHits(xpr) => Some(*xpr),
        _ => None,
      })
      .max_by_key(|xpr| xpr.max_roll());
    let mut attacks_todo = gun.attacks.roll(g);
    if distance <= (gun.range / 2) {
      attacks_todo += gun
        .rules
        .iter()
        .filter_map(|r| match r {
          WeaponRule::RapidFire(xpr) => Some(*xpr),
          _ => None,
        })
        .max_by_key(|xpr| xpr.max_roll())
        .map(|xpr| xpr.roll(g))
        .unwrap_or(0);
    }

    /*
     * ATTACK ROLL
     */
    let base_hit_tn = gun.skill as i32;
    // TODO: stealth
    let hit_tn_delta = 0;
    let hit_tn = base_hit_tn + hit_tn_delta.clamp(-1, 1);
    let crit_tn = 6;
    let mut hits = 0;
    let mut saves = 0;

    for _ in 0..attacks_todo {
      let mut attack_roll = g.d6();

      if eagle_hit_reroll && attack_roll < hit_tn {
        eagle_hit_reroll = false;
        attack_roll = g.d6();
      }
      // TODO: other situations that can cause an attack reroll go here.

      if attack_roll >= crit_tn {
        hits += 1;
        if weapon_is_lethal_hits {
          saves += 1;
        }
        if let Some(sustained) = opt_sustained_hits {
          hits += sustained.roll(g);
        }
      } else if attack_roll >= hit_tn {
        hits += 1;
      };
    }

    /*
     * WOUND ROLL
     */
    let defender_toughness =
      if let Some(m) = defender.models.get(0) { m.toughness } else { return };
    let base_wound_tn = calc_base_wound_tn(gun.strength, defender_toughness);
    let wound_tn_delta = 0;
    let wound_tn = base_wound_tn + wound_tn_delta.clamp(-1, 1);
    let mut crit_wound_tn = 6;
    for weapon_rule in gun.rules.iter() {
      if let WeaponRule::Anti(model_rule, x) = weapon_rule {
        if defender.models[0].rules.contains(model_rule) {
          crit_wound_tn = crit_wound_tn.min(*x as i32);
        }
      }
    }
    for _ in 0..hits {
      let mut wound_roll = g.d6();

      if eagle_wound_reroll && wound_roll < wound_tn {
        eagle_wound_reroll = false;
        wound_roll = g.d6();
      }
      // TODO: other situations that can cause a wound reroll go here.

      if wound_roll >= crit_wound_tn {
        if weapon_is_devastating_wounds {
          devastating.push(gun.damage.roll(g));
        } else {
          saves += 1;
        }
      } else if wound_roll >= wound_tn {
        saves += 1;
      };
    }

    /*
     * SAVE ROLL
     */
    for _ in 0..saves {
      let benefit_of_cover = if ctx.defender_has_cover
        && !(defender.models[0].armor <= 3 && gun.ap == 0)
        && !gun.rules.contains(&WeaponRule::IgnoresCover)
      {
        1
      } else {
        0
      };
      let armor_tn = defender.models[0].armor + gun.ap - benefit_of_cover;
      let invuln_tn = defender.models[0].invuln.unwrap_or(7);
      let save_tn = armor_tn.min(invuln_tn);
      let save_roll = g.d6();
      if save_roll < i32::from(save_tn) {
        let mut damage_roll = gun.damage.roll(g);
        if damage_roll <= 6 && eagle_damage_reroll {
          eagle_damage_reroll = false;
          damage_roll = gun.damage.roll(g);
        }

        if let Some(tn) = defender.models[0].fnp {
          for _ in 0..damage_roll {
            if g.d6() < i32::from(tn) {
              defender.models[0].health =
                defender.models[0].health.saturating_sub(1);
            }
          }
        } else {
          defender.models[0].health =
            defender.models[0].health.saturating_sub(damage_roll as u8);
        }

        if defender.models[0].health == 0 {
          defender.models.remove(0);
        }
      }
    }
  }

  /*
   * DEVASTATING DAMAGE
   */
  for devastating_damage in devastating {
    if let Some(m) = defender.models.get_mut(0) {
      if let Some(tn) = m.fnp.or(m.fnp_dev) {
        for _ in 0..devastating_damage {
          if g.d6() < i32::from(tn) {
            defender.models[0].health =
              defender.models[0].health.saturating_sub(1);
          }
        }
      } else {
        defender.models[0].health =
          defender.models[0].health.saturating_sub(devastating_damage as u8);
      }
      if defender.models[0].health == 0 {
        defender.models.remove(0);
      }
    } else {
      return;
    }
  }
}

fn calc_base_wound_tn(attacker_str: u8, defender_toughness: u8) -> i32 {
  if attacker_str >= 2 * defender_toughness {
    2
  } else if attacker_str > defender_toughness {
    3
  } else if attacker_str == defender_toughness {
    4
  } else if attacker_str <= defender_toughness / 2 {
    6
  } else {
    5
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Unit {
  pub name: String,
  pub models: Vec<Model>,
  pub starting_models: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Model {
  pub name: String,
  pub speed: u8,
  pub toughness: u8,
  pub armor: u8,
  pub invuln: Option<u8>,
  pub fnp: Option<u8>,
  pub fnp_dev: Option<u8>,
  pub health: u8,
  pub starting_health: u8,
  pub guns: Vec<Weapon>,
  pub sticks: Vec<Weapon>,
  pub rules: Vec<ModelRule>,
}
impl Model {
  pub fn is_vehicle(&self) -> bool {
    self.rules.contains(&ModelRule::Vehicle)
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Weapon {
  pub damage: Expr,
  pub name: String,
  pub range: u8,
  pub attacks: Expr,
  pub skill: u8,
  pub strength: u8,
  pub ap: u8,
  pub rules: Vec<WeaponRule>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Expr {
  F(u8),
  D3(u8, u8),
  D6(u8, u8),
}
impl Expr {
  pub const _1: Self = Self::F(1);
  pub const _2: Self = Self::F(2);
  pub const _3: Self = Self::F(3);
  pub const _4: Self = Self::F(4);
  pub const _5: Self = Self::F(5);

  pub fn roll(&self, g: &mut impl Gen32) -> i32 {
    match self {
      Self::F(f) => i32::from(*f),
      Self::D3(count, bonus) => {
        let mut total = i32::from(*bonus);
        for _ in 0..*count {
          total += ((g.d6() as f32) / 2.0).ceil() as i32;
        }
        total
      }
      Self::D6(count, bonus) => {
        let mut total = i32::from(*bonus);
        for _ in 0..*count {
          total += g.d6();
        }
        total
      }
    }
  }

  pub fn max_roll(&self) -> i32 {
    match self {
      Self::F(x) => i32::from(*x),
      Self::D3(x, y) => i32::from(*x) * 3 + i32::from(*y),
      Self::D6(x, y) => i32::from(*x) * 6 + i32::from(*y),
    }
  }
}
impl Default for Expr {
  #[inline]
  fn default() -> Self {
    Self::F(1)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ModelRule {
  Vehicle,
  Smoke,
  Imperium,
  EagleOptics,
  Walker,
  Dreadnought,
  BallistusStrike,
  DarkPacts,
  Infantry,
  Chaos,
  DeepStrike,
  Fly,
  TerminatorDespoilers,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WeaponRule {
  Heavy,
  Anti(ModelRule, u8),
  RapidFire(Expr),
  Blast,
  TwinLinked,
  SustainedHits(Expr),
  LethalHits,
  DevastatingWounds,
  IgnoresCover,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Context {
  pub oath: bool,
  pub storm_of_fire: bool,
  pub devastator_doctrine: bool,
  pub attacker_movement: UnitMovement,
  pub defender_below_half_strength: bool,
  pub dark_pact_for_sustained: bool,
  pub defender_has_cover: bool,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnitMovement {
  #[default]
  Normal,
  Advance,
  Stationary,
}
