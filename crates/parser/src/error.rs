use std::{borrow::Cow, fmt, ops::Range};

use nom::{
    error::{ContextError, FromExternalError, ParseError},
    Err,
};

#[derive(Debug)]
pub struct Error<I> {
    pub errors: Vec<(I, ErrorKind)>,
    pub message: Option<String>,
    pub notes: Vec<String>,
    pub help: Option<String>,
}

impl<I> Default for Error<I> {
    fn default() -> Self {
        Self {
            errors: Vec::new(),
            message: None,
            notes: Vec::new(),
            help: None,
        }
    }
}

impl<I> Error<I> {
    pub fn single(input: I, error: ErrorKind) -> Self {
        Self {
            errors: vec![(input, error)],
            ..Default::default()
        }
    }

    pub fn eof(input: I) -> Err<Self> {
        Err::Error(Self::single(
            input,
            ErrorKind::Nom(nom::error::ErrorKind::Eof),
        ))
    }

    pub fn make_custom(input: I, message: impl Into<Cow<'static, str>>) -> Self {
        let cow: Cow<'static, str> = message.into();
        Self::single(input, ErrorKind::Custom(cow.clone())).with_message(cow.to_string())
    }

    pub fn with_message(mut self, message: impl ToString) -> Self {
        self.message = Some(message.to_string());
        self
    }

    pub fn with_note(mut self, note: impl ToString) -> Self {
        self.notes.push(note.to_string());
        self
    }

    pub fn with_help(mut self, help: impl ToString) -> Self {
        self.help = Some(help.to_string());
        self
    }

    pub fn clear_message(mut self) -> Self {
        self.message = None;
        self
    }

    pub fn custom(input: I, message: impl Into<Cow<'static, str>>) -> Err<Self> {
        Err::Error(Self::make_custom(input, message))
    }

    pub fn custom_failure(input: I, message: impl Into<Cow<'static, str>>) -> Err<Self> {
        Err::Failure(Self::make_custom(input, message))
    }
}

#[derive(Debug)]
pub struct ReportInfo {
    pub message: Option<String>,
    pub notes: Vec<String>,
    pub help: Option<String>,
}

impl<'a> Error<&'a str> {
    pub fn resolve_spans(
        self,
        main: &'a str,
    ) -> (
        impl Iterator<Item = (Range<usize>, ErrorKind)> + use<'a>,
        ReportInfo,
    ) {
        // what in the hellspawn is use<'a>
        (
            self.errors.into_iter().map(|(input, kind)| {
                let main_ptr = main.as_ptr() as usize;
                let input_ptr = input.as_ptr() as usize;
                let start = input_ptr.abs_diff(main_ptr);
                let end = start + input.len();
                (start..end, kind)
            }),
            ReportInfo {
                message: self.message,
                notes: self.notes,
                help: self.help,
            },
        )
    }
}

impl<I: fmt::Display> fmt::Display for Error<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (input, error) in &self.errors {
            match error {
                ErrorKind::Nom(e) => writeln!(f, "{e:?} at:\n{input}")?,
                ErrorKind::Char(c) => writeln!(f, "expected '{c}' at:\n{input}")?,
                ErrorKind::Context(s) => writeln!(f, "in {s}, at:\n{input}")?,
                ErrorKind::Custom(s) => writeln!(f, "{s} at:\n{input}")?,
            }
        }
        Ok(())
    }
}

impl<I: fmt::Debug + fmt::Display> std::error::Error for Error<I> {}

impl<I> ContextError<I> for Error<I> {
    fn add_context(input: I, ctx: &'static str, mut other: Self) -> Self {
        other.errors.push((input, ErrorKind::Context(ctx)));
        other
    }
}

impl<I, E: fmt::Display> FromExternalError<I, E> for Error<I> {
    fn from_external_error(input: I, _kind: nom::error::ErrorKind, e: E) -> Self {
        Self::single(input, ErrorKind::Custom(Cow::Owned(e.to_string())))
    }
}

impl<I> ParseError<I> for Error<I> {
    fn from_error_kind(input: I, kind: nom::error::ErrorKind) -> Self {
        Self::single(input, ErrorKind::Nom(kind))
    }

    fn or(mut self, other: Self) -> Self {
        self.errors.extend(other.errors);
        self.notes.extend(other.notes);
        if let Some(message) = other.message {
            if self.message.is_none() {
                self.message = Some(message);
            }
        }

        if let Some(help) = other.help {
            if self.help.is_none() {
                self.help = Some(help);
            }
        }

        self
    }

    fn append(input: I, kind: nom::error::ErrorKind, mut other: Self) -> Self {
        other.errors.push((input, ErrorKind::Nom(kind)));
        other
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    Nom(nom::error::ErrorKind),
    Char(char),
    Context(&'static str),
    Custom(Cow<'static, str>),
}

impl ErrorKind {
    pub fn custom(message: impl Into<Cow<'static, str>>) -> Self {
        Self::Custom(message.into())
    }
}
