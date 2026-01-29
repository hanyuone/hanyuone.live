use leptos::prelude::*;
use markdown::translate::container::callout::CalloutKind;

use crate::components::icon::Icon;

pub struct CalloutData {
    pub colour: String,
    pub icon: &'static icondata_core::IconData,
    pub title: String,
}

impl From<CalloutKind> for CalloutData {
    fn from(value: CalloutKind) -> Self {
        match value {
            CalloutKind::Note => Self {
                colour: "bg-blue/50".to_string(),
                icon: icondata::BsInfoCircleFill,
                title: "Note".to_string(),
            },
            CalloutKind::Tip => Self {
                colour: "bg-green/50".to_string(),
                icon: icondata::BsLightbulbFill,
                title: "Tip".to_string(),
            },
            CalloutKind::Important => Self {
                colour: "bg-purple/50".to_string(),
                icon: icondata::BsCheckCircleFill,
                title: "Important".to_string(),
            },
            CalloutKind::Warning => Self {
                colour: "bg-yellow/50".to_string(),
                icon: icondata::BsExclamationTriangleFill,
                title: "Warning".to_string(),
            },
            CalloutKind::Caution => Self {
                colour: "bg-red/50".to_string(),
                icon: icondata::BsXOctagonFill,
                title: "Caution".to_string(),
            },
        }
    }
}

#[component]
pub fn Callout(
    #[prop(into)] colour: String,
    #[prop(into)] icon: &'static icondata_core::IconData,
    #[prop(into)] title: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class={format!("max-w-[65ch] my-4 p-2 {}", colour.clone())}>
            <div class="flex flex-row">
                <Icon icon />
                <p class="font-bold">{title.clone()}</p>
            </div>
            <article class="prose dark:prose-invert">
                {children()}
            </article>
        </div>
    }
}
