use bevy::prelude::SystemLabel;

#[derive(Hash, PartialEq, Eq, Debug, Clone, SystemLabel)]
pub enum AppLabels {
    Websocket,
    Ui,
    AfterUi,
    Selection,
    Indication,
    CubeDisplay,
    CubeRotation
}
