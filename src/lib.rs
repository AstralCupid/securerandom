use anyhow::anyhow;
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;

// getrandom is a wrapper that provides entropy to the crate. The current implementation uses
// getrandom::getrandom on every call, but the plan is to update this in the future so that
// randomness is fetched once at init and then securely mutated to improve both performance and
// also protect against weak/compromised rng from the operating system.
fn getrandom(dest: &mut [u8]) -> Result<()> {
    getrandom::getrandom(dest).context("unable to get entropy from the OS")
}

// rand_u64 will return a u64 that is in the range [lower_bound, upper_bound].
pub fn rand_u64(lower_bound: u64, upper_bound: u64) -> Result<u64, Error> {
    // Perform bounds checking.
    if lower_bound > upper_bound {
        return Err(anyhow!("upper_bound should be >= lower_bound"));
    }

    // Grab some random numbers.
    let mut rand_buf = [0u8; 32];
    getrandom(&mut rand_buf).context("unable to call getrandom for seed entropy")?;

    // Convert the numbers to a u64.
    let mut full_range = 0u64;
    for i in 0..rand_buf.len() - 1 {
        if i > 0 {
            full_range = full_range.wrapping_mul(255);
        }
        full_range = full_range.wrapping_add(rand_buf[i] as u64);
    }

    // Mod the u64 into range.
    let range = upper_bound - lower_bound;
    let ranged_rand = full_range % (range + 1);
    return Ok(ranged_rand + lower_bound);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ensure the bounds checking is working.
    #[test]
    fn catch_bad_bounds() {
        let t0 = match rand_u64(0, 0) {
            Ok(rand) => rand,
            Err(error) => panic!("{}", error),
        };
        assert_eq!(t0, 0);
        match rand_u64(1, 0) {
            Ok(rand) => panic!("{}", rand),
            Err(error) => {
                let err_str = format!("{}", error);
                assert_eq!(err_str, "upper_bound should be >= lower_bound")
            }
        };
    }

    // Check for simple off-by-one errors.
    #[test]
    fn catch_off_by_one() {
        // Try range [1,1]
        let mut one = false;
        for _ in 0..1000 {
            let result = match rand_u64(1, 1) {
                Ok(result) => result,
                Err(error) => {
                    panic!("{}", error);
                }
            };
            if result == 1 {
                one = true
            } else {
                panic!("{}", result);
            }
        }
        if !one {
            panic!("one was not hit");
        }

        // Try range [0,1]
        let mut zero = false;
        let mut one = false;
        for _ in 0..1000 {
            let result = match rand_u64(0, 1) {
                Ok(result) => result,
                Err(error) => {
                    panic!("{}", error);
                }
            };
            if result == 0 {
                zero = true
            } else if result == 1 {
                one = true
            } else {
                panic!("{}", result);
            }
        }
        if !zero {
            panic!("zero was not hit");
        }
        if !one {
            panic!("one was not hit");
        }

        // Try range [1,2]
        let mut one = false;
        let mut two = false;
        for _ in 0..1000 {
            let result = match rand_u64(1, 2) {
                Ok(result) => result,
                Err(error) => {
                    panic!("{}", error);
                }
            };
            if result == 1 {
                one = true
            } else if result == 2 {
                two = true
            } else {
                panic!("{}", result);
            }
        }
        if !one {
            panic!("one was not hit");
        }
        if !two {
            panic!("two was not hit");
        }

        // Try range [0,2]
        let mut zero = false;
        let mut one = false;
        let mut two = false;
        for _ in 0..1000 {
            let result = match rand_u64(0, 2) {
                Ok(result) => result,
                Err(error) => {
                    panic!("{}", error);
                }
            };
            if result == 0 {
                zero = true
            } else if result == 1 {
                one = true
            } else if result == 2 {
                two = true
            } else {
                panic!("{}", result);
            }
        }
        if !zero {
            panic!("zero was not hit");
        }
        if !one {
            panic!("one was not hit");
        }
        if !two {
            panic!("two was not hit");
        }
    }

    // Perform a statistical test to make sure numbers are distributing evenly. This isn't a
    // cryptographic test, it's a test  to look for more basic mistakes.
    #[test]
    fn check_rand_distribution() {
        let mut one = 0u64;
        let mut two = 0u64;
        let mut three = 0u64;
        for _ in 0..1_000_000 {
            match rand_u64(1,3) {
                Ok(result) => {
                    if result == 1 {
                        one += 1;
                    } else if result == 2 {
                        two += 1;
                    } else if result == 3 {
                        three += 1;
                    } else {
                        panic!("{}", result);
                    }
                },
                Err(error) => {
                    panic!("{}", error);
                }
            }
        }

        // Check that the distribution is good. A typical spread will put more than 332_000 into
        // each bucket, it's extremely unlikely that you'd ever see a number get 320_000 or less by
        // chance.
        if one < 320_000 || two < 320_000 || three < 320_000 {
            panic!("{} - {} - {}", one, two, three);
        }
    }
}
