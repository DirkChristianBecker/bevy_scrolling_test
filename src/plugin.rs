use bevy::prelude::*;

use bevy::a11y::{
    accesskit::{NodeBuilder, Role},
    AccessibilityNode,
};

use bevy::input::mouse::*;


#[derive(Component, Default)]
struct ScrollingList {
    x : f32,
    y : f32, 
}


fn setup(
    mut commands: Commands, 
    assets: Res<AssetServer>) {
    
    commands.spawn(
        // UI Root
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_self : AlignSelf::Center,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        // 
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(200.),
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        TextBundle::from_section(
                            "Scrolling list",
                            TextStyle {
                                font: assets.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 25.,
                                ..default()
                            },
                        ),
                        Label,
                    ));
                    // List with hidden overflow
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_self: AlignSelf::Stretch,
                                height: Val::Percent(50.0),
                                overflow: Overflow::clip(),
                                ..default()
                            },
                            background_color: Color::rgb(0.10, 0.10, 0.10).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // Moving panel
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            flex_direction: FlexDirection::Column,
                                            align_items: AlignItems::Center,
                                            width : Val::Percent(25.0),
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    ScrollingList::default(),
                                    AccessibilityNode(NodeBuilder::new(Role::List)),
                                ))
                                .with_children(|parent| {
                                    // List items
                                    for i in 0..30 {
                                        parent.spawn((
                                            TextBundle::from_section(
                                                format!("Item {i}"),
                                                TextStyle {
                                                    font: assets.load("fonts/FiraSans-Bold.ttf"),
                                                    font_size: 20.,
                                                    ..default()
                                                },
                                            ),
                                            Label,
                                            AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                        ));
                                    }
                                });
                        });
                });
        });
}

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        let mut shift_pressed = false;
        if keys.pressed(KeyCode::ShiftLeft) { shift_pressed = true; }
        if keys.pressed(KeyCode::ShiftRight) { shift_pressed = true; }

        for (mut scrolling_list, mut style, parent, list_node) in &mut query_list {
            if shift_pressed {
                let items_width = list_node.size().x;
                let container_width = query_node.get(parent.get()).unwrap().size().x;

                let max_scroll = (container_width - items_width).max(0.0);

                let dy = match mouse_wheel_event.unit {
                    MouseScrollUnit::Line => mouse_wheel_event.y * 20.0,
                    MouseScrollUnit::Pixel => mouse_wheel_event.y,
                };

                scrolling_list.y += dy;
                scrolling_list.y = scrolling_list.y.clamp(0.0, max_scroll);
                style.left = Val::Px(scrolling_list.y);
            } else {
                let items_height = list_node.size().y;
                let container_height = query_node.get(parent.get()).unwrap().size().y;
    
                let max_scroll = (items_height - container_height).max(0.);
    
                let dy = match mouse_wheel_event.unit {
                    MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                    MouseScrollUnit::Pixel => mouse_wheel_event.y,
                };
    
                scrolling_list.x += dy;
                scrolling_list.x = scrolling_list.x.clamp(-max_scroll, 0.0);
                style.top = Val::Px(scrolling_list.x);
            }
        }
    }
}

pub struct ScrollingExamplePlugin;
impl Plugin for ScrollingExamplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, mouse_scroll);
    }
}