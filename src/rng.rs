use rand::prelude::*;
use rand_pcg::Pcg64;
use rand_seeder::{Seeder, SipHasher};

fn main() {
    // In one line:
    let mut rng: Pcg64 = Seeder::from("stripy zebra").make_rng();
    println!("{:#?}", rng.gen::<char>());

    // If we want to be more explicit, first we create a SipRng:
    let hasher = SipHasher::from("a sailboat");
    let mut hasher_rng = hasher.into_rng();
    // (Note: hasher_rng is a full RNG and can be used directly.)

    // Now, we use hasher_rng to create a seed:
    let mut seed: <Pcg64 as SeedableRng>::Seed = Default::default();
    hasher_rng.fill(&mut seed);

    // And create our RNG from that seed:
    let mut rng = Pcg64::from_seed(seed);
    let sr: String = rng.gen::<char>().into();
    println!("{:#?}", sr);
}
