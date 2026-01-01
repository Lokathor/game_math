#![allow(unused_imports)]

use game_math::*;

fn main() {
  let mut g = randomize::PCG32K::<1024>::from_getrandom().unwrap();
  let _u = g.next_u32();
  for use_combi_weapon in [false, true] {
    for dark_pact_for_sustained in [false, true] {
      let trials = 100000;
      let mut remaining_total = 0_u64;
      for _ in 0..trials {
        let use_paired_weapon = false;
        let mut a = chaos_terminators(use_combi_weapon, use_paired_weapon);
        //let mut d = gladiator_lancer_w_grenades();
        let mut d = a.clone();
        let range = 9;
        let context = Context {
          defender_has_cover: false,
          dark_pact_for_sustained,
          attacker_ap_bonus: 0,
          ..Default::default()
        };
        do_shooting(&mut g, &mut a, &mut d, range, context);
        let remaining: u64 = d.models.iter().map(|m| m.health as u64).sum();
        if remaining > 16 {
          panic!("{}", remaining);
        }
        remaining_total += remaining;
      }
      let average_remaining = (remaining_total as f64) / (trials as f64);
      println!(
        "[use_combi:{use_combi_weapon}][dark_pact_sustain:{dark_pact_for_sustained}] Average Target Wounds Remaining: {average_remaining:0.3} (lower is better)"
      );
    }
  }
}
