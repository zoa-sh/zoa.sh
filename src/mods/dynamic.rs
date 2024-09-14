// PATH: src/mods/dynamic.rs
use rand::Rng;

// Randomly Generated Stars
pub fn generate_stars(count: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut stars = String::with_capacity(count * 60);

    for _ in 0..count {
        let x = rng.gen_range(0..100);
        let y = rng.gen_range(0..100);
        stars.push_str(&format!(r#"<div class="star" style="left:{}%; top:{}%;"></div>"#, x, y));
    }

    stars
}

