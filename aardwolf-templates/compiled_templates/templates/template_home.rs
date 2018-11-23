use std::io::{self, Write};
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use ::templates::{Html,ToHtml};
use gettext::Catalog;
use crate::templates::{home::{feed, footer, nav}, shortcuts, new_post};

pub fn home(out: &mut Write, catalog: &Catalog, profile_link: &str, username: &str)
-> io::Result<()> {
nav(out, catalog)?;
write!(out, "\n<section class=\"section\">\n    ")?;
shortcuts(out, catalog, profile_link, username)?;
write!(out, "\n</section>\n<section class=\"section\">\n    <div class=\"container\">\n        ")?;
new_post(out, catalog)?;
write!(out, "\n        ")?;
feed(out, catalog)?;
write!(out, "\n    </div>\n</section>\n")?;
footer(out, catalog)?;
write!(out, "\n")?;
Ok(())
}