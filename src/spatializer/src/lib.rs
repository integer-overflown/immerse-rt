use irt_space as space;

struct Scene {
    sources: Vec<Source>,
}

struct Source {
    position: space::Point3,
}

struct Listener {
    orientation: space::Orientation,
}
