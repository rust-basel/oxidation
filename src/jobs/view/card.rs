use maud::{Markup, html};

use crate::model::Job;

pub fn all(jobs: Vec<Job>) -> Markup {
    html! {
        @for job in jobs{
            (job_card(job.title(), job.preface(), vec![html!{
                div class="badge badge-accent" {
                    "On-site"
                }
            }], job.description(), html!{
                a class="btn btn-secondary btn-sm bg-base" href=(job.uri()) {
                "Apply"
            }}))
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
        div class="card bg-base-100 shadow-md border border-base-300" {
            div class="card-body" {
                h2 class="card-title text-base" {
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
