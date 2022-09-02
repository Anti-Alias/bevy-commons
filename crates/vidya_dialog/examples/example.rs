pub use bevy::prelude::*;
use vidya_dialog::DialogPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DialogPlugin::new(10))
        .add_startup_system(startup)
        .run();
}

fn startup(
    mut commands: Commands,
    assets: Res<AssetServer>
) {

    let box_width = Val::Px(512.0);
    let box_height = Val::Px(128.0);
    let box_bottom = Val::Px(128.0);
    let left_slice = 4.0;
    let right_slice = 4.0;
    let bottom_slice = 4.0;
    let top_slice = 4.0;

    commands.spawn_bundle(Camera2dBundle::default());

    // Root node size of screen
    commands.spawn_bundle(NodeBundle {
        style: Style {
            justify_content: JustifyContent::SpaceAround,
            align_items: AlignItems::Center,
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..Default::default()
        },
        ..Default::default()
    
    }).with_children(|node| {
        // Text-box node container
        node.spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                size: Size::new(box_width, box_height),
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: box_bottom,
                    ..Default::default()
                },
                ..Default::default()
            },
            color: Color::BLACK.into(),
            ..Default::default()
        }).with_children(|node| {

            // Bottom row (bottom)
            node.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Px(bottom_slice)),
                    min_size: Size::new(Val::Percent(100.0), Val::Px(bottom_slice)),
                    ..Default::default()
                },
                color: Color::RED.into(),
                ..Default::default()
            }).with_children(|node| {

                // Bottom-left
                node.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(left_slice), Val::Percent(100.0)),
                        min_size: Size::new(Val::Px(left_slice), Val::Percent(100.0)),
                        ..Default::default()
                    },
                    color: Color::YELLOW.into(),
                    ..Default::default()
                });

                // Bottom-middle
                node.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        ..Default::default()
                    },
                    color: Color::ORANGE.into(),
                    ..Default::default()
                });

                // Bottom-right
                node.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(right_slice), Val::Percent(100.0)),
                        min_size: Size::new(Val::Px(right_slice), Val::Percent(100.0)),
                        ..Default::default()
                    },
                    color: Color::TEAL.into(),
                    ..Default::default()
                });
            });

            // Middle row (center)
            node.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..Default::default()
                },
                color: Color::GREEN.into(),
                ..Default::default()
            }).with_children(|node| {
                // Center-left
                node.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(left_slice), Val::Percent(100.0)),
                        min_size: Size::new(Val::Px(left_slice), Val::Percent(100.0)),
                        ..Default::default()
                    },
                    color: Color::ORANGE.into(),
                    ..Default::default()
                });

                // Center-middle
                node.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        ..Default::default()
                    },
                    color: Color::TEAL.into(),
                    ..Default::default()
                });

                // Center-right
                node.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(right_slice), Val::Percent(100.0)),
                        min_size: Size::new(Val::Px(right_slice), Val::Percent(100.0)),
                        ..Default::default()
                    },
                    color: Color::YELLOW.into(),
                    ..Default::default()
                });
            });

            // Top row (bottom)
            node.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Px(top_slice)),
                    min_size: Size::new(Val::Percent(100.0), Val::Px(top_slice)),
                    ..Default::default()
                },
                color: Color::BLUE.into(),
                ..Default::default()
            }).with_children(|node| {
                // Center-left
                node.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(left_slice), Val::Percent(100.0)),
                        min_size: Size::new(Val::Px(left_slice), Val::Percent(100.0)),
                        ..Default::default()
                    },
                    color: Color::YELLOW.into(),
                    ..Default::default()
                });

                // Center-middle
                node.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        ..Default::default()
                    },
                    color: Color::ORANGE.into(),
                    ..Default::default()
                });

                // Center-right
                node.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(right_slice), Val::Percent(100.0)),
                        min_size: Size::new(Val::Px(right_slice), Val::Percent(100.0)),
                        ..Default::default()
                    },
                    color: Color::TEAL.into(),
                    ..Default::default()
                });
            });
        });
    });
}