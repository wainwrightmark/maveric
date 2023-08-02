use std::sync::Arc;

pub use crate::prelude::*;
pub use bevy::prelude::*;

#[derive(PartialEq, Debug)]
pub struct TextNode;

#[derive(PartialEq, Debug, Clone)]
pub struct TextNodeStyle {
    pub font_size: f32,
    pub color: Color,
    pub font: &'static str,
}

impl NodeBase for TextNode {
    type Context = AssetServer;
    type Args = (String, Arc<TextNodeStyle>);
}

impl ComponentsAspect for TextNode {
    fn set_components<'r>(
        args: &Self::Args,
        context: &<Self::Context as NodeContext>::Ref<'r>,
        commands: &mut impl ComponentCommands,
        _event: SetComponentsEvent,
    ) {
        let font = context.load(args.1.font);

        //TODO only update text and node components
        commands.insert(TextBundle::from_section(
            args.0.clone(),
            TextStyle {
                font,
                font_size: args.1.font_size,
                color: args.1.color,
            },
        ));
    }
}

impl HierarchyNode for TextNode {
    type ComponentsAspect = Self;

    type AncestorAspect = ();

    fn components_context<'a, 'r>(
        context: &'a <<Self as NodeBase>::Context as NodeContext>::Wrapper<'r>,
    ) -> &'a <<Self::ComponentsAspect as NodeBase>::Context as NodeContext>::Wrapper<'r> {
        context
    }

    fn ancestor_context<'a, 'r>(
        _context: &'a <<Self as NodeBase>::Context as NodeContext>::Wrapper<'r>,
    ) -> &'a <<Self::AncestorAspect as NodeBase>::Context as NodeContext>::Wrapper<'r> {
        &()
    }

    fn component_args<'a>(
        args: &'a <Self as NodeBase>::Args,
    ) -> &'a <Self::ComponentsAspect as NodeBase>::Args {
        args
    }

    fn ancestor_args<'a>(
        _args: &'a <Self as NodeBase>::Args,
    ) -> &'a <Self::AncestorAspect as NodeBase>::Args {
        &()
    }
}
