use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::components::ScoreText;
use crate::resources::Score;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..Default::default()
    });
}

pub fn spawn_score(mut commands: Commands, asset_server: Res<AssetServer>, score: Res<Score>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                min_size: Size {
                    width: Val::Percent(100.0),
                    ..Default::default()
                },
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    get_score_string(&score),
                    TextStyle {
                        font: asset_server.load("FiraSans-Bold.ttf"),
                        font_size: 50.0,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment::Center),
                ScoreText,
            ));
        });
}

pub fn update_score(mut text_query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    if score.is_changed() {
        let mut text = text_query.get_single_mut().unwrap();

        text.sections[0].value = get_score_string(&score);
    }
}

fn get_score_string(score: &Score) -> String {
    format!("{} - {}", score.player_one, score.player_two)
}
