#![feature(proc_macro_span)]

use std::env;
use std::fs;
use std::str::FromStr;
use std::os::unix::fs::MetadataExt;

use time::{OffsetDateTime, Month};
use proc_macro::Span;

#[proc_macro]
pub fn spooky(code: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match real_spooky(code) {
        Ok(code) => code,
        Err(err) => panic!("{}", err)
    }
}

fn real_spooky(code: proc_macro::TokenStream) -> Result<proc_macro::TokenStream, &'static str> {
    let here = Span::call_site();
    let this_file = here.source_file();
    let path = this_file.path();
    let cwd = env::current_dir().map_err(|_| "👻 where am i? 👻")?;
    let full_path = cwd.join(path);

    let metadata = fs::metadata(full_path).map_err(|_| "👻 that wasn't candy 👻")?;
    let mtime = OffsetDateTime::from_unix_timestamp(metadata.mtime()).map_err(|_| "🙀 not spooky enough 😿")?;
    let month = mtime.month();

    if month == Month::October {
        let code = code.to_string();
        let code = format!("unsafe {{ {} }}", code);
        Ok(proc_macro::TokenStream::from_str(&code).map_err(|_| "🎃 trick! 🎃")?)
    } else {
        Err("🙀 it's not the spooky season yet. 😿")
    }
}
