use super::InteractionUI;
use bevy::prelude::*;

#[derive(Component)]
#[require(TextColor, InteractionUI)]
pub struct SelectableText {
    selected: bool,
    text: String,
}

impl SelectableText {
    pub fn new(text: &str, selected: bool) -> Self {
        Self {
            selected,
            text: text.to_string(),
        }
    }
    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }
}

pub struct SelectableTextPlugin;

impl Plugin for SelectableTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_selectable_text);
    }
}

fn handle_selectable_text(
    mut commands: Commands,
    selectable_text_query: Query<(Entity, &SelectableText), Changed<SelectableText>>,
) {
    for (entity, selectable_text) in selectable_text_query.iter() {
        let text = format!(
            "{}{}",
            if selectable_text.selected { "-> " } else { "" },
            selectable_text.text
        );
        commands.entity(entity).insert(Text::new(text));
    }
}
