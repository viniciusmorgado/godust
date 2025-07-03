use rand::rng;

pub fn generate_godot_uid() -> String {
    use rand::Rng;
    let mut rng = rng();
    let chars: String = (0..13)
        .map(|_| {
            let chars = "abcdefghijklmnopqrstuvwxyz0123456789";
            chars.chars().nth(rng.random_range(0..chars.len())).unwrap()
        })
        .collect();
    format!("uid://{}", chars)
}
