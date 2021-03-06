use crate::*;

pub fn increment_attacks_dealt(stat_set: &mut StatSet<Stats>) {
    stat_set.stats.get_mut(&Stats::AttacksDealt).unwrap().value += 1.0;
}

pub fn damage(stat_set: &mut StatSet<Stats>, damage: f64) -> bool {
    let mut health_inst = stat_set.stats.get_mut(&Stats::Health).unwrap();
    health_inst.value -= damage;
    health_inst.value <= 0.0
}

pub fn entities_in_radius<
    D: Deref<Target = MaskedStorage<Point>>,
    F1: Fn(Entity, Point) -> bool,
    F2: Fn(Entity, Point, f32) -> bool,
>(
    around: &Point,
    entities: &EntitiesRes,
    positions: &Storage<'_, Point, D>,
    pre_filter: F1,
    post_filter: F2,
) -> Vec<(Entity, Point, f32)> {
    let mut vec = (&*entities, positions)
        .join()
        .filter(|(e, p)| pre_filter(*e, **p))
        .map(|(e, p)| (e, p.clone(), dist(around, p)))
        .filter(|(e, p, d)| post_filter(*e, *p, *d))
        .collect::<Vec<_>>();
    // Sort by distance
    vec.sort_by(|e1, e2| e1.2.partial_cmp(&e2.2).unwrap());
    vec
}

#[cfg(not(features="wasm"))]
pub fn load_yaml<T: serde::de::DeserializeOwned>(filepath: &str) -> T {
    return serde_yaml::from_reader(std::fs::File::open(filepath).expect("Failed to load yaml file")).expect("Failed to parse yaml file into the requested type.");
}

#[cfg(features="wasm")]
pub fn load_yaml<T: serde::de::DeserializeOwned>(filepath: &str) -> T {
    let content_bytes = EMBED.lock().get_resource(filepath.to_string()).expect("Yaml file isn't embedded into the binary.");
    let content = String::from_utf8(content_bytes.to_vec()).unwrap();
    return serde_yaml::from_str(&content).expect("Failed to parse yaml file into the requested type.");
}

#[macro_export]
macro_rules! centity {
    ($world:ident, $($comps:expr),*$(,)?) => {
        $world.create_entity()
            $(.with($comps))*
            .build()
    }
}

#[allow(unused)]
#[macro_export]
macro_rules! add_embed {
    ($($path:literal),*) => {$(EMBED.lock().add_resource($path.to_string().replace("../", ""), include_bytes!($path));)*}
}

#[macro_export]
macro_rules! register {
    ($world:ident, $($types:ty),*$(,)?) => {$($world.register::<$types>();)*}
}

