use bevy::prelude::*;
use shooting_game_shared::ServerMessage;

use crate::{
    states::{AppState, OnlineGameState},
    ui_components::{Blink, InteractionUI, MainContainer},
    util::cleanup_components,
};

use super::connection::ReceiveMessageEvent;
pub struct ErrorPagePlugin;

impl Plugin for ErrorPagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(OnlineGameState::Error), show_error_page)
            .add_systems(
                Update,
                handle_return_button_interaction.run_if(in_state(OnlineGameState::Error)),
            )
            .add_systems(
                OnExit(OnlineGameState::Error),
                cleanup_components::<ErrorPage>,
            )
            .add_observer(listen_to_game_interrupted);
    }
}

#[derive(Component)]
struct ErrorPage;

#[derive(Component)]
struct ReturnButton;

fn show_error_page(mut commands: Commands) {
    commands
        .spawn((ErrorPage, MainContainer))
        .with_children(|error_page_background| {
            error_page_background.spawn(Text::new("Error Occured"));
            error_page_background
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
    mut return_button_query: Query<&Interaction, With<ReturnButton>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let Ok(interaction) = return_button_query.single_mut() else {
        warn!("Start button not found in handle_return_button_interaction");
        return;
    };
    if *interaction == Interaction::Pressed {
        next_state.set(AppState::MainMenu);
    };
}

fn listen_to_game_interrupted(
    trigger: Trigger<ReceiveMessageEvent>,
    current_state: Res<State<OnlineGameState>>,
    mut next_state: ResMut<NextState<OnlineGameState>>,
) {
    if matches!(
        current_state.get(),
        OnlineGameState::Error | OnlineGameState::Result
    ) {
        return;
    }
    if matches!(trigger.event().0, ServerMessage::GameInterrupted) {
        next_state.set(OnlineGameState::Error);
    }
}
