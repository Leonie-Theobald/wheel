use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Text::new(
            "    S - Toggle visibility of sums
   UP - Select disc up
 DOWN - Select disc down
 LEFT - Rotate disc clock wise
RIGHT - Rotate disc counter clock wise
",
        ),
        TextFont {
            font_size: FontSize::Px(14.),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
    ));
}
