#![allow(non_snake_case)]
use db::authz::Rbac;
use db::queries::{datasets::Dataset, documents::Document};
use dioxus::prelude::*;
use daisy_rsx::*; 
use crate::app_layout::{Layout, SideBar};
use assets::files::*;

#[component]
pub fn Page(rbac: Rbac, team_id: i32, dataset: Dataset, documents: Vec<Document>) -> Element {
    rsx! {
        Layout {
            section_class: "normal",
            selected_item: SideBar::Datasets,
            team_id: team_id,
            rbac: rbac,
            title: "{dataset.name} / Documents",
            header: rsx!(
                h3 { "{dataset.name} / Documents" }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    drawer_trigger: "upload-form",
                    button_scheme: ButtonScheme::Primary,
                    "Add Document"
                }
            ),

            if documents.is_empty() {
                BlankSlate {
                    heading: "Looks like this dataset doesn't have any documents yet",
                    visual: nav_ccsds_data_svg.name,
                    description: "Here you can upload documents in a range of formats",
                    primary_action_drawer: (
                        "Add a Document".to_string(),
                        "upload-form".to_string()
                    )
                }

                // The form to create an invitation
                super::upload::Upload {
                    upload_action: crate::routes::documents::upload_route(team_id, dataset.id)
                }
            } else {
                Box {
                    class: "has-data-table",
                    BoxHeader {
                        title: "Documents"
                    }
                    BoxBody {
                        table {
                            id: "documents",
                            class: "table table-sm",
                            thead {
                                th { "Name" }
                                th { "No. Chunks" }
                                th { "Content Size (Bytes)" }
                                th { "Status" }
                                th {
                                    class: "text-right",
                                    "Action"
                                }
                            }
                            tbody {
                                for doc in &documents {
                                    Row {
                                        doc: doc.clone(),
                                        team_id: team_id
                                    }
                                }
                            }
                        }
                    }
                }

                for doc in documents {
                    super::delete::DeleteDrawer {
                        team_id: team_id,
                        document_id: doc.id,
                        dataset_id: doc.dataset_id,
                        trigger_id: format!("delete-doc-trigger-{}-{}", doc.id, team_id)
                    }
                }

                // The form to create an invitation
                super::upload::Upload {
                    upload_action: crate::routes::documents::upload_route(team_id, dataset.id)
                }
            }
        }
    }
}

#[component]
pub fn Row(doc: Document, team_id: i32) -> Element {
    let text = doc.failure_reason.clone().unwrap().replace(['{', '"', ':', '}'], " ");
    let class = if doc.waiting > 0 || doc.batches == 0 {
        "processing"
    } else {
        "processing-finished"
    };
    rsx!(
        tr {
            class: class,
            td { "{doc.file_name}" }
            td { "{doc.batches}" }
            td { "{doc.content_size}" }
            td {
                if doc.waiting > 0 || doc.batches == 0 {
                    Label {
                        "Processing ({doc.waiting} remaining)"
                    }
                } else if doc.failure_reason.is_some() {
                    
                    ToolTip {
                        text: "{text}",
                        Label {
                            label_role: LabelRole::Danger,
                            "Failed"
                        }
                    }
                } else if doc.batches == 0 {
                    Label {
                        "Queued"
                    }
                } else if doc.fail_count > 0 {
                    Label {
                        label_role: LabelRole::Danger,
                        "Processed ({doc.fail_count} failed)"
                    }
                } else if doc.failure_reason.is_some() {
                    Label {
                        label_role: LabelRole::Danger,
                        "Failed"
                    }
                } else {
                    Label {
                        label_role: LabelRole::Success,
                        "Processed"
                    }
                }
            }
            td {
                class: "text-right",
                DropDown {
                    direction: Direction::Left,
                    button_text: "...",
                    DropDownLink {
                        drawer_trigger: format!("delete-doc-trigger-{}-{}", 
                            doc.id, team_id),
                        href: "#",
                        target: "_top",
                        "Delete Document"
                    }
                }
            }
        }
    )
}

pub fn index(props: PageProps) -> String {
    crate::render(VirtualDom::new_with_props(Page, props))
}
