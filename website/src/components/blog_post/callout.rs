use markdown::translate::node::CalloutKind;
use yew::{classes, function_component, html, props, Children, Html, Properties};
use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq)]
pub struct CalloutProps {
    colour: String,
    icon: IconId,
    title: String,
    #[prop_or_default]
    pub children: Children,
}

impl From<CalloutKind> for CalloutProps {
    fn from(value: CalloutKind) -> Self {
        match value {
            CalloutKind::Note => props!(CalloutProps {
                colour: "bg-blue/50",
                icon: IconId::BootstrapInfoCircleFill,
                title: "Note",
            }),
            CalloutKind::Tip => props!(CalloutProps {
                colour: "bg-green/50",
                icon: IconId::BootstrapLightbulbFill,
                title: "Tip",
            }),
            CalloutKind::Important => props!(CalloutProps {
                colour: "bg-purple/50",
                icon: IconId::BootstrapCheckCircleFill,
                title: "Important",
            }),
            CalloutKind::Warning => props!(CalloutProps {
                colour: "bg-yellow/50",
                icon: IconId::BootstrapExclamationTriangleFill,
                title: "Warning",
            }),
            CalloutKind::Caution => props!(CalloutProps {
                colour: "bg-red/50",
                icon: IconId::BootstrapXOctagonFill,
                title: "Caution",
            }),
        }
    }
}

#[function_component(Callout)]
pub fn callout(props: &CalloutProps) -> Html {
    html! {
        <div class={classes!("max-w-[65ch]", "my-4", "p-2", props.colour.clone())}>
            <div class="flex flex-row">
                <Icon class="p-1" icon_id={props.icon} />
                <p class="font-bold">{props.title.clone()}</p>
            </div>
            <article class="prose dark:prose-invert">
                {props.children.clone()}
            </article>
        </div>
    }
}
