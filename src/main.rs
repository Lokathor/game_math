use randomize::{Gen32, PCG32K};

fn main() {
  let g = &mut *Box::new(PCG32K::<1024>::from_getrandom().unwrap());
  const TRIALS: u64 = 1_000_000;

  let mut total = 0_f64;
  for _trial in 0..1_000_000 {
    total += do_combi(g, false) as f64;
  }
  println!("Long Range Combi: {}", total / (TRIALS as f64));

  let mut total = 0_f64;
  for _trial in 0..1_000_000 {
    total += do_storm(g, false) as f64;
  }
  println!("Long Range Storm: {}", total / (TRIALS as f64));

  let mut total = 0_f64;
  for _trial in 0..1_000_000 {
    total += do_combi(g, true) as f64;
  }
  println!("Short Range Combi: {}", total / (TRIALS as f64));

  let mut total = 0_f64;
  for _trial in 0..1_000_000 {
    total += do_storm(g, true) as f64;
  }
  println!("Short Range Storm: {}", total / (TRIALS as f64));
}

pub fn do_combi(g: &mut impl Gen32, half_range: bool) -> i32 {
  let attacks = if half_range { 2 } else { 1 };
  let mut hits = 0;
  for _ in 0..attacks {
    let attack_roll = g.d6();
    if attack_roll >= 4 {
      hits += 1;
    }
  }
  let mut wounds = 0;
  for _ in 0..hits {
    let wound_roll = g.d6();
    if wound_roll >= 4 {
      wounds += 1;
    }
  }
  let damage = wounds;
  return damage;
}

pub fn do_storm(g: &mut impl Gen32, half_range: bool) -> i32 {
  let attacks = if half_range { 4 } else { 2 };
  let mut hits = 0;
  for _ in 0..attacks {
    let attack_roll = g.d6();
    if attack_roll >= 3 {
      hits += 1;
    }
  }
  let mut wounds = 0;
  for _ in 0..hits {
    let wound_roll = g.d6();
    if wound_roll >= 4 {
      wounds += 1;
    }
  }
  let mut damage = 0;
  for _ in 0..wounds {
    let save_roll = g.d6();
    if save_roll < 3 {
      damage += 1;
    }
  }
  return damage;
}
