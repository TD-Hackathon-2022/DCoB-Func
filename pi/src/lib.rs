use wasm_bindgen::prelude::*;
use rand::prelude::*;

const TOTAL: u32 = 1000000;

#[wasm_bindgen]
pub fn monte_carlo_pi_start() -> u32 {
    let mut rng = rand::thread_rng();
    let mut cnt: u32 = 0;
    for _i in 0..TOTAL {
        let x = rng.gen::<f64>();
        let y = rng.gen::<f64>();
        let len = (x.powi(2) + y.powi(2)).sqrt();
        if len < 1 as f64 {
            cnt += 1;
        }
    }

    cnt
}

#[cfg(test)]
mod tests {
    use crate::{monte_carlo_pi_start, TOTAL};

    #[test]
    fn cnt_should_smaller_than_total() {
        let cnt = monte_carlo_pi_start();
        assert!(cnt > 0);
        assert!(cnt < TOTAL);
    }
}