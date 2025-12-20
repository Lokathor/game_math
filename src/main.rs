#![allow(unused_imports)]

use game_math::*;

fn main() {
  let mut g = randomize::PCG32::from_getrandom().unwrap();
  let _u = g.next_u32();
  let trials = 10000;
  let mut kills = 0.0_f64;
  for _ in 0..trials {
    let mut a = gladiator_lancer_w_grenades();
    let mut d = gladiator_lancer_w_grenades();
    do_shooting(&mut a, &mut d, 23, 0.98, Effects::default());
    if d.models[0].health == 0 {
      kills += 1.0;
    }
  }
  let kill_rate = kills / (trials as f64) * 100.0;
  println!("Kill Rate: {kill_rate:0.2}%");
}
