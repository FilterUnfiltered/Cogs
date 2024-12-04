use core::fmt;
use std::future::Future;

pub trait Component {
    type Props;
    type Error;

    fn render<'a>(&'a self, props: Self::Props) -> impl Future<Output = Result<String, Self::Error>> + Send + 'a;
}

pub trait Render {
    fn render(&self) -> String;
}

impl Render for () {
    fn render(&self) -> String {
        String::new()
    }
}

impl Render for &str {
    fn render(&self) -> String {
        format!("{}", self)
    }
}

impl Render for String {
    fn render(&self) -> String {
        format!("{}", self)
    }
}

impl<T: Render> Render for &T {
    fn render(&self) -> String {
        (**self).render()
    }
}

impl<T: Render> Render for Box<T> {
    fn render(&self) -> String {
        (**self).render()
    }
}
