use randomize::Gen32;

#[derive(Debug, Clone, Copy, Default)]
pub enum Dice {
  Flat(i32),
  XD3(i32),
  XD6(i32),
  XD3P(i32, i32),
  XD6P(i32, i32),
  #[default]
  Zero,
}
impl Dice {
  pub fn roll(self, g: &mut impl Gen32) -> i32 {
    match self {
      Dice::Flat(x) => x,
      Dice::XD3(x) => (0..x).map(|_| g.d6() / 2).sum(),
      Dice::XD6(x) => (0..x).map(|_| g.d6()).sum(),
      Dice::XD3P(x, p) => (0..x).map(|_| g.d6() / 2).sum::<i32>() + p,
      Dice::XD6P(x, p) => (0..x).map(|_| g.d6()).sum::<i32>() + p,
      Dice::Zero => 0,
    }
  }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ToHit {
  Roll(i32),
  #[default]
  Torrent,
}

/// Rerolls allowed.
///
/// This isn't part of a weapon profile directly, it has to be handled at the
/// model/unit level because of sitautions where there's only one reroll allowed
/// per attack sequence.
#[derive(Debug, Clone, Copy, Default)]
pub enum Reroll {
  #[default]
  Nothing,
  Ones,
  Misses,
  Any,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct WeaponProfile {
  /// Melee weapons are just range 0.
  pub range: i32,
  pub num_attacks: Dice,
  pub to_hit: ToHit,
  pub strength: i32,
  pub ap: i32,
  pub damage: Dice,
  //
  pub assault: bool,
  pub rapid_fire: i32,
  pub ignores_cover: bool,
  pub twin_linked: bool,
  pub indirect_fire: bool,
  pub precision: bool,
  pub hazardous: bool,
  pub devastating: bool,
  pub pistol: bool,
  pub lethal_hits: bool,
  pub lance: bool,
  pub blast: bool,
  pub melta: i32,
  pub heavy: bool,
  pub sustained_hits: i32,
  pub extra_attacks: bool,
  pub anti: i32,
  pub hit_crits: i32,
  pub wound_crits: i32,
}
impl WeaponProfile {
  pub fn determine_attack_count(
    &self, g: &mut impl Gen32, distance: i32, target_model_count: i32,
  ) -> i32 {
    if distance > self.range {
      return 0;
    }
    let mut attack_count = self.num_attacks.roll(g);
    if distance <= self.range / 2 {
      attack_count += self.rapid_fire;
    }
    if self.blast {
      attack_count += target_model_count / 5;
    }
    attack_count
  }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DefenderProfile {
  pub toughness: i32,
  pub armor_save: i32,
  pub invuln_save: i32,
  pub feel_no_pain: i32,
  pub wounds: i32,
  //
  pub has_cover: bool,
  pub minus_hit: i32,
  pub minus_wound: i32,
  /// Gives -1 to wound only when the attacker strength exceeds defender's
  /// toughness.
  pub iron_fortitude: bool,
  pub minus_damage: i32,
  pub half_damage: bool,
  pub change_damage_to_1: bool,
}
