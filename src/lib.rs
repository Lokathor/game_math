use randomize::Gen32;
use randomize::PCG32;

#[allow(unused)]
pub fn do_shooting(
  attacker: &mut Unit, defender: &mut Unit, distance: u8, cover: f32,
  effects: Effects,
) {
  let mut g = &mut randomize::PCG32::from_getrandom().unwrap();
  let mut eagle_hit_reroll =
    attacker.models[0].rules.contains(&ModelRule::EagleOptics);
  let mut eagle_wound_reroll =
    attacker.models[0].rules.contains(&ModelRule::EagleOptics);
  let mut eagle_damage_reroll =
    attacker.models[0].rules.contains(&ModelRule::EagleOptics);

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

  // shoot the weapons
  for gun in shooting_weapons.iter() {
    let hit_tn = gun.skill.unwrap_or_default() as i32;
    let mut hits = 0;
    let mut attacks = gun.attacks.roll(g);
    for _ in 0..attacks {
      if g.d6() >= hit_tn {
        hits += 1;
      } else if eagle_hit_reroll {
        eagle_hit_reroll = false;
        if g.d6() >= hit_tn {
          hits += 1;
        }
      }
    }
    let mut wound_tn = 4;
    let mut wounds = 0;
    for _ in 0..hits {
      if g.d6() >= wound_tn {
        wounds += 1;
      } else if eagle_wound_reroll {
        eagle_wound_reroll = false;
        if g.d6() >= wound_tn {
          wounds += 1;
        }
      }
    }
    for _ in 0..wounds {
      if g.d6() < 4 {
        let mut dam = gun.damage.roll(g);
        if dam < 6 && eagle_damage_reroll {
          eagle_damage_reroll = false;
          dam = gun.damage.roll(g);
        }
        defender.models[0].health =
          defender.models[0].health.saturating_sub(dam);
      }
    }
  }
}

pub fn gladiator_lancer_w_grenades() -> Unit {
  Unit {
    name: "Galdiator Lancer".into(),
    models: vec![Model {
      name: "Gladiator Lancer".into(),
      speed: 10,
      toughness: 10,
      armor: 3,
      health: 12,
      guns: vec![
        Weapon {
          name: "Lancer Laser Destroyer".into(),
          range: 72,
          attacks: Expr::_2,
          skill: Some(3),
          strength: 14,
          ap: 4,
          damage: Expr::D6(1, 3),
          rules: vec![WeaponRule::Heavy],
          ..Default::default()
        },
        Weapon {
          name: "Icarus Rocket Pod".into(),
          range: 24,
          attacks: Expr::D3(1, 0),
          skill: Some(3),
          strength: 8,
          ap: 1,
          damage: Expr::_2,
          rules: vec![WeaponRule::AntiFly2],
          ..Default::default()
        },
        Weapon {
          name: "Ironhail Heavy Stubber".into(),
          range: 36,
          attacks: Expr::_3,
          skill: Some(3),
          strength: 4,
          ap: 0,
          damage: Expr::_1,
          rules: vec![WeaponRule::RapidFire3],
          ..Default::default()
        },
        Weapon {
          name: "Fragstorm Grenade Launcher".into(),
          range: 18,
          attacks: Expr::D6(1, 0),
          skill: Some(3),
          strength: 4,
          ap: 0,
          damage: Expr::_1,
          rules: vec![WeaponRule::Blast],
          ..Default::default()
        },
        Weapon {
          name: "Fragstorm Grenade Launcher".into(),
          range: 18,
          attacks: Expr::D6(1, 0),
          skill: Some(3),
          strength: 4,
          ap: 0,
          damage: Expr::_1,
          rules: vec![WeaponRule::Blast],
          ..Default::default()
        },
      ],
      sticks: vec![Weapon {
        name: "Armored_Hull".into(),
        range: 1,
        attacks: Expr::_3,
        skill: Some(4),
        strength: 6,
        ap: 0,
        damage: Expr::_1,
        ..Default::default()
      }],
      rules: vec![
        ModelRule::Vehicle,
        ModelRule::Smoke,
        ModelRule::Imperial,
        ModelRule::EagleOptics,
      ],
      ..Default::default()
    }],
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Unit {
  pub name: String,
  pub models: Vec<Model>,
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
  pub skill: Option<u8>,
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

  pub fn roll(&self, g: &mut PCG32) -> u8 {
    match self {
      Self::F(f) => *f,
      Self::D3(count, bonus) => {
        let mut total = *bonus;
        for _ in 0..*count {
          total += ((g.d6() as f32) / 2.0).ceil() as u8;
        }
        total
      }
      Self::D6(count, bonus) => {
        let mut total = *bonus;
        for _ in 0..*count {
          total += g.d6() as u8;
        }
        total
      }
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
  Imperial,
  LancerTank,
  EagleOptics,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WeaponRule {
  Heavy,
  AntiFly2,
  RapidFire3,
  Blast,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Effects {
  pub oath: bool,
  pub storm_of_fire: bool,
  pub devastator_doctrine: bool,
  pub attacker_movement: UnitMovement,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnitMovement {
  #[default]
  Normal,
  Advance,
  Stationary,
}
