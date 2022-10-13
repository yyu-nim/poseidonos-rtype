use rand::Rng;

pub fn GenerateUniqueId() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen::<u32>()
}