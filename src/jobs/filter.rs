use maud::{Markup, html};

pub fn single(value: impl Into<String>) -> Markup {
    html! {
      (single_color(value,"bg-base-300"))
    }
}

pub fn single_color(value: impl Into<String>, color: impl Into<String>) -> Markup {
    let color = color.into();
    let value = value.into();
    let classes = format!("btn {}", color);
    html! {
    input class=(classes) type="radio" name="frameworks" aria-label=(value);
    }
}

pub fn render(inputs: Vec<Markup>) -> Markup {
    html! {
        form class="filter" {
            input class="btn btn-square" type="reset" value="×";
            @for f in inputs{
                (f)
            }

        }
    }
}
