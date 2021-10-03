use crate::templates::docs::container::{DocsContainer, DocsContainerProps};
use crate::templates::docs::generation::{
    get_build_paths, get_build_state, DocsManifest, DocsVersionStatus,
};
use perseus::{t, GenericNode, Template};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use sycamore::prelude::Template as SycamoreTemplate;
use sycamore::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct DocsPageProps {
    // We don't need to use translation IDs here because the docs are i18ned at the filesystem level
    pub title: String,
    pub content: String,
    pub sidebar_content: String,
    pub status: DocsVersionStatus,
    pub manifest: DocsManifest,
    pub current_version: String,
}

#[component(DocsPage<G>)]
pub fn docs_page(props: DocsPageProps) -> SycamoreTemplate<G> {
    // These come pre-translated for the current locale
    // Note that all the docs files have a title emblazoned at the top already, so we only need the title in the `<head>`
    let DocsPageProps {
        content,
        sidebar_content,
        status,
        manifest,
        current_version,
        ..
    } = props;
    template! {
        DocsContainer(DocsContainerProps {
            docs_links: sidebar_content,
            children: template! {
                div(class = "markdown", dangerously_set_inner_html = &content)
            },
            status,
            manifest,
            current_version
        })
        // Because of how Perseus currently shifts everything, we need to re-highlight
        script {
            "window.Prism.highlightAll();"
        }
    }
}

pub fn get_template<G: GenericNode>() -> Template<G> {
    Template::new("docs")
        .build_paths_fn(Rc::new(get_build_paths))
        .build_state_fn(Rc::new(get_build_state))
        .template(Rc::new(|props| {
            template! {
                DocsPage(serde_json::from_str(&props.unwrap()).unwrap())
            }
        }))
        .head(Rc::new(|props| {
            let props: DocsPageProps = serde_json::from_str(&props.unwrap()).unwrap();
            template! {
                title { (format!("{} | {}", props.title, t!("docs-title-base"))) }
                link(rel = "stylesheet", href = "/.perseus/static/styles/markdown.css")
                link(rel = "stylesheet", href = "/.perseus/static/styles/docs_links_markdown.css")
                link(rel = "stylesheet", href = "/.perseus/static/prism.css")
                script(src = "/.perseus/static/prism.js")
            }
        }))
}
