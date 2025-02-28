use bevy::prelude::*;

use crate::{
    components::{Score, SelfPlayer},
    states::{AppState, OnlineGameState},
    ui_components::{Blink, InteractionUI, MainContainer},
};

pub struct ResultPlugin;

impl Plugin for ResultPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(OnlineGameState::Result), show_result)
            .add_systems(
                Update,
                handle_return_button_interaction.run_if(in_state(OnlineGameState::Result)),
            );
    }
}

#[derive(Component)]
struct Result;

#[derive(Component)]
struct ReturnButton;

fn show_result(mut commands: Commands, score_q: Query<(&Score, Option<&SelfPlayer>)>) {
    let mut your_score = 0;
    let mut opponent_score = 0;
    for (score, self_player_op) in score_q.iter() {
        if self_player_op.is_none() {
            opponent_score = score.0 as i8;
        } else {
            your_score = score.0 as i8;
        }
    }
    let result_text = match your_score - opponent_score {
        0 => "Draw",
        x if x > 0 => "You Win",
        x if x < 0 => "You Lose",
        _ => "Error",
    };

    commands
        .spawn((Result, MainContainer))
        .with_children(|result_background| {
            result_background.spawn(Text::new(result_text));
            result_background.spawn(Text::new(format!("Your Score: {}", your_score)));
            result_background.spawn(Text::new(format!("Opponent Score: {}", opponent_score)));
            result_background
                .spawn(Node {
                    height: Val::Percent(100.),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexEnd,
                    row_gap: Val::Px(5.),
                    ..default()
                })
                .with_children(|return_container| {
                    return_container.spawn((
                        Blink::new_with_speed(0.02),
                        Text::new("Click Return to return to main menu"),
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
                    return_container
                        .spawn((
                            ReturnButton,
                            InteractionUI,
                            Node {
                                align_self: AlignSelf::FlexEnd,
                                width: Val::Px(120.),
                                height: Val::Px(50.),
                                border: UiRect::all(Val::Px(2.)),
                                display: Display::Flex,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BackgroundColor::from(Color::srgba(0.1, 0.1, 0.1, 1.)),
                            BorderColor::from(Color::BLACK),
                        ))
                        .with_child(Text::new("Return"));
                });
        });
}

fn handle_return_button_interaction(
    mut commands: Commands,
    mut return_button_query: Query<&Interaction, With<ReturnButton>>,
    result_query: Query<Entity, With<Result>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let Ok(interaction) = return_button_query.get_single_mut() else {
        warn!("Start button not found in handle_return_button_interaction");
        return;
    };
    if *interaction == Interaction::Pressed {
        let Ok(result) = result_query.get_single() else {
            panic!("Result not found in handle_return_button_interaction");
        };
        if let Some(entity_commands) = commands.get_entity(result) {
            entity_commands.despawn_recursive();
        }
        next_state.set(AppState::MainMenu);
    };
}
