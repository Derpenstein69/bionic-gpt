#![allow(non_snake_case)]
use daisy_rsx::*;
use db::queries::conversations::History;
use dioxus::prelude::*;

#[component]
pub fn HistoryDrawer(
    trigger_id: String,
    team_id: i32,
    history: Vec<History>,
) -> Element {
    rsx! {
        Drawer {
            label: "Your History",
            trigger_id: &trigger_id,
            DrawerBody {
                for history in history {
                    li {
                        class: "w-full overflow-hidden truncate",
                        a {
                            href: "{crate::routes::console::conversation_route(team_id, history.id)}",
                            {history.summary.clone()}
                        }
                    }
                }
            }
            DrawerFooter {
            }
        }
    }
}
