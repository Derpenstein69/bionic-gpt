#![allow(non_snake_case)]
use daisy_rsx::*;
use db::queries::teams::Team;
use dioxus::prelude::*;
use assets::files::profile_svg;
use assets::files::button_select_svg;

#[component]
pub fn Page(teams: Vec<(String, String)>, team: Team) -> Element {
    if let Some(name) = &team.name.clone() {
        rsx! {
            turbo-frame {
                id: "teams-popup",
                class: "min-w-full",
                DropDown {
                    direction: Direction::Bottom,
                    button_text: "{name}",
                    prefix_image_src: profile_svg.name,
                    suffix_image_src: button_select_svg.name,
                    class: "min-w-full",
                    strong {
                        "Switch Teams"
                    },
                    for team in teams {
                        DropDownLink {
                            href: "{team.1}",
                            target: "_top",
                            "{team.0}"
                        }
                    }
                }
            }
        }
    } else {
        rsx! {
            turbo-frame {
                id: "teams-popup",
                class: "w-full",
                div {
                    class: "flex justify-center height-full w-full items-center",
                    h4 {
                        "BionicGPT"
                    }
                }
            }
        }
    }
}

pub fn team_popup(props: PageProps) -> String {
    crate::render(VirtualDom::new_with_props(Page, props))
}
