use bevy::{app::AppExit, prelude::*};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.53, 0.53, 0.53)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(spawn_player)
        .add_system(text_input)
        .add_system(move_player) 
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scale = 1. / 50.;
    commands.spawn_bundle(camera_bundle);
}

#[derive(Component)]
struct Player;

fn spawn_player(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0., 0.47, 1.),
                custom_size: Some(Vec2::new(1., 1.)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player); 
}

struct Moves(char);

fn text_input(
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut string: Local<String>,
    //mut ev_sender: EventWriter<Moves>,
) {
    for ev in char_evr.iter() {
        println!("Got char: '{}'", ev.char);
        string.push(ev.char);
        //ev_sender.send(Moves(ev.char));
    }
    if keys.just_pressed(KeyCode::Up) {
        println!("Got char: '{}'", 'w');
        string.push('w');
        //ev_sender.send(Moves('w'.clone()));
    }
    if keys.just_pressed(KeyCode::Down) {
        println!("Got char: '{}'", 's');
        string.push('s');
        //ev_sender.send(Moves('s'.clone()));
    }
    if keys.just_pressed(KeyCode::Right) {
        println!("Got char: '{}'", 'd');
        string.push('d');
    }
        //ev_sender.send(Moves('d'.clone()re
    if keys.just_pressed(KeyCode::Left) {
        println!("Got char: '{}'", 'a');
        string.push('a');
        //ev_sender.send(Moves('a'.clone()));
    }
    if keys.just_pressed(KeyCode::Return) {
        println!("Text input: {}", *string);
        string.clear();
    }
}



//only works with wasd, not up,down,left,right keys -- converting to char is diff across os
fn move_player(mut ev_rec: EventReader<ReceivedCharacter>, mut player_query: Query<&mut Transform, With<Player>>) {
    let mut direction = Vec2::ZERO;
    for ev in ev_rec.iter() {
        print!("received {}", ev.char);
        if ev.char == 'w' {
            direction.y += 1.;
        }
        if ev.char == 's' {
            direction.y -= 1.;
        }
        if ev.char == 'd' {
            direction.x += 1.;
        }
        if ev.char == 'a' {
            direction.x -= 1.;
        }
    }
    
    if direction == Vec2::ZERO {
        return;
    }

    let move_speed = 0.13;
    let move_delta = (direction * move_speed).extend(0.);

    for mut transform in player_query.iter_mut() {
        transform.translation += move_delta;
    }
}

// fn move_player(mut ev_rec: EventReader<ReceivedCharacter>, mut player_query: Query<&mut Transform, With<Player>>) {
//     let mut direction = Vec2::ZERO;
//     for ev in ev_rec.iter() {
//         print!("received {}", ev.char);
//         if ev.char == 'w' {
//             direction.y += 1.;
//         }
//         if ev.char == 's' {
//             direction.y -= 1.;
//         }
//         if ev.char == 'd' {
//             direction.x += 1.;
//         }
//         if ev.char == 'a' {
//             direction.x -= 1.;
//         }
//     }
    
//     if direction == Vec2::ZERO {
//         return;
//     }

//     let move_speed = 0.13;
//     let move_delta = (direction * move_speed).extend(0.);

//     for mut transform in player_query.iter_mut() {
//         transform.translation += move_delta;
//     }
// }

// fn move_player(keys: Res<Input<KeyCode>>, mut player_query: Query<&mut Transform, With<Player>>) {
//     let mut direction = Vec2::ZERO;
//     if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
//         direction.y += 1.;
//     }
//     if keys.any_pressed([KeyCode::Down, KeyCode::S]) {
//         direction.y -= 1.;
//     }
//     if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
//         direction.x += 1.;
//     }
//     if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
//         direction.x -= 1.;
//     }
//     if direction == Vec2::ZERO {
//         return;
//     }

//     let move_speed = 0.13;
//     let move_delta = (direction * move_speed).extend(0.);

//     for mut transform in player_query.iter_mut() {
//         transform.translation += move_delta;
//     }
// }
