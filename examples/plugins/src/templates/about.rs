use perseus::Template;
use sycamore::prelude::{component, template, GenericNode, Template as SycamoreTemplate};

// This page will actually be replaced entirely by a plugin!
#[perseus::template(AboutPage)]
#[component(AboutPage<G>)]
pub fn about_page() -> SycamoreTemplate<G> {
    template! {
        p { "About." }
    }
}

pub fn get_template<G: GenericNode>() -> Template<G> {
    Template::new("about").template(about_page).head(|_| {
        template! {
            title { "About Page | Perseus Example – Plugins" }
        }
    })
}
