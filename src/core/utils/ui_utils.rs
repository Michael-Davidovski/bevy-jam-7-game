use bevy::prelude::*;


pub struct UiUtilsPlugin;

impl Plugin for UiUtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            button_interaction_visual,
            button_sound_feedback,
        ));
    }
}

/// System to handle button visual feedback
fn button_interaction_visual(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(colors::BUTTON_PRESSED);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(colors::BUTTON_HOVERED);
            }
            Interaction::None => {
                *color = BackgroundColor(colors::BUTTON_NORMAL);
            }
        }
    }
}

/// System to handle button sound feedback (placeholder - add your own sounds)
fn button_sound_feedback(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    // audio: Res<Audio>, // Uncomment when you add audio
) {
    for interaction in interaction_query.iter() {
        match *interaction {
            Interaction::Pressed => {
                // Play click sound
                // audio.play(asset_server.load("sounds/click.ogg"));
                println!("Button clicked!");
            }
            Interaction::Hovered => {
                // Play hover sound
                // audio.play(asset_server.load("sounds/hover.ogg"));
            }
            Interaction::None => {}
        }
    }
}

/// Builder for creating UI containers easily
pub struct UiBuilder<'w, 's, 'a> {
    commands: &'a mut Commands<'w, 's>,
    parent: Option<Entity>,
}

impl<'w, 's, 'a> UiBuilder<'w, 's, 'a> {
    pub fn new(commands: &'a mut Commands<'w, 's>) -> Self {
        Self {
            commands,
            parent: None,
        }
    }

    /// Create a container (column by default)
    pub fn container(&mut self) -> UiContainer<'w, 's, '_> {
        UiContainer::new(self.commands, self.parent)
    }

    /// Create a row container
    pub fn row(&mut self) -> UiContainer<'w, 's, '_> {
        UiContainer::new(self.commands, self.parent).direction(FlexDirection::Row)
    }

    /// Create a column container
    pub fn column(&mut self) -> UiContainer<'w, 's, '_> {
        UiContainer::new(self.commands, self.parent).direction(FlexDirection::Column)
    }

    /// Create a full-screen overlay
    pub fn overlay(&mut self, color: Color) -> UiContainer<'w, 's, '_> {
        UiContainer::new(self.commands, self.parent)
            .width(Val::Percent(100.0))
            .height(Val::Percent(100.0))
            .background(color)
    }

    /// Create a button
    pub fn button(&mut self, text: &str) -> Entity {
        let button_entity = self.commands
            .spawn((
                Button,
                Node {
                    padding: UiRect::all(Val::Px(15.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new(text),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
            })
            .id();

        if let Some(parent) = self.parent {
            self.commands.entity(parent).add_child(button_entity);
        }

        button_entity
    }

    /// Create a text label
    pub fn label(&mut self, text: &str) -> Entity {
        self.text(text, 20.0, Color::WHITE)
    }

    /// Create text with custom size and color
    pub fn text(&mut self, text: &str, size: f32, color: Color) -> Entity {
        let text_entity = self.commands
            .spawn((
                Text::new(text),
                TextFont {
                    font_size: size,
                    ..default()
                },
                TextColor(color),
            ))
            .id();

        if let Some(parent) = self.parent {
            self.commands.entity(parent).add_child(text_entity);
        }

        text_entity
    }

    /// Create a spacer
    pub fn spacer(&mut self, size: f32) -> Entity {
        let spacer = self.commands
            .spawn(Node {
                width: Val::Px(size),
                height: Val::Px(size),
                ..default()
            })
            .id();

        if let Some(parent) = self.parent {
            self.commands.entity(parent).add_child(spacer);
        }

        spacer
    }
}

/// Container builder for nested UI elements
pub struct UiContainer<'w, 's, 'a> {
    commands: &'a mut Commands<'w, 's>,
    entity: Entity,
    node: Node,
    background: Option<BackgroundColor>,
    parent: Option<Entity>,
}

impl<'w, 's, 'a> UiContainer<'w, 's, 'a> {
    fn new(commands: &'a mut Commands<'w, 's>, parent: Option<Entity>) -> Self {
        let entity = commands.spawn_empty().id();
        
        Self {
            commands,
            entity,
            node: Node {
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background: None,
            parent,
        }
    }

    // Dimension methods
    pub fn width(mut self, val: Val) -> Self {
        self.node.width = val;
        self
    }

    pub fn height(mut self, val: Val) -> Self {
        self.node.height = val;
        self
    }

    pub fn size(mut self, width: Val, height: Val) -> Self {
        self.node.width = width;
        self.node.height = height;
        self
    }

    // Layout methods
    pub fn direction(mut self, direction: FlexDirection) -> Self {
        self.node.flex_direction = direction;
        self
    }

    pub fn align_items(mut self, align: AlignItems) -> Self {
        self.node.align_items = align;
        self
    }

    pub fn justify_content(mut self, justify: JustifyContent) -> Self {
        self.node.justify_content = justify;
        self
    }

    pub fn center(mut self) -> Self {
        self.node.align_items = AlignItems::Center;
        self.node.justify_content = JustifyContent::Center;
        self
    }

    // Spacing methods
    pub fn padding(mut self, val: Val) -> Self {
        self.node.padding = UiRect::all(val);
        self
    }

    pub fn padding_custom(mut self, left: Val, right: Val, top: Val, bottom: Val) -> Self {
        self.node.padding = UiRect { left, right, top, bottom };
        self
    }

    pub fn gap(mut self, val: Val) -> Self {
        self.node.row_gap = val;
        self.node.column_gap = val;
        self
    }

    // Style methods
    pub fn background(mut self, color: Color) -> Self {
        self.background = Some(BackgroundColor(color));
        self
    }

    pub fn display(mut self, display: Display) -> Self {
        self.node.display = display;
        self
    }

    // Add a component to the container
    pub fn component<T: Component>(self, component: T) -> Self {
        self.commands.entity(self.entity).insert(component);
        self
    }

    // Build the container and return its entity
    pub fn build(self) -> Entity {
        let mut entity_commands = self.commands.entity(self.entity);
        entity_commands.insert(self.node);
        
        if let Some(bg) = self.background {
            entity_commands.insert(bg);
        }

        if let Some(parent) = self.parent {
            self.commands.entity(parent).add_child(self.entity);
        }

        self.entity
    }

    // Build and get a builder for adding children
    pub fn children(mut self) -> UiBuilder<'w, 's, 'a> {
        let mut entity_commands = self.commands.entity(self.entity);
        entity_commands.insert(self.node);
        
        if let Some(bg) = self.background {
            entity_commands.insert(bg);
        }

        if let Some(parent) = self.parent {
            self.commands.entity(parent).add_child(self.entity);
        }

        UiBuilder {
            commands: self.commands,
            parent: Some(self.entity),
        }
    }

    // Build with a closure for adding children
    pub fn with_children<F>(mut self, f: F) -> Entity 
    where
        F: FnOnce(&mut UiBuilder),
    {
        let mut entity_commands = self.commands.entity(self.entity);
        entity_commands.insert(self.node);
        
        if let Some(bg) = self.background {
            entity_commands.insert(bg);
        }

        if let Some(parent) = self.parent {
            self.commands.entity(parent).add_child(self.entity);
        }

        let entity = self.entity;
        let mut builder = UiBuilder {
            commands: self.commands,
            parent: Some(entity),
        };
        f(&mut builder);
        entity
    }
}

// Convenience functions for quick Val creation
pub fn px(val: f32) -> Val {
    Val::Px(val)
}

pub fn percent(val: f32) -> Val {
    Val::Percent(val)
}

pub fn vw(val: f32) -> Val {
    Val::Vw(val)
}

pub fn vh(val: f32) -> Val {
    Val::Vh(val)
}

// Color presets
pub mod colors {
    use bevy::prelude::Color;

    pub const DARK_BG: Color = Color::srgb(0.1, 0.1, 0.1);
    pub const DARK_PANEL: Color = Color::srgb(0.15, 0.15, 0.15);
    pub const BUTTON_NORMAL: Color = Color::srgb(0.2, 0.2, 0.2);
    pub const BUTTON_HOVERED: Color = Color::srgb(0.3, 0.3, 0.3);
    pub const BUTTON_PRESSED: Color = Color::srgb(0.15, 0.15, 0.15);
    pub const TEXT_NORMAL: Color = Color::srgb(0.9, 0.9, 0.9);
    pub const TEXT_DIM: Color = Color::srgb(0.6, 0.6, 0.6);
    pub const OVERLAY: Color = Color::srgba(0.0, 0.0, 0.0, 0.7);
}

// Example usage in a system:
/*
fn setup_pause_menu(mut commands: Commands) {
    UiBuilder::new(&mut commands)
        .overlay(colors::OVERLAY)
        .center()
        .with_children(|ui| {
            ui.column()
                .size(px(400.0), px(500.0))
                .background(colors::DARK_PANEL)
                .padding(px(30.0))
                .gap(px(15.0))
                .center()
                .component(PauseMenuEntity)
                .with_children(|ui| {
                    ui.label("PAUSED");
                    ui.spacer(20.0);
                    ui.button("Resume").component(ResumeButton);
                    ui.button("Settings").component(SettingsButton);
                    ui.button("Main Menu").component(MainMenuButton);
                    ui.button("Quit").component(QuitButton);
                });
        });
}
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_utils_plugin_registers() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
           .add_plugins(UiUtilsPlugin);
        
        app.update();
        // Plugin läuft ohne Fehler
    }

    #[test]
    fn test_button_interaction_changes_color_on_hover() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
           .add_plugins(UiUtilsPlugin);
        
        // Button spawnen
        let button = app.world_mut().spawn((
            Button,
            Interaction::None,
            BackgroundColor(colors::BUTTON_NORMAL),
        )).id();
        
        app.update();
        
        // Interaction auf Hovered ändern
        app.world_mut().entity_mut(button)
            .insert(Interaction::Hovered);
        
        app.update();
        
        // Farbe sollte geändert sein
        let color = app.world_mut().get::<BackgroundColor>(button).unwrap();
        assert_eq!(color.0, colors::BUTTON_HOVERED);
    }

    #[test]
    fn test_button_interaction_changes_color_on_press() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
           .add_plugins(UiUtilsPlugin);
        
        let button = app.world_mut().spawn((
            Button,
            Interaction::None,
            BackgroundColor(colors::BUTTON_NORMAL),
        )).id();
        
        app.update();
        
        // Interaction auf Pressed ändern
        app.world_mut().entity_mut(button)
            .insert(Interaction::Pressed);
        
        app.update();
        
        let color = app.world_mut().get::<BackgroundColor>(button).unwrap();
        assert_eq!(color.0, colors::BUTTON_PRESSED);
    }

    #[test]
    fn test_button_interaction_returns_to_normal() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
           .add_plugins(UiUtilsPlugin);
        
        let button = app.world_mut().spawn((
            Button,
            Interaction::Hovered,
            BackgroundColor(colors::BUTTON_HOVERED),
        )).id();
        
        app.update();
        
        // Zurück zu None
        app.world_mut().entity_mut(button)
            .insert(Interaction::None);
        
        app.update();
        
        let color = app.world_mut().get::<BackgroundColor>(button).unwrap();
        assert_eq!(color.0, colors::BUTTON_NORMAL);
    }

    #[test]
    fn test_ui_builder_creates_button() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        
        // System das Button erstellt
        fn create_button_system(mut commands: Commands) {
            let mut builder = UiBuilder::new(&mut commands);
            builder.button("Test Button");
        }
        
        app.add_systems(Update, create_button_system);
        app.update();
        
        // Prüfe ob Button existiert
        let mut button_query = app.world_mut()
            .query_filtered::<Entity, With<Button>>();
        let button_count = button_query.iter(app.world()).count();
        assert_eq!(button_count, 1);
    }

    #[test]
    fn test_ui_builder_creates_text() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        
        fn create_text_system(mut commands: Commands) {
            let mut builder = UiBuilder::new(&mut commands);
            builder.text("Custom Text", 32.0, Color::srgb(1.0, 0.0, 0.0));
        }
        
        app.add_systems(Update, create_text_system);
        app.update();
        
        // Prüfe ob Text mit richtiger font size existiert
        let mut text_query = app.world_mut().query::<&TextFont>();
        let fonts: Vec<_> = text_query.iter(app.world()).collect();
        
        assert_eq!(fonts.len(), 1);
        assert_eq!(fonts[0].font_size, 32.0);
    }

    #[test]
    fn test_ui_container_builds_with_size() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        
        fn create_container_system(mut commands: Commands) {
            let mut builder = UiBuilder::new(&mut commands);
            builder.container()
                .width(Val::Px(100.0))
                .height(Val::Px(200.0))
                .build();
        }
        
        app.add_systems(Update, create_container_system);
        app.update();
        
        // Prüfe ob Container mit richtiger Größe existiert
        let mut node_query = app.world_mut().query::<&Node>();
        let nodes: Vec<_> = node_query.iter(app.world()).collect();
        
        assert!(nodes.iter().any(|n| 
            n.width == Val::Px(100.0) && n.height == Val::Px(200.0)
        ));
    }

    #[test]
    fn test_ui_container_row_direction() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        
        fn create_row_system(mut commands: Commands) {
            let mut builder = UiBuilder::new(&mut commands);
            builder.row().build();
        }
        
        app.add_systems(Update, create_row_system);
        app.update();
        
        let mut node_query = app.world_mut().query::<&Node>();
        let nodes: Vec<_> = node_query.iter(app.world()).collect();
        
        assert!(nodes.iter().any(|n| n.flex_direction == FlexDirection::Row));
    }

    #[test]
    fn test_ui_container_with_background() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        
        let test_color = Color::srgb(1.0, 0.0, 0.0);
        
        fn create_colored_container(mut commands: Commands) {
            let test_color = Color::srgb(1.0, 0.0, 0.0);
            let mut builder = UiBuilder::new(&mut commands);
            builder.container()
                .background(test_color)
                .build();
        }
        
        app.add_systems(Update, create_colored_container);
        app.update();
        
        let mut bg_query = app.world_mut().query::<&BackgroundColor>();
        let backgrounds: Vec<_> = bg_query.iter(app.world()).collect();
        
        assert!(backgrounds.iter().any(|bg| bg.0 == test_color));
    }

    #[test]
    fn test_ui_container_padding() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        
        fn create_padded_container(mut commands: Commands) {
            let mut builder = UiBuilder::new(&mut commands);
            builder.container()
                .padding(Val::Px(20.0))
                .build();
        }
        
        app.add_systems(Update, create_padded_container);
        app.update();
        
        let mut node_query = app.world_mut().query::<&Node>();
        let nodes: Vec<_> = node_query.iter(app.world()).collect();
        
        assert!(nodes.iter().any(|n| n.padding == UiRect::all(Val::Px(20.0))));
    }

    #[test]
    fn test_ui_container_center() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        
        fn create_centered_container(mut commands: Commands) {
            let mut builder = UiBuilder::new(&mut commands);
            builder.container()
                .center()
                .build();
        }
        
        app.add_systems(Update, create_centered_container);
        app.update();
        
        let mut node_query = app.world_mut().query::<&Node>();
        let nodes: Vec<_> = node_query.iter(app.world()).collect();
        
        assert!(nodes.iter().any(|n| 
            n.align_items == AlignItems::Center && 
            n.justify_content == JustifyContent::Center
        ));
    }

    #[test]
    fn test_convenience_val_functions() {
        assert_eq!(px(100.0), Val::Px(100.0));
        assert_eq!(percent(50.0), Val::Percent(50.0));
        assert_eq!(vw(80.0), Val::Vw(80.0));
        assert_eq!(vh(90.0), Val::Vh(90.0));
    }

    #[test]
    fn test_color_constants_are_valid() {
        // Teste dass alle Farbkonstanten gültig sind
        let _ = colors::DARK_BG;
        let _ = colors::DARK_PANEL;
        let _ = colors::BUTTON_NORMAL;
        let _ = colors::BUTTON_HOVERED;
        let _ = colors::BUTTON_PRESSED;
        let _ = colors::TEXT_NORMAL;
        let _ = colors::TEXT_DIM;
        let _ = colors::OVERLAY;
    }

    #[test]
    fn test_ui_builder_spacer() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        
        fn create_spacer_system(mut commands: Commands) {
            let mut builder = UiBuilder::new(&mut commands);
            builder.spacer(50.0);
        }
        
        app.add_systems(Update, create_spacer_system);
        app.update();
        
        let mut node_query = app.world_mut().query::<&Node>();
        let nodes: Vec<_> = node_query.iter(app.world()).collect();
        
        assert!(nodes.iter().any(|n| 
            n.width == Val::Px(50.0) && n.height == Val::Px(50.0)
        ));
    }
}
