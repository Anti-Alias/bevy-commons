pub use bevy::prelude::*;
use vidya_dialog::DialogPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DialogPlugin::new(10))
        .add_startup_system(startup)
        .run();
}

fn startup(mut commands: Commands) {

    commands.spawn(Camera2dBundle::default());

    // Dimensions of the box
    let box_width = Val::Px(512.0);
    let box_height = Val::Px(128.0);
    let box_bottom = Val::Px(128.0);

    // Places to make slice in pixels
    let left_slice = 4.0;
    let right_slice = 4.0;
    let bottom_slice = 4.0;
    let top_slice = 4.0;

    // Root node size of screen
    commands.spawn(NodeBundle {
        style: Style {
            justify_content: JustifyContent::SpaceAround,
            align_items: AlignItems::Center,
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..Default::default()
        },
        ..Default::default()
    
    }).with_children(|node| {
        // Text-box node container
        node.spawn(NodeBundle {
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
            background_color: Color::BLACK.into(),
            ..Default::default()
        }).with_children(|node| {

            // Bottom row (bottom)
            node.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Px(bottom_slice)),
                    min_size: Size::new(Val::Percent(100.0), Val::Px(bottom_slice)),
                    ..Default::default()
                },
                background_color: Color::RED.into(),
                ..Default::default()
            }).with_children(|node| {

                // Bottom-left
                node.spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(left_slice), Val::Percent(100.0)),
                        min_size: Size::new(Val::Px(left_slice), Val::Percent(100.0)),
                        ..Default::default()
                    },
                    background_color: Color::YELLOW.into(),
                    ..Default::default()
                });

                // Bottom-middle
                node.spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        ..Default::default()
                    },
                    background_color: Color::ORANGE.into(),
                    ..Default::default()
                });

                // Bottom-right
                node.spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(right_slice), Val::Percent(100.0)),
                        min_size: Size::new(Val::Px(right_slice), Val::Percent(100.0)),
                        ..Default::default()
                    },
                    background_color: Color::TEAL.into(),
                    ..Default::default()
                });
            });

            // Middle row (center)
            node.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..Default::default()
                },
                background_color: Color::GREEN.into(),
                ..Default::default()
            }).with_children(|node| {
                // Center-left
                node.spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(left_slice), Val::Percent(100.0)),
                        min_size: Size::new(Val::Px(left_slice), Val::Percent(100.0)),
                        ..Default::default()
                    },
                    background_color: Color::ORANGE.into(),
                    ..Default::default()
                });

                // Center-middle
                node.spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        ..Default::default()
                    },
                    background_color: Color::TEAL.into(),
                    ..Default::default()
                });

                // Center-right
                node.spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(right_slice), Val::Percent(100.0)),
                        min_size: Size::new(Val::Px(right_slice), Val::Percent(100.0)),
                        ..Default::default()
                    },
                    background_color: Color::YELLOW.into(),
                    ..Default::default()
                });
            });

            // Top row (bottom)
            node.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Px(top_slice)),
                    min_size: Size::new(Val::Percent(100.0), Val::Px(top_slice)),
                    ..Default::default()
                },
                background_color: Color::BLUE.into(),
                ..Default::default()
            }).with_children(|node| {
                // Center-left
                node.spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(left_slice), Val::Percent(100.0)),
                        min_size: Size::new(Val::Px(left_slice), Val::Percent(100.0)),
                        ..Default::default()
                    },
                    background_color: Color::YELLOW.into(),
                    ..Default::default()
                });

                // Center-middle
                node.spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        ..Default::default()
                    },
                    background_color: Color::ORANGE.into(),
                    ..Default::default()
                });

                // Center-right
                node.spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(right_slice), Val::Percent(100.0)),
                        min_size: Size::new(Val::Px(right_slice), Val::Percent(100.0)),
                        ..Default::default()
                    },
                    background_color: Color::TEAL.into(),
                    ..Default::default()
                });
            });
        });
    });
}