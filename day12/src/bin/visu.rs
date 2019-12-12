use day12::*;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use nalgebra::Translation3;
use std::fs::File;
use std::io::{prelude::*, BufReader};

struct Draw {
    pub sphere: SceneNode,
    pub age: usize,
}

impl Draw {
    pub fn from(sphere: SceneNode) -> Self {
        Draw { sphere, age: 0 }
    }
}

fn main() {
    let filename = std::env::args()
        .skip(1)
        .next()
        .expect("give me the path to your program"); // Skiping the name of the binary

    let file = File::open(filename).expect("Can’t open file");
    let reader = BufReader::new(file);
    let mut window = Window::new("Advent of Code 2019 - Day 12");
    window.set_light(Light::StickToCamera);

    let mut system: System = reader
        .lines()
        .map(|l| l.unwrap().trim().parse().unwrap())
        .collect();
    let mut draw: Vec<Draw> = Vec::new();
    let mut time = std::time::Instant::now();

    while window.render() {
        if time.elapsed().as_millis() < 500 {
            continue;
        } else {
            time = std::time::Instant::now();
        }
        for d in draw.iter_mut() {
            if d.age == 10 {
                window.remove_node(&mut d.sphere);
            }
            let scale = (10. - d.age as f32) / 10.;
            d.sphere.set_local_scale(scale, scale, scale);
            d.age += 1;
        }
        system.moons.iter().enumerate().for_each(|(idx, m)| {
            let mut c = window.add_sphere(0.5);
            c.append_translation(&Translation3::new(
                m.position.x as f32,
                m.position.y as f32,
                m.position.z as f32,
            ));
            match idx {
                0 => c.set_color(1., 1., 1.),
                1 => c.set_color(1., 0., 0.),
                2 => c.set_color(0., 1., 0.),
                3 => c.set_color(0., 0., 1.),
                _ => panic!("too much element in the array"),
            }
            draw.push(Draw::from(c));
        });
        system.update();
    }
}
