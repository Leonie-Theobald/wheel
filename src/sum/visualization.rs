use bevy::prelude::*;

use crate::sum::sum_number::SumNumber;

#[derive(Resource)]
pub(crate) struct SumMaterial(pub Handle<StandardMaterial>);

pub(crate) fn toggle_visibility(
    keys: Res<ButtonInput<KeyCode>>,
    sums: Query<&mut Visibility, With<SumNumber>>,
) {
    if !keys.just_pressed(KeyCode::KeyS) {
        return;
    }

    for mut visibility in sums {
        *visibility = match *visibility {
            Visibility::Visible => Visibility::Hidden,
            Visibility::Hidden => Visibility::Visible,
            Visibility::Inherited => Visibility::Hidden,
        }
    }
}
