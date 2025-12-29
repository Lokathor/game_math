use super::*;

pub fn gladiator_lancer_w_grenades() -> Unit {
  Unit {
    name: "Galdiator Lancer".into(),
    starting_models: 1,
    models: vec![Model {
      name: "Gladiator Lancer".into(),
      speed: 10,
      toughness: 10,
      armor: 3,
      health: 12,
      starting_health: 12,
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
    starting_models: 1,
    models: vec![Model {
      name: "Ballistus Dreadnought".into(),
      speed: 8,
      toughness: 10,
      armor: 2,
      health: 12,
      starting_health: 12,
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

pub fn chaos_terminators(
  use_combi_weapon: bool, use_paired_weapon: bool,
) -> Unit {
  let gun_str = if use_combi_weapon { "combi-weapon" } else { "combi-bolter" };
  let paired_str = if use_paired_weapon { "paired-acursed" } else { "no-pair" };
  let mut unit = Unit {
    name: format!("Chaos Terminators ({gun_str},{paired_str})"),
    models: Vec::new(),
    starting_models: 5,
  };

  let terminator = Model {
    name: "Chaos Terminator".into(),
    speed: 5,
    toughness: 5,
    armor: 2,
    invuln: Some(4),
    fnp: None,
    fnp_dev: None,
    health: 3,
    starting_health: 3,
    guns: vec![],
    sticks: vec![],
    rules: vec![
      ModelRule::DarkPacts,
      ModelRule::DeepStrike,
      ModelRule::Chaos,
      ModelRule::Infantry,
      ModelRule::TerminatorDespoilers,
    ],
  };

  let common_gun = if use_combi_weapon {
    Weapon {
      name: "Combi-weapon".into(),
      range: 24,
      attacks: Expr::_1,
      skill: 4,
      strength: 4,
      ap: 0,
      damage: Expr::_1,
      rules: vec![
        WeaponRule::DevastatingWounds,
        WeaponRule::Anti(ModelRule::Infantry, 4),
      ],
    }
  } else {
    Weapon {
      name: "Combi-bolter".into(),
      range: 24,
      attacks: Expr::_2,
      skill: 3,
      strength: 4,
      ap: 0,
      damage: Expr::_1,
      rules: vec![WeaponRule::RapidFire(Expr::_2)],
    }
  };

  let reaper_autocannon = Weapon {
    name: "Reaper Autocannon".into(),
    range: 36,
    attacks: Expr::_4,
    skill: 3,
    strength: 7,
    ap: 1,
    damage: Expr::_1,
    rules: vec![
      WeaponRule::DevastatingWounds,
      WeaponRule::SustainedHits(Expr::_1),
    ],
  };
  let power_fist = Weapon {
    name: "Power Fist".into(),
    range: 1,
    attacks: Expr::_3,
    skill: 3,
    strength: 8,
    ap: 2,
    damage: Expr::_2,
    rules: vec![],
  };
  let chain_fist = Weapon {
    name: "Chain Fist".into(),
    range: 1,
    attacks: Expr::_3,
    skill: 4,
    strength: 8,
    ap: 2,
    damage: Expr::_2,
    rules: vec![WeaponRule::Anti(ModelRule::Vehicle, 3)],
  };
  let acursed_weapon = Weapon {
    name: "Acursed Weapon".into(),
    range: 1,
    attacks: Expr::_4,
    skill: 3,
    strength: 5,
    ap: 2,
    damage: Expr::_1,
    rules: vec![],
  };
  let paired_acursed_weapon = Weapon {
    name: "Paired Acursed Weapon".into(),
    range: 1,
    attacks: Expr::_4,
    skill: 3,
    strength: 5,
    ap: 2,
    damage: Expr::_1,
    rules: vec![],
  };

  unit.models.push(Model {
    guns: vec![reaper_autocannon],
    sticks: vec![power_fist.clone()],
    ..terminator.clone()
  });
  unit.models.push(Model {
    guns: vec![common_gun.clone()],
    sticks: vec![power_fist.clone()],
    ..terminator.clone()
  });
  unit.models.push(Model {
    guns: vec![common_gun.clone()],
    sticks: vec![power_fist.clone()],
    ..terminator.clone()
  });
  unit.models.push(Model {
    guns: vec![common_gun.clone()],
    sticks: vec![chain_fist],
    ..terminator.clone()
  });
  unit.models.push(Model { ..terminator.clone() });
  if use_paired_weapon {
    unit.models[4].sticks.push(paired_acursed_weapon);
  } else {
    unit.models[4].guns.push(common_gun.clone());
    unit.models[4].sticks.push(acursed_weapon);
  };

  unit
}
