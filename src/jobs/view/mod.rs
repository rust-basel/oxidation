use maud::{Markup, html};

pub mod card;
mod filter;

pub fn index() -> Markup {
    html! {
    (maud::DOCTYPE)
    html data-theme="rust-ember" {
        head {
            script src="/_assets/htmx.js" {}
            script src="/_assets/tw.js" {}
            link href="/_assets/daisy.css" rel="stylesheet" type="text/css";
            link href="/_assets/themes.css" rel="stylesheet" type="text/css";
            meta charset="UTF-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            title {
                "Oxidation"
            }


        }
        body {
            header class="mb-8 text-center" {
                h1 class="text-xl text-base mb-2" {
                    "Oxidation"
                }
                p class="text-sm text-neutral-content" {
                    "Hand-picked opportunities for Rustaceans \u{1f980}"
                }
            }
            main {


                div ."mx-3 mb-5" {
                    (filter::render(
                        vec![filter::single("Full Rust"), filter::single("Some Rust")]
                    ))
                    (filter::render(
                        vec![ filter::single_color("On-Site", "bg-primary")]
                    ))
                }


                div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3 mx-3" {



               div hx-get="/jobs" hx-trigger="load" {

               }
            }
            }


            form hx-put="/jobs" hx-trigger="load" hx-target="null" {
                input type="hidden" id="uri" name="uri" value="https://rust-basel.ch" {}
                input type="hidden" id="title" name="title" value="https://rust-basel.ch" {}
                input type="hidden" id="preface" name="preface" value="https://rust-basel.ch" {}
                input type="hidden" id="description" name="description" value="https://rust-basel.ch" {}
            }
        }
    }
    }
}
