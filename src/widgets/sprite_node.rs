use bevy::sprite::Anchor;

pub use crate::prelude::*;

/// A sprite node in 2d space.
/// Note that you will need to attach a transform as well
#[derive(Debug, Clone)]
pub struct SpriteNode {
    pub texture_path: &'static str,
    pub sprite: Sprite,
}

impl PartialEq for SpriteNode {
    fn eq(&self, other: &Self) -> bool {
        self.texture_path == other.texture_path && sprite_compare(&self.sprite, &other.sprite)
    }
}

fn sprite_compare(l: &Sprite, r: &Sprite) -> bool {
    l.color == r.color
        && l.flip_x == r.flip_x
        && l.flip_y == r.flip_y
        && l.custom_size == r.custom_size
        && l.rect == r.rect
        && anchor_compare(&l.anchor, &r.anchor)
}

fn anchor_compare(l: &Anchor, r: &Anchor) -> bool {
    l.as_vec() == r.as_vec()
}

impl MavericNode for SpriteNode {
    type Context = ();

    fn set_components(mut commands: SetComponentCommands<Self, Self::Context>) {
        commands.insert_static_bundle(SpatialBundle::default());

        commands.scope(|commands| {
            commands
                .advanced(|args, commands| {
                    if args.is_hot() {
                        let node = args.node;
                        let server: &AssetServer = commands
                            .get_res_untracked()
                            .expect("Could not get asset server");
                        let image_handle: Handle<Image> = server.load(node.texture_path);

                        commands.insert(image_handle);
                    }
                })
                .finish()
        });
        commands.insert_with_node(|x| x.sprite.clone());
    }

    fn set_children<R: MavericRoot>(_commands: SetChildrenCommands<Self, Self::Context, R>) {}
}
