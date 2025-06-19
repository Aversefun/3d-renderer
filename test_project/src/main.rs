fn main() {
    let mut scene = threed::Scene::new();
    scene.render();
    scene.display_sdl3().unwrap();
}
