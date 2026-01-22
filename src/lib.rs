use randomize::Gen32;

pub mod units;
pub use units::*;

#[allow(unused)]
pub fn do_shooting(
  g: &mut impl Gen32, attacker: &mut Unit, defender: &mut Unit,
  mut ctx: Context,
) {
  if attacker.models[0].rules.contains(&ModelRule::EagleOptics) {
    // Assumption: the Eagle Optics rule is per model, so this ends up being
    // wrong if the model is ever joined into a larger unit (which is currently
    // impossible).
    ctx.reroll_hit_rolls =
      ctx.reroll_hit_rolls.max(RerollAvailabilty::Limited(1));
    ctx.reroll_wound_rolls =
      ctx.reroll_wound_rolls.max(RerollAvailabilty::Limited(1));
    ctx.reroll_damage_rolls =
      ctx.reroll_damage_rolls.max(RerollAvailabilty::Limited(1));
  }
  if ctx.target_is_oath_target
    && attacker.models[0].rules.contains(&ModelRule::OathOfMoment)
  {
    ctx.reroll_hit_rolls =
      ctx.reroll_hit_rolls.max(RerollAvailabilty::Unlimited);
    if ctx.oath_effect_wound_bonus {
      ctx.attacker_wound_modifier += 1;
    }
  }

  let mut apply_dark_pact_effect = false;
  if attacker.models.iter().any(|m| m.rules.contains(&ModelRule::DarkPacts)) {
    // trigger a dark pact
    let unit_leadership = i32::from(
      attacker.models.iter().map(|m| m.leadership).min().unwrap_or_default(),
    );
    let mut leadership_roll = g.d6() + g.d6();
    if leadership_roll < unit_leadership
      && attacker.models.iter().any(|m| m.rules.contains(&ModelRule::ChaosIcon))
    {
      leadership_roll = g.d6() + g.d6();
    }
    if leadership_roll < unit_leadership {
      let damage_roll = Expr::D3(1, 0).roll(g);
      for _ in 0..damage_roll {
        let target_index = 0;
        if let Some(m) = attacker.models.get_mut(target_index) {
          if let Some(tn) = m.fnp.or(m.fnp_dev) {
            if g.d6() < i32::from(tn) {
              m.health = m.health.saturating_sub(1);
            }
          } else {
            m.health = m.health.saturating_sub(1);
          }
          if m.health == 0 {
            attacker.models.remove(target_index);
          }
        } else {
          return;
        }
      }
    }
    apply_dark_pact_effect = true;
    if attacker
      .models
      .iter()
      .any(|m| m.rules.contains(&ModelRule::TerminatorDespoilers))
    {
      ctx.reroll_hit_rolls = RerollAvailabilty::Unlimited;
    }
  }

  let mut shooting_weapons = vec![];
  // gather weapons that will shoot.
  for model in attacker.models.iter() {
    for gun in model.guns.iter() {
      if gun.range >= ctx.range {
        shooting_weapons.push(gun.clone());
        if apply_dark_pact_effect {
          if ctx.dark_pact_for_sustained {
            shooting_weapons
              .last_mut()
              .unwrap()
              .rules
              .push(WeaponRule::SustainedHits(Expr::_1));
          } else {
            shooting_weapons
              .last_mut()
              .unwrap()
              .rules
              .push(WeaponRule::LethalHits);
          }
        }
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
    if ctx.range <= (gun.range / 2) {
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
    let mut hit_tn_delta =
      if attacker.models[0].rules.contains(&ModelRule::Stealth) {
        ctx.attacker_hit_modifier - 1
      } else {
        ctx.attacker_hit_modifier
      }
      .clamp(-1, 1);
    // subtract so that a roll modifier becomes a tn modifier
    let hit_tn = base_hit_tn - hit_tn_delta;
    let crit_tn = 6;
    let mut required_wound_rolls = 0;
    let mut required_save_rolls = 0;

    for _ in 0..attacks_todo {
      let mut attack_roll = g.d6();

      // determine what hit reroll to do, if any.
      if attack_roll < hit_tn {
        match ctx.reroll_hit_rolls {
          RerollAvailabilty::NoRerolls => (),
          RerollAvailabilty::Limited(n) => {
            if n > 0 {
              attack_roll = g.d6();
              ctx.reroll_hit_rolls = RerollAvailabilty::Limited(n - 1);
            } else {
              ctx.reroll_hit_rolls = RerollAvailabilty::NoRerolls;
            }
          }
          RerollAvailabilty::Unlimited => {
            attack_roll = g.d6();
          }
        }
      }

      if attack_roll >= crit_tn {
        required_wound_rolls += 1;
        if weapon_is_lethal_hits {
          required_save_rolls += 1;
        }
        if let Some(sustained) = opt_sustained_hits {
          required_wound_rolls += sustained.roll(g);
        }
      } else if attack_roll >= hit_tn {
        required_wound_rolls += 1;
      };
    }

    /*
     * WOUND ROLL
     */
    let defender_toughness =
      if let Some(m) = defender.models.get(0) { m.toughness } else { return };
    let base_wound_tn = calc_base_wound_tn(gun.strength, defender_toughness);
    let wound_tn_delta = ctx.attacker_wound_modifier.clamp(-1, 1);
    // subtract so that a roll modifier becomes a tn modifier
    let wound_tn = base_wound_tn - wound_tn_delta;
    let mut crit_wound_tn = 6;
    for weapon_rule in gun.rules.iter() {
      if let WeaponRule::Anti(model_rule, x) = weapon_rule {
        if defender.models[0].rules.contains(model_rule) {
          crit_wound_tn = crit_wound_tn.min(*x as i32);
        }
      }
    }
    for _ in 0..required_wound_rolls {
      let mut wound_roll = g.d6();

      if wound_roll < wound_tn {
        if gun.rules.contains(&WeaponRule::TwinLinked) {
          wound_roll = g.d6();
        } else {
          match ctx.reroll_wound_rolls {
            RerollAvailabilty::NoRerolls => (),
            RerollAvailabilty::Limited(n) => {
              if n > 0 {
                wound_roll = g.d6();
                ctx.reroll_wound_rolls = RerollAvailabilty::Limited(n - 1);
              } else {
                ctx.reroll_wound_rolls = RerollAvailabilty::NoRerolls;
              }
            }
            RerollAvailabilty::Unlimited => wound_roll = g.d6(),
          }
        }
      }

      if wound_roll >= crit_wound_tn {
        if weapon_is_devastating_wounds {
          devastating.push(gun.damage.roll(g));
        } else {
          required_save_rolls += 1;
        }
      } else if wound_roll >= wound_tn {
        required_save_rolls += 1;
      }
    }

    let effective_ap = (i32::from(gun.ap) + ctx.attacker_ap_modifier).max(0);

    /*
     * SAVE ROLL
     */
    for _ in 0..required_save_rolls {
      let target_index = 0;
      if let Some(def) = defender.models.get_mut(target_index) {
        let benefit_of_cover = if ctx.defender_has_cover
          && !(def.armor <= 3 && effective_ap == 0)
          && !gun.rules.contains(&WeaponRule::IgnoresCover)
        {
          1
        } else {
          0
        };
        let armor_tn = i32::from(def.armor) + effective_ap - benefit_of_cover;
        let invuln_tn = def.invuln.unwrap_or(7);
        let save_tn = armor_tn.min(i32::from(invuln_tn));
        let save_roll = g.d6();
        if save_roll < i32::from(save_tn) {
          let mut damage_roll = gun.damage.roll(g);

          if gun.damage.reroll_favored(damage_roll) {
            match ctx.reroll_damage_rolls {
              RerollAvailabilty::NoRerolls => (),
              RerollAvailabilty::Limited(n) => {
                if n > 0 {
                  damage_roll = gun.damage.roll(g);
                  ctx.reroll_damage_rolls = RerollAvailabilty::Limited(n - 1);
                } else {
                  ctx.reroll_damage_rolls = RerollAvailabilty::NoRerolls;
                }
              }
              RerollAvailabilty::Unlimited => damage_roll = gun.damage.roll(g),
            }
          }

          // TODO: defender damage halfing
          // TODO: defender damage minus 1
          // TODO: melta damage plus

          if let Some(tn) = def.fnp {
            for _ in 0..damage_roll {
              if g.d6() < i32::from(tn) {
                def.health = def.health.saturating_sub(1);
              }
            }
          } else {
            def.health = def.health.saturating_sub(damage_roll as u8);
          }

          if def.health == 0 {
            defender.models.remove(target_index);
          }
        }
      } else {
        return;
      };
    }
  }

  /*
   * DEVASTATING DAMAGE
   */
  for devastating_damage in devastating {
    let target_index = 0;
    if let Some(m) = defender.models.get_mut(target_index) {
      if let Some(tn) = m.fnp.or(m.fnp_dev) {
        for _ in 0..devastating_damage {
          if g.d6() < i32::from(tn) {
            m.health = m.health.saturating_sub(1);
          }
        }
      } else {
        m.health = m.health.saturating_sub(devastating_damage as u8);
      }
      if m.health == 0 {
        defender.models.remove(target_index);
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
  pub leadership: u8,
  pub oc: u8,
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
  pub const _6: Self = Self::F(6);

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

  pub fn reroll_favored(&self, current: i32) -> bool {
    match self {
      Expr::F(_) => false,
      Expr::D3(x, y) => (current - i32::from(*y)) <= (i32::from(*x) * 2),
      Expr::D6(x, y) => (current - i32::from(*y)) <= (i32::from(*x) * 3),
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
  ChaosIcon,
  Battleline,
  MeteoricDescent,
  OathOfMoment,
  JumpPack,
  Gravis,
  Stealth,
  BringerOfChange, // TODO
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
  Pistol,
  Torrent,
  Psychic,
  Assault,
  Hazardous,
  Precision,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Context {
  pub range: u8,
  pub storm_of_fire: bool,
  pub devastator_doctrine: bool,
  pub attacker_movement: UnitMovement,
  pub defender_below_half_strength: bool,
  pub dark_pact_for_sustained: bool,
  pub defender_has_cover: bool,
  pub attacker_hit_modifier: i32,
  pub attacker_wound_modifier: i32,
  pub attacker_ap_modifier: i32,
  pub reroll_number_of_attacks: RerollAvailabilty,
  pub reroll_hit_rolls: RerollAvailabilty,
  pub reroll_wound_rolls: RerollAvailabilty,
  pub reroll_damage_rolls: RerollAvailabilty,
  pub target_is_oath_target: bool,
  pub oath_effect_wound_bonus: bool,
  pub attacker_on_objective: bool,
  pub defender_on_objective: bool,
  pub attacker_controls_objective: bool,
  pub defender_controls_objective: bool,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnitMovement {
  #[default]
  Normal,
  Advance,
  Stationary,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RerollAvailabilty {
  #[default]
  NoRerolls,
  Limited(u32),
  Unlimited,
}
