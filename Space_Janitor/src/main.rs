use bevy::{prelude::*, window::PresentMode};

mod endcredits;

const TITLE: &str = "Space Janitor";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    EndCredits,
}

fn main(){
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from(TITLE),
                resolution: (WIN_W, WIN_H).into(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))



        //General Systems
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(GameState::EndCredits), log_state_change)

        //Subsystems
        .add_plugins((
            endcredits::EndCreditsPlugin,
        ))

        //Initial State
        .init_state::<GameState>()
        
        .run();
}

fn setup_camera(mut commands: Commands){
    commands.spawn(Camera2d);
}

fn log_state_change(state: Res<State<GameState>>) {
    info!("Just moved to {:?}!", state.get());
}