use super::*;

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
          skill: 3,
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
          skill: 3,
          strength: 8,
          ap: 1,
          damage: Expr::_2,
          rules: vec![WeaponRule::Anti(ModelRule::Fly, 2)],
          ..Default::default()
        },
        Weapon {
          name: "Ironhail Heavy Stubber".into(),
          range: 36,
          attacks: Expr::_3,
          skill: 3,
          strength: 4,
          ap: 0,
          damage: Expr::_1,
          rules: vec![WeaponRule::RapidFire(Expr::_3)],
          ..Default::default()
        },
        Weapon {
          name: "Fragstorm Grenade Launcher".into(),
          range: 18,
          attacks: Expr::D6(1, 0),
          skill: 3,
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
          skill: 3,
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
        skill: 4,
        strength: 6,
        ap: 0,
        damage: Expr::_1,
        ..Default::default()
      }],
      rules: vec![
        ModelRule::Vehicle,
        ModelRule::Smoke,
        ModelRule::Imperium,
        ModelRule::EagleOptics,
      ],
      ..Default::default()
    }],
  }
}

pub fn ballistus_dreadnought_krak() -> Unit {
  Unit {
    name: "Ballistus Dreadnought".into(),
    models: vec![Model {
      name: "Ballistus Dreadnought".into(),
      speed: 8,
      toughness: 10,
      armor: 2,
      health: 12,
      guns: vec![
        Weapon {
          name: "Ballistus Lascannon".into(),
          range: 48,
          attacks: Expr::_2,
          skill: 3,
          strength: 12,
          ap: 3,
          damage: Expr::D6(1, 1),
          rules: vec![],
        },
        Weapon {
          name: "Ballistus Missile Launcher (Krak)".into(),
          range: 48,
          attacks: Expr::_2,
          skill: 3,
          strength: 10,
          ap: 2,
          damage: Expr::D6(1, 0),
          rules: vec![],
        },
        Weapon {
          name: "Twin-linked Storm Bolter".into(),
          range: 24,
          attacks: Expr::_2,
          skill: 3,
          strength: 4,
          ap: 0,
          damage: Expr::_1,
          rules: vec![WeaponRule::RapidFire(Expr::_2), WeaponRule::TwinLinked],
        },
      ],
      sticks: vec![Weapon {
        name: "Armored Feet".into(),
        range: 1,
        attacks: Expr::_5,
        skill: 3,
        strength: 7,
        ap: 0,
        damage: Expr::_1,
        rules: vec![],
      }],
      rules: vec![
        ModelRule::Vehicle,
        ModelRule::Walker,
        ModelRule::Imperium,
        ModelRule::Dreadnought,
        ModelRule::BallistusStrike,
      ],
      ..Default::default()
    }],
  }
}
