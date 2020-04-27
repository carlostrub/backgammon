extern crate rand;

/// roll generates two random numbers between 1 and 6, replicating a perfect dice. We use the
/// operating systems random number generator.
fn roll() -> (u8, u8) {
    use self::rand::{OsRng, Rng};
    let mut rng = match OsRng::new() {
        Ok(g) => g,
        Err(e) => panic!("Failed to obtain OS RNG: {}", e),
    };

    return (rng.gen_range(1, 7), rng.gen_range(1, 7));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roll_test() {
        let d = roll();
        assert!(d.0 > 0);
        assert!(d.0 < 7);
        assert!(d.1 > 0);
        assert!(d.1 < 7);
    }

    #[test]
    fn roll_fair() {
        let mut sum: u32 = 0;

        for x in 0..10000 {
            let d = roll();
            sum += (d.0 + d.1) as u32;
        }

        let average = (sum as f64) / 20000.;
        assert!(average < 3.6);
        assert!(average > 3.4);
    }
}
