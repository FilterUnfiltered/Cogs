use ariadne::{Label, Source};
use cogs_parser::error::{Error, ErrorKind};
use std::{ops::Range, sync::Arc};

type Span = Range<usize>;
type SpanWithFile = (Arc<str>, Span);

fn add_error(builder: &mut ariadne::ReportBuilder<'static, SpanWithFile>, span: Span, kind: ErrorKind, file: Arc<str>) {
    let label = Label::new((file, span));
    builder.add_label(match kind {
        ErrorKind::Nom(e) => label.with_message(e.description()),
        ErrorKind::Char(c) => label.with_message(format!("expected '{c}'")),
        ErrorKind::Context(s) => label.with_message(format!("while parsing {s}")),
        ErrorKind::Custom(s) => label.with_message(s),
    });
}

pub fn nom_diagnostic<'a>(main: &'a str, error: Error<&'a str>, file: &str) {
    let file = Arc::<str>::from(file);
    let (mut iter, info) = error.resolve_spans(main);
    let Some((span, kind)) = iter.next() else {
        return;
    };
    let mut report = ariadne::Report::build(ariadne::ReportKind::Error, (file.clone(), span.clone()));
    if let Some(message) = info.message {
        report.set_message(message);
    }
    add_error(&mut report, span, kind, file.clone());
    for (span, kind) in iter {
        add_error(&mut report, span, kind, file.clone());
    }
    let _ = report.finish().print((file, Source::from(main)));
}
