use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
fn main() {
    App::new()
        .add_startup_system(hello_world)
        .add_system(person_queryu)
        .run()
}

fn hello_world(mut commands: Commands) {
    commands.spawn(Person {
        name: "Momofor".to_string(),
    });
    commands.spawn(Person {
        name: "Haha".to_string(),
    });
}

fn person_queryu(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Hello: {}", person.name);
    }
}

#[derive(Component)]
struct Person {
    name: String,
}
