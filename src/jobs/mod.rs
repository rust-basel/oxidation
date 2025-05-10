use maud::{Markup, html};

mod root;

pub use root::router;

pub fn ui() -> Markup {
    html! {
    (maud::DOCTYPE)
    html data-theme="rust-ember" {
        head {
            script src="/_assets/tw.js" {}
            link href="/_assets/daisy.css" rel="stylesheet" type="text/css";
            link href="/_assets/themes.css" rel="stylesheet" type="text/css";
            meta charset="UTF-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            title {
                "Oxidation Jobs"
            }


        }
        body {
            header class="mb-8 text-center" {
                h1 class="text-4xl font-bold text-primary mb-2" {
                    "Rust Job Postings"
                }
                p class="text-sm text-neutral-content" {
                    "Hand-picked opportunities for Rustaceans \u{1f980}"
                }
            }
            main class="grid gap-6 md:grid-cols-2 lg:grid-cols-3" {

               (job_card("Krypto Eng.", "Write the new app layers", vec![html!{
                   div class="badge badge-accent" {
                       "On-site"
                   }
               }], "This job is great because lorem ipsum", html!{ button class="btn btn-primary btn-sm" {
                   "Apply"
               }}))

            }
        }
    }
    }
}

fn job_card(
    title: impl Into<String>,
    subtitle: impl Into<String>,
    tags: Vec<Markup>,
    description: impl Into<String>,
    action: Markup,
) -> Markup {
    let title = title.into();
    let subtitle = subtitle.into();
    let description = description.into();
    html! {
        div class="card bg-base-100 shadow-md border border-neutral" {
            div class="card-body" {
                h2 class="card-title text-primary" {
                    (title)
                }
                p class="text-sm text-neutral-content" {
                    (subtitle)
                }
                @for badge in tags{
                        (badge)
                }
                p class="mt-4" {
                    (description)
                }
                div class="card-actions justify-end mt-4" {
                    (action)

                }
            }
        }
    }
}
