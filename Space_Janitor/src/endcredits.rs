use bevy::prelude::*;

use crate::{GameState};

#[derive(Component)]
pub struct EndCreditsScreen;

//derive adds the trait listed inside the () to the struct or enum below?
//In this case Resource is the trait
//Resource allows you to store a single global instance of some data type
//Meaning you can't copy it? I don't get the point but aight
// #[derive(Resource)]
// pub struct EndCreditsImage(Handle<Image>);

pub struct EndCreditsPlugin;
impl Plugin for EndCreditsPlugin{
    fn build(&self, app: &mut App){
        app
            //.add_systems(Startup, load_endcred)
            .add_systems(OnEnter(GameState::EndCredits), setup_endcredits);
    }
}

// fn load_endcred(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
// ){
//     let endcred_tex_handle:Handle<Image>  = asset_server.load("EndCreditsImage.png");
//     commands.insert_resource(EndCreditsImage(endcred_tex_handle));
// }

fn setup_endcredits(
    mut commands: Commands,
    //endcred_img: Res<EndCreditsImage>,
    asset_server: Res<AssetServer>,
    //mut camera: Single<&mut Transform, With<Camera>>,
) {
    commands.spawn((
        Sprite::from_image(asset_server.load("EndCreditsImage.png")),
        //Transform::from_xyz(0., 0., 0.),
        EndCreditsScreen,
    ));

    //camera.translation.x = 0.;
}