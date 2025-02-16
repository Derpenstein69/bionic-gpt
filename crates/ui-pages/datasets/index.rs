#![allow(non_snake_case)]
use assets::files::*;
use db::authz::Rbac;
use db::queries::{datasets::Dataset, models::Model};
use dioxus::prelude::*;
use daisy_rsx::*; 
use crate::app_layout::Layout;
use crate::app_layout::SideBar;

#[component]
pub fn Page(
    rbac: Rbac,
    team_id: i32,
    datasets: Vec<Dataset>,
    models: Vec<Model>,
) -> Element {
    rsx! {
        Layout {
            section_class: "normal",
            selected_item: SideBar::Datasets,
            team_id: team_id,
            rbac: rbac,
            title: "Datasets",
            header: rsx!(
                h3 { "Datasets" }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    drawer_trigger: "new-dataset-form",
                    button_scheme: ButtonScheme::Primary,
                    "Add Dataset"
                }
            ),

            if datasets.is_empty() {
                BlankSlate {
                    heading: "Looks like you don't have any datasets yet",
                    visual: nav_ccsds_data_svg.name,
                    description: "Datasets allow you to organize your documents like folders"
                }

                super::new::New {
                    models: models.clone(),
                    team_id: team_id,
                    combine_under_n_chars: 500,
                    new_after_n_chars: 1000,
                    _multipage_sections: true,
                }
            } else {
                Box {
                    class: "has-data-table",
                    BoxHeader {
                        title: "Datasets"
                    }
                    BoxBody {
                        table {
                            class: "table table-sm",
                            thead {
                                th { "Name" }
                                th { "Visibility" }
                                th { "Document Count" }
                                th { "Chunking Strategy" }
                                th {
                                    class: "text-right",
                                    "Action"
                                }
                            }
                            tbody {

                                for dataset in &datasets {
                                    tr {
                                        td {
                                            a {
                                                href: "{crate::routes::documents::index_route(team_id, dataset.id)}",
                                                "{dataset.name}"
                                            }
                                        }
                                        td {
                                            crate::prompts::visibility::VisLabel {
                                                visibility: dataset.visibility
                                            }
                                        }
                                        td { "{dataset.count}" }
                                        td {
                                            Label {
                                                label_role: LabelRole::Highlight,
                                                "By Title"
                                            }
                                            }
                                        td {
                                            class: "text-right",
                                            DropDown {
                                                direction: Direction::Left,
                                                button_text: "...",
                                                DropDownLink {
                                                    href: "{crate::routes::documents::index_route(team_id, dataset.id)}",
                                                    target: "_top",
                                                    "View"
                                                }
                                                DropDownLink {
                                                    drawer_trigger: format!("delete-trigger-{}-{}",
                                                        dataset.id, team_id),
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

                    for item in datasets {
                        super::delete::DeleteDrawer {
                            team_id: team_id,
                            id: item.id,
                            trigger_id: format!("delete-trigger-{}-{}", item.id, team_id)
                        }
                    }

                    super::new::New {
                        models: models.clone(),
                        team_id: team_id,
                        combine_under_n_chars: 500,
                        new_after_n_chars: 1000,
                        _multipage_sections: true,
                    }
                }
            }
        }
    }
}

pub fn index(props: PageProps) -> String {
    crate::render(VirtualDom::new_with_props(Page, props))
}
