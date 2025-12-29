#![allow(unused_imports)]

use game_math::*;

fn main() {
  let mut g = randomize::PCG32K::<1024>::from_getrandom().unwrap();
  let _u = g.next_u32();
  let mut remaining_total = 0.0_f64;
  let trials = 100000_usize;
  for _ in 0..trials {
    let use_combi_weapon = true;
    let use_paired_weapon = false;
    let mut a = chaos_terminators(use_combi_weapon, use_paired_weapon);
    let mut d = chaos_terminators(use_combi_weapon, use_paired_weapon);
    let range = 9;
    let context = Context { defender_has_cover: false, ..Default::default() };
    do_shooting(&mut g, &mut a, &mut d, range, context);
    let remaining: u32 = d.models.iter().map(|m| m.health as u32).sum();
    remaining_total += remaining as f64;
  }
  let average_remaining = remaining_total / (trials as f64);
  println!("Average Remaining: {average_remaining:0.3}");
}
