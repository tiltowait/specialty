use rand::distributions::{Distribution, Uniform};

fn main() {
  let args: Vec<String> = std::env::args().collect();
  assert!(args.len() == 4, "Usage: ./specialty <pool> <difficulty> <target>");

  let pool: i16 = args[1].parse().expect("Pool must be a number!");
  let difficulty: i16 = args[2].parse().expect("Difficulty must be a number!");
  let target: i16 = args[3].parse().expect("Target must be a number!");

  let trials = 1_000_000;

  let mut successful_trials = 0;
  for _ in 0..trials {
    successful_trials += if roll(pool, difficulty, target) { 1 } else { 0 };
  }

  let ratio = successful_trials as f64 / trials as f64;
  println!("{} diff {}: {}", pool, difficulty, ratio);
}

fn roll(pool: i16, difficulty: i16, target: i16) -> bool {
  let mut rng = rand::thread_rng();
  let die = Uniform::from(1..=10);

  let mut successes = 0;

  for _ in 1..=pool {
    let throw = die.sample(&mut rng);

    if throw == 1 {
      successes -= 1;
    } else if throw == 10 {
      successes += 2;
    } else if throw >= difficulty {
      successes += 1;
    }
  }
  successes >= target
}
