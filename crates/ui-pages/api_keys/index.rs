#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use assets::files::*;
use daisy_rsx::*;
use db::{authz::Rbac, ApiKey, Prompt};
use dioxus::prelude::*;

#[component]
pub fn Page(
    rbac: Rbac,
    team_id: i32,
    api_keys: Vec<ApiKey>,
    prompts: Vec<Prompt>,
) -> Element {
    rsx! {
        if api_keys.is_empty() {
            Layout {
                section_class: "normal",
                selected_item: SideBar::ApiKeys,
                team_id: team_id,
                rbac: rbac,
                title: "API Keys",
                header: rsx! {
                    h3 { "API Keys" }
                },
                BlankSlate {
                    heading: "Looks like you don't have any API keys",
                    visual: empty_api_keys_svg.name,
                    description: "API Keys allow you to access our programming interface",
                    primary_action_drawer: Some(("New API Key".to_string(), "create-api-key".to_string()))
                },
                super::form::Form {
                    team_id: team_id,
                    prompts: prompts.clone()
                }
            }
        } else {
            Layout {
                section_class: "normal",
                selected_item: SideBar::ApiKeys,
                team_id: team_id,
                rbac: rbac,
                title: "API Keys",
                header: rsx!(
                    h3 { "API Keys" }
                    Button {
                        drawer_trigger: "create-api-key",
                        button_scheme: ButtonScheme::Primary,
                        "Add Key"
                    }
                ),
                Box {
                    class: "has-data-table",
                    BoxHeader {
                        title: "API Keys"
                    }
                    BoxBody {
                        table {
                            class: "table table-sm",
                            thead {
                                th { "Name" }
                                th { "API Key" }
                                th { "Prompt" }
                                th {
                                    class: "text-right",
                                    "Action"
                                }
                            }
                            tbody {
                                for key in &api_keys {
                                    tr {
                                        td {
                                            "{key.name}"
                                        }
                                        td {
                                            Input {
                                                value: key.api_key.clone(),
                                                name: "api_key"
                                            }
                                        }
                                        td {
                                            "{key.prompt_name}"
                                        }
                                        td {
                                            class: "text-right",
                                            DropDown {
                                                direction: Direction::Left,
                                                button_text: "...",
                                                DropDownLink {
                                                    drawer_trigger: format!("delete-trigger-{}-{}",
                                                        key.id, team_id),
                                                    href: "#",
                                                    target: "_top",
                                                    "Delete"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },

                for item in api_keys {
                    super::delete::DeleteDrawer {
                        team_id: team_id,
                        id: item.id,
                        trigger_id: format!("delete-trigger-{}-{}", item.id, team_id)
                    }
                }

                // Drawers have to be fairly high up in the hierarchy or they
                // get missed off in turbo::load
                super::form::Form {
                    team_id: team_id,
                    prompts: prompts.clone()
                }
            }
        }
    }
}

pub fn index(props: PageProps) -> String {
    crate::render(VirtualDom::new_with_props(Page, props))
}
