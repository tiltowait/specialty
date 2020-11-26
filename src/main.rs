use rand::distributions::{Distribution, Uniform};

fn main() {
  let args: Vec<String> = std::env::args().collect();
  assert!(args.len() == 4, "Usage: ./specialty <pool> <difficulty> <target>");

  let pool: i16 = args[1].parse().expect("Pool must be a number!");
  let difficulty: i16 = args[2].parse().expect("Difficulty must be a number!");
  let target: i16 = args[3].parse().expect("Target must be a number!");

  const TRIALS: i32 = 1_000_000;

  // We calculate both the specced (10s are doubled) and unspecced probabilities
  let mut spec = 0;
  let mut unspec = 0;

  for _ in 0..TRIALS {
    let result = roll(pool, difficulty, target);

    spec += if result.0 { 1 } else { 0 };
    unspec += if result.1 { 1 } else { 0 };
  }

  let spec_ratio = spec as f64 / TRIALS as f64;
  let unspec_ratio = unspec as f64 / TRIALS as f64;

  println!("Pool {}, difficulty {}", pool, difficulty);
  println!("With specialty: {}", spec_ratio);
  println!("Without:        {}", unspec_ratio);
}

/// Returns true/false for spec/unspecced if a roll hit the target successes
fn roll(pool: i16, difficulty: i16, target: i16) -> (bool, bool) {
  let mut rng = rand::thread_rng();
  let die = Uniform::from(1..=10);

  let mut spec = 0;
  let mut unspec = 0;

  for _ in 1..=pool {
    let throw = die.sample(&mut rng);

    if throw == 1 {
      unspec -= 1;
    } else if throw == 10 {
      unspec += 1;
      spec += 1;
    } else if throw >= difficulty {
      unspec += 1;
    }
  }
  spec += unspec; // Only difference between spec and unspec are 10s, so we can
                  // afford to be a little concise here.

  (spec >= target, unspec >= target)
}
