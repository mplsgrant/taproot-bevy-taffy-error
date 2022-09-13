use bevy::prelude::*;

use bevy::app::AppExit;
use bevy::window::close_on_esc;

use std::time::Duration;

use rand::prelude::*;

fn setup_global(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn spawn_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "Button",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_button).add_system(button_system);
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        children
            .iter()
            .for_each(|child| println!("child: {:?}", child));
        let mut text = text_query.get_mut(children[0]).unwrap();
        match interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Clicked".to_string();
                *color = Color::rgb(0.35, 0.75, 0.35).into();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hovered".to_string();
                *color = Color::rgb(0.25, 0.25, 0.25).into();
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = Color::rgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}

#[derive(Component)]
struct Person;

#[derive(Component, Debug)]
struct Name(String);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("Hello {}, {:?}", name.0, timer.0.elapsed());
        }
    }
}

fn insert_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Elaina Proctor".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Renzo Hume".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Zayna Nieves".to_string()));
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(insert_people)
            .add_system(greet_people);
    }
}

fn hello_world(timer: Res<GreetTimer>) {
    if timer.0.just_finished() {
        println!("HELLO WORLD 🚀! {:?}", timer.0.elapsed());
    }
}

fn change_clear_color(input: Res<Input<KeyCode>>, mut clear_color: ResMut<ClearColor>) {
    if input.just_pressed(KeyCode::Right) {
        clear_color.0 = Color::rgb(0.5, 0.5, 0.1)
    }
}

struct GreetTimer(Timer);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
        .add_startup_system(setup_global)
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .add_plugin(ButtonPlugin)
        .add_system(hello_world)
        .add_system(change_clear_color)
        .run();
}
