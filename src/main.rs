use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, (update_people, great_people).chain()))
        .run();
}

fn hello_world() {
    println!("Hello World");
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zeyna Nieves".to_string())));
}

fn great_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}", name.0);
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elain Hume".to_string();
            break;
        }
    }
}
