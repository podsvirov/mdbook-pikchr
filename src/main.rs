mod pikchr;
use crate::mdbook_pikchr::PikchrPreprocessor;
use clap::{App, Arg, ArgMatches};
use log::{debug, trace, warn};
use mdbook::book::{Book, BookItem, Chapter};
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use mdbook::utils::new_cmark_parser;
use pikchr::Pikchr;
use semver::{Version, VersionReq};
use std::io;
use std::process;

pub fn make_app() -> App<'static> {
    App::new("mdbook-pikchr")
        .about("A mdbook preprocessor to render pikchr code blocks as images in your book")
        .subcommand(
            App::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
}

fn main() {
    env_logger::init();

    trace!("in main");

    let matches = make_app().get_matches();

    let preprocessor = PikchrPreprocessor::new();

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(sub_args);
    } else if let Err(e) = handle_preprocessing(&preprocessor) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn handle_supports(sub_args: &ArgMatches) -> ! {
    let renderer = sub_args.value_of("renderer").expect("Required argument");

    if renderer == "html" {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    let book_version = Version::parse(&ctx.mdbook_version)?;
    let version_req = VersionReq::parse(mdbook::MDBOOK_VERSION)?;

    if !version_req.matches(&book_version) {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

mod mdbook_pikchr {
    use super::*;
    use pulldown_cmark::{CodeBlockKind, CowStr, Event, Tag};
    use pulldown_cmark_to_cmark::cmark;

    enum Align {
        Left,
        Center,
        Right,
    }

    pub struct PikchrPreprocessor;

    impl PikchrPreprocessor {
        pub fn new() -> PikchrPreprocessor {
            PikchrPreprocessor
        }

        pub fn render_pikchr(
            &self,
            ctx: &PreprocessorContext,
            chapter: &mut Chapter,
        ) -> Result<String, ()> {
            let mut buf = String::with_capacity(chapter.content.len());

            let mut should_render = false;
            let mut curly_quotes = false;
            if let Some(cfg_curly_quotes) = ctx.config.get("output.html.curly-quotes") {
                curly_quotes = cfg_curly_quotes.as_bool().unwrap_or(curly_quotes);
                debug!("curly_quotes: {:?}", curly_quotes);
            }
            let mut align_default = Align::Center;
            if let Some(cfg_align) = ctx.config.get("preprocessor.pikchr.align") {
                align_default = match cfg_align.as_str().unwrap_or("center") {
                    "left" => Align::Left,
                    "center" => Align::Center,
                    "right" => Align::Right,
                    _ => align_default,
                };
            }
            let mut align = align_default;
            let events =
                new_cmark_parser(&chapter.content, curly_quotes).map(|event| match event {
                    Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(CowStr::Borrowed(lang)))) => {
                        if lang.contains("pikchr") {
                            debug!("Start lang: pikchr");
                            should_render = true;
                            if lang.contains("left") {
                                align = Align::Left;
                            }
                            else if lang.contains("center") {
                                align = Align::Center;
                            }
                            else if lang.contains("right") {
                                align = Align::Right;
                            }
                            Event::Text(CowStr::Borrowed("\n"))
                        } else {
                            event
                        }
                    }
                    Event::Text(CowStr::Borrowed(code)) => {
                        if should_render {
                            debug!("Should render: {:?}", code);
                            let margin = match align {
                                Align::Left => "0 auto 0 0",
                                Align::Center => "0 auto",
                                Align::Right => "0 0 0 auto",
                            };
                            match Pikchr::render(code, None) {
                                Ok(svg) => Event::Html(
                                    format!(
                                    "<div style=\"margin:{};max-width:{}px\">\n{}\n</div>\n",
                                    margin,
                                    svg.width,
                                    svg.to_string()
                                )
                                    .into(),
                                ),
                                Err(err) => {
                                    Event::Html(format!("<code>{}</code>\n{}", code, err).into())
                                }
                            }
                        } else {
                            event
                        }
                    }
                    Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(CowStr::Borrowed(lang)))) => {
                        if lang.contains("pikchr") {
                            debug!("End lang: pikchr");
                            should_render = false;
                            Event::Text(CowStr::Borrowed("\n"))
                        } else {
                            event
                        }
                    }
                    _ => event,
                });

            cmark(events, &mut buf)
                .map(|_| buf)
                .map_err(|err| warn!("Markdown serialization failed: {}", err))
        }
    }

    impl Preprocessor for PikchrPreprocessor {
        fn name(&self) -> &str {
            "mdbook-pikchr"
        }

        fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
            if let Some(cfg) = ctx.config.get_preprocessor(self.name()) {
                if cfg.contains_key("dark-mode") {
                    // TODO: Implement dark mode
                }
            }

            book.for_each_mut(|item: &mut BookItem| {
                if let BookItem::Chapter(ref mut chapter) = *item {
                    debug!("chapter: {}\ncontent: {}\n", chapter.name, chapter.content);
                    let res = self
                        .render_pikchr(ctx, chapter)
                        .expect("Rendering pikchr failed");
                    debug!("result: {}\n", res);
                    chapter.content = res;
                }
            });

            Ok(book)
        }
    }
}
