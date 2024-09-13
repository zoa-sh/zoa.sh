// PATH: src/mods/dynamic.rs
use rand::Rng;

// Randomly Generated Stars
pub fn generate_stars(count: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..count).map(|_| {
        let x = rng.gen_range(0..100);
        let y = rng.gen_range(0..100);
        format!(r#"<div class="star" style="left:{}%; top:{}%;"></div>"#, x, y)
    }).collect()
}
