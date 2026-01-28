use std::vec;

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
      leadership: 6,
      oc: 3,
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
        ModelRule::OathOfMoment,
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
      leadership: 6,
      oc: 4,
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
        ModelRule::OathOfMoment,
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
    leadership: 6,
    oc: 1,
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

pub fn rubric_marines(count: u8, use_flamers: bool, icon: bool) -> Unit {
  let mut out = Unit {
    name: "Rubric Marines".into(),
    models: Vec::new(),
    starting_models: count,
  };

  let aspiring_sorcerer = Model {
    armor: 3,
    name: "Aspiring Sorcerer".into(),
    speed: 6,
    toughness: 4,
    invuln: Some(5),
    fnp: None,
    fnp_dev: None,
    health: 2,
    starting_health: 2,
    leadership: 6,
    oc: 2,
    rules: vec![ModelRule::Infantry, ModelRule::Chaos, ModelRule::Battleline],
    guns: vec![
      Weapon {
        name: "Warpflame Pistol".into(),
        range: 12,
        attacks: Expr::D6(1, 0),
        skill: 0,
        strength: 3,
        ap: 1,
        damage: Expr::_1,
        rules: vec![
          WeaponRule::Pistol,
          WeaponRule::IgnoresCover,
          WeaponRule::Torrent,
        ],
      },
      Weapon {
        name: "Malefic Curse".into(),
        range: 24,
        attacks: Expr::_3,
        skill: 3,
        strength: 4,
        ap: 3,
        damage: Expr::_1,
        rules: vec![
          WeaponRule::Anti(ModelRule::Infantry, 4),
          WeaponRule::DevastatingWounds,
          WeaponRule::Psychic,
        ],
      },
    ],
    sticks: vec![Weapon {
      name: "Force Weapon".into(),
      range: 1,
      attacks: Expr::_3,
      skill: 3,
      strength: 6,
      ap: 1,
      damage: Expr::D3(1, 0),
      rules: vec![WeaponRule::Psychic],
    }],
  };

  let soulreaper = Model {
    armor: 3,
    name: "Rubric Marine".into(),
    speed: 6,
    toughness: 4,
    invuln: Some(5),
    fnp: None,
    fnp_dev: None,
    health: 2,
    starting_health: 2,
    leadership: 6,
    oc: 2,
    rules: vec![ModelRule::Infantry, ModelRule::Chaos, ModelRule::Battleline],
    guns: vec![Weapon {
      name: "Soulreaper Cannon".into(),
      range: 24,
      attacks: Expr::F(6),
      skill: 3,
      strength: 6,
      ap: 2,
      damage: Expr::_1,
      rules: vec![WeaponRule::DevastatingWounds],
    }],
    sticks: vec![Weapon {
      name: "Close Combat Weapon".into(),
      range: 1,
      attacks: Expr::_2,
      skill: 3,
      strength: 4,
      ap: 0,
      damage: Expr::_1,
      rules: vec![],
    }],
  };

  let basic_gun = if use_flamers {
    Weapon {
      name: "Warpflamer".into(),
      range: 12,
      attacks: Expr::D6(1, 0),
      skill: 0,
      strength: 4,
      ap: 1,
      damage: Expr::_1,
      rules: vec![WeaponRule::IgnoresCover, WeaponRule::Torrent],
    }
  } else {
    Weapon {
      name: "Inferno Boltgun".into(),
      range: 24,
      attacks: Expr::F(2),
      skill: 3,
      strength: 4,
      ap: 2,
      damage: Expr::_1,
      rules: vec![],
    }
  };
  let basic_marine = Model {
    armor: 3,
    name: "Rubric Marine".into(),
    speed: 6,
    toughness: 4,
    invuln: Some(5),
    fnp: None,
    fnp_dev: None,
    health: 2,
    starting_health: 2,
    leadership: 6,
    oc: 2,
    rules: vec![ModelRule::Infantry, ModelRule::Chaos, ModelRule::Battleline],
    guns: vec![basic_gun],
    sticks: vec![Weapon {
      name: "Close Combat Weapon".into(),
      range: 1,
      attacks: Expr::_2,
      skill: 3,
      strength: 4,
      ap: 0,
      damage: Expr::_1,
      rules: vec![],
    }],
  };

  out.models.push(aspiring_sorcerer);
  out.models.push(soulreaper);
  while out.models.len() < usize::from(count) {
    out.models.push(basic_marine.clone());
  }

  if icon {
    out.models.iter_mut().for_each(|m| {
      m.guns.iter_mut().for_each(|g| g.rules.push(WeaponRule::IgnoresCover))
    });
  }

  out
}

pub fn inceptor_bolter(count: u8) -> Unit {
  let mut out =
    Unit { name: "Inceptors".into(), starting_models: count, models: vec![] };

  let model = Model {
    name: "Inceptor".into(),
    speed: 10,
    toughness: 6,
    armor: 3,
    health: 3,
    starting_health: 4,
    leadership: 6,
    oc: 1,
    guns: vec![Weapon {
      name: "Assault Bolter".into(),
      range: 18,
      attacks: Expr::_3,
      skill: 3,
      strength: 5,
      ap: 1,
      damage: Expr::D6(2, 0),
      rules: vec![
        WeaponRule::Assault,
        WeaponRule::Pistol,
        WeaponRule::SustainedHits(Expr::_2),
        WeaponRule::TwinLinked,
      ],
    }],
    sticks: vec![Weapon {
      name: "Close Combat Weapon".into(),
      range: 1,
      attacks: Expr::_3,
      skill: 3,
      strength: 4,
      ap: 0,
      damage: Expr::_1,
      rules: vec![],
    }],
    rules: vec![
      ModelRule::MeteoricDescent,
      ModelRule::DeepStrike,
      ModelRule::OathOfMoment,
      ModelRule::Infantry,
      ModelRule::Fly,
      ModelRule::Imperium,
      ModelRule::Gravis,
    ],
    ..Default::default()
  };

  while out.models.len() < usize::from(out.starting_models) {
    out.models.push(model.clone());
  }

  out
}

pub fn inceptor_plasma(count: u8, overcharge: bool) -> Unit {
  let mut out =
    Unit { name: "Inceptors".into(), starting_models: count, models: vec![] };

  let mut model = Model {
    name: "Inceptor".into(),
    speed: 10,
    toughness: 6,
    armor: 3,
    health: 3,
    starting_health: 4,
    leadership: 6,
    oc: 1,
    guns: vec![Weapon {
      name: "Plasma Exterminator".into(),
      range: 18,
      attacks: Expr::_2,
      skill: 3,
      strength: if overcharge { 8 } else { 7 },
      ap: if overcharge { 3 } else { 2 },
      damage: if overcharge { Expr::_3 } else { Expr::_2 },
      rules: vec![
        WeaponRule::Assault,
        WeaponRule::Pistol,
        WeaponRule::TwinLinked,
      ],
    }],
    sticks: vec![Weapon {
      name: "Close Combat Weapon".into(),
      range: 1,
      attacks: Expr::_3,
      skill: 3,
      strength: 4,
      ap: 0,
      damage: Expr::_1,
      rules: vec![],
    }],
    rules: vec![
      ModelRule::MeteoricDescent,
      ModelRule::DeepStrike,
      ModelRule::OathOfMoment,
      ModelRule::Infantry,
      ModelRule::Fly,
      ModelRule::Imperium,
      ModelRule::Gravis,
    ],
    ..Default::default()
  };
  if overcharge {
    model.guns[0].rules.push(WeaponRule::Hazardous);
  }

  while out.models.len() < usize::from(out.starting_models) {
    out.models.push(model.clone());
  }

  out
}

pub fn company_heroes() -> Unit {
  let bolt_pistol = Weapon {
    name: "Bolt Pistol".into(),
    range: 12,
    attacks: Expr::_1,
    skill: 3,
    strength: 4,
    ap: 0,
    damage: Expr::_1,
    rules: vec![WeaponRule::Pistol],
  };
  let clone_combat = Weapon {
    name: "Close Combat Weapon".into(),
    range: 1,
    attacks: Expr::_5,
    skill: 3,
    strength: 4,
    ap: 0,
    damage: Expr::_1,
    rules: vec![],
  };

  let ancient = Model {
    name: "Ancient".into(),
    speed: 6,
    toughness: 4,
    armor: 3,
    health: 4,
    starting_health: 4,
    leadership: 6,
    oc: 1,
    guns: vec![
      Weapon {
        name: "Bolt Rifle".into(),
        range: 24,
        attacks: Expr::_2,
        skill: 3,
        strength: 4,
        ap: 1,
        damage: Expr::_1,
        rules: vec![],
      },
      bolt_pistol.clone(),
    ],
    sticks: vec![clone_combat.clone()],
    rules: vec![
      ModelRule::Ancient,
      ModelRule::Infantry,
      ModelRule::Grenades,
      ModelRule::Imperium,
      ModelRule::Tacticus,
      ModelRule::CommandSquad,
      ModelRule::OathOfMoment,
    ],
    ..Default::default()
  };

  let company_champion = Model {
    name: "Company Champion".into(),
    speed: 6,
    toughness: 4,
    armor: 3,
    health: 4,
    starting_health: 4,
    leadership: 6,
    oc: 1,
    guns: vec![bolt_pistol.clone()],
    sticks: vec![Weapon {
      name: "Master-crafted power weapon".into(),
      range: 1,
      attacks: Expr::_6,
      skill: 2,
      strength: 5,
      ap: 2,
      damage: Expr::_2,
      rules: vec![WeaponRule::Precision],
    }],
    rules: vec![
      ModelRule::Infantry,
      ModelRule::Grenades,
      ModelRule::Imperium,
      ModelRule::Tacticus,
      ModelRule::CommandSquad,
      ModelRule::OathOfMoment,
    ],
    ..Default::default()
  };

  let rifle_vet = Model {
    name: "Company Vetern".into(),
    speed: 6,
    toughness: 4,
    armor: 3,
    health: 4,
    starting_health: 4,
    leadership: 6,
    oc: 1,
    guns: vec![
      Weapon {
        name: "Master-crafted Bolt Rifle".into(),
        range: 24,
        attacks: Expr::_2,
        skill: 2,
        strength: 4,
        ap: 1,
        damage: Expr::_2,
        rules: vec![
          WeaponRule::DevastatingWounds,
          WeaponRule::RapidFire(Expr::_1),
        ],
      },
      bolt_pistol.clone(),
    ],
    sticks: vec![clone_combat.clone()],
    rules: vec![
      ModelRule::Infantry,
      ModelRule::Grenades,
      ModelRule::Imperium,
      ModelRule::Tacticus,
      ModelRule::CommandSquad,
      ModelRule::OathOfMoment,
    ],
    ..Default::default()
  };

  let heavy_vet = Model {
    name: "Company Vetern".into(),
    speed: 6,
    toughness: 4,
    armor: 3,
    health: 4,
    starting_health: 4,
    leadership: 6,
    oc: 1,
    guns: vec![
      Weapon {
        name: "Master-crafted Heavy Bolter".into(),
        range: 36,
        attacks: Expr::_3,
        skill: 3,
        strength: 5,
        ap: 1,
        damage: Expr::_3,
        rules: vec![WeaponRule::Heavy, WeaponRule::SustainedHits(Expr::_2)],
      },
      bolt_pistol.clone(),
    ],
    sticks: vec![clone_combat.clone()],
    rules: vec![
      ModelRule::Infantry,
      ModelRule::Grenades,
      ModelRule::Imperium,
      ModelRule::Tacticus,
      ModelRule::CommandSquad,
      ModelRule::OathOfMoment,
    ],
    ..Default::default()
  };

  Unit {
    name: "Company Heroes".into(),
    models: vec![ancient, company_champion, rifle_vet, heavy_vet],
    starting_models: 4,
  }
}

pub fn marneus_calgar(bodyguard: Option<Unit>) -> Unit {
  let guard = Model {
    name: "Victrix Honor Guard".into(),
    speed: 6,
    toughness: 4,
    armor: 2,
    invuln: Some(4),
    health: 3,
    starting_health: 3,
    leadership: 6,
    oc: 1,
    guns: Vec::new(),
    sticks: vec![Weapon {
      name: "Victrix Power Sword".into(),
      range: 1,
      attacks: Expr::_5,
      skill: 2,
      strength: 5,
      ap: 2,
      damage: Expr::_2,
      rules: Vec::new(),
    }],
    rules: vec![ModelRule::Imperium, ModelRule::Infantry],
    ..Default::default()
  };

  let cally = Model {
    name: "Marneus Calgar".into(),
    speed: 6,
    toughness: 6,
    armor: 2,
    invuln: Some(4),
    health: 6,
    starting_health: 6,
    leadership: 6,
    oc: 1,
    guns: vec![Weapon {
      name: "Gauntlets of Ultramar".into(),
      range: 18,
      attacks: Expr::_4,
      skill: 2,
      strength: 4,
      ap: 2,
      damage: Expr::_2,
      rules: vec![WeaponRule::Pistol, WeaponRule::TwinLinked],
    }],
    sticks: vec![Weapon {
      name: "Gauntlets of Ultramar".into(),
      range: 1,
      attacks: Expr::_6,
      skill: 2,
      strength: 8,
      ap: 3,
      damage: Expr::_3,
      rules: vec![WeaponRule::TwinLinked],
    }],
    rules: vec![
      ModelRule::Infantry,
      ModelRule::Imperium,
      ModelRule::Character,
      ModelRule::EpicHero,
      ModelRule::Gravis,
      ModelRule::ChapterMaster,
      ModelRule::OathOfMoment,
    ],
    ..Default::default()
  };

  if let Some(mut guard_unit) = bodyguard {
    guard_unit.models.push(cally);
    guard_unit.models.push(guard.clone());
    guard_unit.models.push(guard);
    Unit {
      name: format!("Marneus Calgar Leading {}", &guard_unit.name),
      models: guard_unit.models,
      starting_models: guard_unit.starting_models + 3,
    }
  } else {
    Unit {
      name: "Marneus Calgar".into(),
      models: vec![cally, guard.clone(), guard],
      starting_models: 3,
    }
  }
}

pub fn lieutenant(bodyguard: Option<Unit>) -> Unit {
  let lt = Model {
    name: "Lieutenant".into(),
    speed: 6,
    toughness: 4,
    armor: 3,
    health: 4,
    starting_health: 4,
    leadership: 6,
    oc: 1,
    guns: vec![
      Weapon {
        name: "Heavy Bolt Pistol".into(),
        range: 18,
        attacks: Expr::_1,
        skill: 2,
        strength: 4,
        ap: 1,
        damage: Expr::_1,
        rules: vec![WeaponRule::Pistol],
      },
      Weapon {
        name: "Plasma Pistol (Standard)".into(),
        range: 12,
        attacks: Expr::_1,
        skill: 2,
        strength: 7,
        ap: 2,
        damage: Expr::_1,
        rules: vec![WeaponRule::Pistol],
      },
    ],
    sticks: vec![Weapon {
      name: "Power Fist".into(),
      range: 1,
      attacks: Expr::_4,
      skill: 2,
      strength: 8,
      ap: 2,
      damage: Expr::_2,
      rules: vec![],
    }],
    rules: vec![
      ModelRule::Infantry,
      ModelRule::Character,
      ModelRule::Grenades,
      ModelRule::Imperium,
      ModelRule::Tacticus,
      ModelRule::Lieutenant,
      ModelRule::TacticalPrecision,
      ModelRule::OathOfMoment,
    ],
    ..Default::default()
  };

  if let Some(mut guard_unit) = bodyguard {
    guard_unit.models.push(lt);
    Unit {
      name: format!("Lieutenant Leading {}", &guard_unit.name),
      models: guard_unit.models,
      starting_models: guard_unit.starting_models + 1,
    }
  } else {
    Unit { name: "Lieutenant".into(), models: vec![lt], starting_models: 1 }
  }
}

pub fn assault_intercessors(count: u8) -> Unit {
  let basic = Model {
    name: "Assault Intercessor".into(),
    speed: 6,
    toughness: 4,
    armor: 3,
    health: 2,
    starting_health: 2,
    leadership: 6,
    oc: 2,
    guns: vec![Weapon {
      name: "Heavy Bolt Pistol".into(),
      range: 18,
      attacks: Expr::_1,
      skill: 3,
      strength: 4,
      ap: 1,
      damage: Expr::_1,
      rules: vec![WeaponRule::Pistol],
    }],
    sticks: vec![Weapon {
      name: "Chainsword".into(),
      range: 1,
      attacks: Expr::_4,
      skill: 3,
      strength: 4,
      ap: 1,
      damage: Expr::_1,
      rules: vec![],
    }],
    rules: vec![
      ModelRule::Battleline,
      ModelRule::Infantry,
      ModelRule::Grenades,
      ModelRule::Imperium,
      ModelRule::Tacticus,
      ModelRule::ShockAssault,
      ModelRule::OathOfMoment,
    ],
    ..Default::default()
  };

  let sergeant = Model {
    guns: vec![Weapon {
      name: "Plasma Pistol (Standard)".into(),
      range: 12,
      attacks: Expr::_1,
      skill: 2,
      strength: 7,
      ap: 2,
      damage: Expr::_1,
      rules: vec![WeaponRule::Pistol],
    }],
    sticks: vec![Weapon {
      name: "Power Fist".into(),
      range: 1,
      attacks: Expr::_3,
      skill: 3,
      strength: 8,
      ap: 2,
      damage: Expr::_2,
      rules: vec![],
    }],
    ..basic.clone()
  };

  let mut out = Unit {
    name: "Assault Intercesors".into(),
    models: Vec::new(),
    starting_models: count,
  };

  while out.models.len() < (usize::from(count) - 1) {
    out.models.push(basic.clone())
  }
  out.models.push(sergeant);

  out
}
