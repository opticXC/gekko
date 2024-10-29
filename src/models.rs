use rand::Rng;
use rocket::serde::Deserialize;

pub const RAND_MAX: i32 = 1000;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RandomRange {
    pub min: i32,
    pub max: i32,
}

pub fn random(count: usize, min: i32, max: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut vec = Vec::new();
    for _ in 0..count {
        vec.push(rng.gen_range(min..max));
    }
    vec
}

pub fn gen_uuid() -> uuid::Uuid {
    let r = rand::thread_rng().gen_range(1..u128::MAX);

    let uuid = uuid::Uuid::from_u128(r);
    return uuid;
}
