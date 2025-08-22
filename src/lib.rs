
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, Hash)]
pub enum Expr {
  Flat(i32),
  D3,
  D6,
  XD3P(u8,u8),
  XD6P(u8,u8),
}
impl core::cmp::PartialOrd for Expr {
  fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
    self.max_value().partial_cmp(&other.max_value()).map(|x|x.reverse())
  }
}
impl Expr {
  pub fn max_value(self) -> i32 {
    match self {
      Expr::Flat(i) => i,
      Expr::D3 => 3,
      Expr::D6 => 6,
      Expr::XD3P(x, p) => (x as i32)*3+(p as i32),
      Expr::XD6P(x, p) => (x as i32)*6+(p as i32),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum WeaponAbility {
  AntiFly2,
  AntiInfantry4,
  Assault,
  Blast,
  DevastatingWounds,
  ExtraAttacks,
  Hazardous,
  Hazardous2,
  Heavy,
  IgnoresCover,
  LethalHits,
  Pistol,
  Precision,
  Psychic,
  RapidFire1,
  RapidFire2,
  RapidFire3,
  RapidFire4,
  SustainedHits1,
  SustainedHits2,
  SustainedHitsD3,
  Torrent,
  TwinLinked,
}
impl WeaponAbility {
  const fn bitmask(self) -> u64 {
    1 << (self as u64)
  }
 fn try_from_bitmask(bit: u64) -> Option<Self> {
    assert!(bit.count_ones() == 1);
    let x: u8 = bit.trailing_zeros() as u8;
    if x <= (Self::TwinLinked as u8) {
      Some(unsafe { core::mem::transmute(x) })
    } else {
      None
    }
  }
}
#[test]
fn test_twin_linked_tag_value() {
  assert!((WeaponAbility::TwinLinked as u8) < 64);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WeaponAbilityBits(u64);
impl WeaponAbilityBits {
  pub fn contains(self, ability: WeaponAbility) -> bool {
    (self.0 & ability.bitmask()) != 0
  }
  pub fn new(abilities: &[WeaponAbility]) -> Self {
    let mut x = 0;
    for ability in abilities.iter() {
      x |= ability.bitmask();
    }
    Self(x)
  }
  pub fn iter(self) -> impl Iterator<Item=WeaponAbility> + Clone {
    let mut i = 0;
    let bits = self.0;
    core::iter::from_fn(move ||{
      while i < 64 {
        let b = bits&(1<<i);
        i += 1;
        if b != 0 {
          return WeaponAbility::try_from_bitmask(b);
        } else {
          continue
        }
      }
      None
    })
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Weapon {
  // Note(Lokathor): field order affects the sorting order from the derive, so we sort by damage first, then after that just a sort by Name is fine enough.
  pub damage: Expr,
  pub name: String,
  pub attacks: Expr,
  pub skill: u8,
  pub strength: u8,
  pub ap: u8,
  pub abilities: WeaponAbilityBits,
}
