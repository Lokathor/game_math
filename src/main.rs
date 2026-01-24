#![allow(unused_imports)]

use game_math::*;

fn main() {
  let mut g = randomize::PCG32K::<1024>::from_getrandom().unwrap();
  for with_lt in [false, true] {
    for target_is_oath_target in [false, true] {
      let trials = 10000;
      let mut remaining_total = 0_u64;
      for _ in 0..trials {
        let heroes = company_heroes();
        let lt = lieutenant(Some(company_heroes()));
        let mut a = marneus_calgar(Some(if with_lt { lt } else { heroes }));
        let mut d = gladiator_lancer_w_grenades();
        let context = Context {
          range: 9,
          target_is_oath_target,
          oath_effect_wound_bonus: true,
          defender_has_cover: false,
          ..Default::default()
        };
        do_combat(&mut g, &mut a, &mut d, context);
        let remaining: u64 = d.models.iter().map(|m| m.health as u64).sum();
        remaining_total += remaining;
      }
      let average_remaining = (remaining_total as f64) / (trials as f64);
      let lt_txt = if with_lt { "w/LT" } else { "NoLT" };
      let oath = if target_is_oath_target { "Oath" } else { "NoRR" };
      println!(
        "[{lt_txt}][{oath}] Average Wounds Remaining: {average_remaining:0.3} (lower is better)"
      );
    }
  }
}

#[allow(dead_code)]
fn chaos_terminator_shooting() {
  let mut g = randomize::PCG32K::<1024>::from_getrandom().unwrap();
  for use_combi_weapon in [false, true] {
    for dark_pact_for_sustained in [false, true] {
      let trials = 100000;
      let mut remaining_total = 0_u64;
      for _ in 0..trials {
        let use_paired_weapon = false;
        let mut a = chaos_terminators(use_combi_weapon, use_paired_weapon);
        //let mut d = gladiator_lancer_w_grenades();
        let mut d = a.clone();
        let context = Context {
          range: 9,
          defender_has_cover: false,
          dark_pact_for_sustained,
          attacker_ap_modifier: 1,
          ..Default::default()
        };
        do_combat(&mut g, &mut a, &mut d, context);
        let remaining: u64 = d.models.iter().map(|m| m.health as u64).sum();
        remaining_total += remaining;
      }
      let average_remaining = (remaining_total as f64) / (trials as f64);
      println!(
        "[{wep_type}][{dark_pact_type}] Average Target Wounds Remaining: {average_remaining:0.3} (lower is better)",
        wep_type =
          if use_combi_weapon { "combi-weapon" } else { "combi-bolter" },
        dark_pact_type = if dark_pact_for_sustained {
          "dark-pact-sustan"
        } else {
          "dark-pact-lethal"
        }
      );
    }
  }
}
