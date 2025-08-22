#[allow(unused_imports)]

use game_math::WeaponAbility;
use game_math::WeaponAbility::*;
use game_math::WeaponAbilityBits;

fn main() {
  let mut g = randomize::PCG32::from_getrandom().unwrap();
  let u = g.next_u32();
  println!("Hello there: {u:?}");
  let bits = WeaponAbilityBits::new(&[Assault, Heavy]);
  for ability in bits.iter() {
    println!("ability: {:?}", ability);
  }
}
