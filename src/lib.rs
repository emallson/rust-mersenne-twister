extern crate rand;

pub use self::mt32::MTRng32;
pub use self::mt64::MTRng64;

#[cfg(target_pointer_width = "32")]
pub use self::mt32::MTRng32 as MTRng;
#[cfg(target_pointer_width = "64")]
pub use self::mt64::MTRng64 as MTRng;

pub mod mt32;
pub mod mt64;

/// Generates a `MTRng` object by seeding with bytes from `OsRng`
pub fn mersenne() -> MTRng {
    use rand::{Rng, OsRng};

    let mut os = OsRng::new().unwrap();
    MTRng::new(os.next_u64())
}
