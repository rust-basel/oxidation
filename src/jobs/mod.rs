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
                div class="card bg-base-100 shadow-md border border-neutral" {
                    div class="card-body" {
                        h2 class="card-title text-primary" {
                            "Senior Rust Backend Engineer"
                        }
                        p class="text-sm text-neutral-content" {
                            "\u{1f9e0} Systems-level thinking \u{2022} Remote (Worldwide)"
                        }
                        div class="badge badge-secondary" {
                            "Full-time"
                        }
                        div class="badge badge-accent" {
                            "Remote"
                        }
                        p class="mt-4" {
                            "Join a blockchain infrastructure startup building the next-gen WASM execution engine."
                        }
                        div class="card-actions justify-end mt-4" {
                            button class="btn btn-primary btn-sm" {
                                "Apply"
                            }
                        }
                    }
                }
                div class="card bg-base-100 shadow-md border border-neutral" {
                    div class="card-body" {
                        h2 class="card-title text-primary" {
                            "Rust Embedded Developer"
                        }
                        p class="text-sm text-neutral-content" {
                            "\u{1f6e0}\u{fe0f} IoT & Edge Devices \u{2022} Munich, Germany"
                        }
                        div class="badge badge-secondary" {
                            "Contract"
                        }
                        div class="badge badge-accent" {
                            "On-site"
                        }
                        p class="mt-4" {
                            "Work on firmware and device drivers for industrial sensors using bare-metal Rust."
                        }
                        div class="card-actions justify-end mt-4" {
                            button class="btn btn-primary btn-sm" {
                                "Apply"
                            }
                        }
                    }
                }
                div class="card bg-base-100 shadow-md border border-neutral" {
                    div class="card-body" {
                        h2 class="card-title text-primary" {
                            "Rust + WebAssembly Engineer"
                        }
                        p class="text-sm text-neutral-content" {
                            "\u{1f30d} Fintech App \u{2022} Remote (Europe)"
                        }
                        div class="badge badge-secondary" {
                            "Part-time"
                        }
                        div class="badge badge-accent" {
                            "Remote"
                        }
                        p class="mt-4" {
                            "Build performant WebAssembly modules in Rust for high-frequency trading dashboards."
                        }
                        div class="card-actions justify-end mt-4" {
                            button class="btn btn-primary btn-sm" {
                                "Apply"
                            }
                        }
                    }
                }
            }
        }
    }
    }
}
