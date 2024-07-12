use std::borrow::Cow;
use std::fmt::Display;
use std::num::NonZeroU32;

use anyhow::Context;

pub type UniqueId = NonZeroU32;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct IdContext<'a> {
    pub package: Option<Cow<'a, str>>,
    pub domain: Option<Cow<'a, str>>,
}

impl<'a> IdContext<'a> {
    pub const fn full_ref(package: &'a str, domain: &'a str) -> Self {
        assert!(!package.is_empty() && !domain.is_empty());
        Self {
            package: Some(Cow::Borrowed(package)),
            domain: Some(Cow::Borrowed(domain)),
        }
    }

    pub const fn empty() -> Self {
        Self {
            package: None,
            domain: None,
        }
    }

    pub fn with_package(mut self, package: impl Into<Cow<'a, str>>) -> Self {
        let package = package.into();
        assert!(!package.is_empty());
        self.package = Some(package);
        self
    }

    pub fn with_domain(mut self, domain: impl Into<Cow<'a, str>>) -> Self {
        let domain = domain.into();
        assert!(!domain.is_empty());
        self.domain = Some(domain);
        self
    }

    fn ensure_package(&self) -> anyhow::Result<Cow<'a, str>> {
        self.package
            .clone()
            .context("package not known in this context")
    }

    fn ensure_domain(&self) -> anyhow::Result<Cow<'a, str>> {
        self.domain
            .clone()
            .context("domain not known in this context")
    }
}

/// Qualified entity ID.
///
/// Example: `/gpio.v1/Chip/my_chip`
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct QualifiedId<'a> {
    pub package: Cow<'a, str>,
    pub domain: Cow<'a, str>,
    pub name: Cow<'a, str>,
}

impl<'a> QualifiedId<'a> {
    pub fn check_if_str_is_valid(s: &'a str) -> bool {
        Self::parse_absolute(s).is_ok()
    }

    pub fn parse_absolute(s: &'a str) -> anyhow::Result<Self> {
        let s = s.strip_prefix('/').context("missing leading slash")?;
        let mut segments = s
            .split('/')
            .map(|s| percent_encoding::percent_decode_str(s).decode_utf8());
        let package = segments.next().context("missing package")??;
        anyhow::ensure!(package != "", "package must not be empty");
        let domain = segments.next().context("missing domain")??;
        anyhow::ensure!(domain != "", "domain must not be empty");
        let name = segments.next().context("missing name")??;
        anyhow::ensure!(name != "", "name must not be empty");
        anyhow::ensure!(segments.next().is_none(), "too many segments");

        Ok(Self {
            package,
            domain,
            name,
        })
    }

    pub fn parse_with_context(s: &'a str, context: &IdContext<'a>) -> anyhow::Result<Self> {
        if s.starts_with('/') {
            return Self::parse_absolute(s);
        }

        // relative
        let mut rev_segments = s
            .split('/')
            .rev()
            .map(|s| percent_encoding::percent_decode_str(s).decode_utf8());

        // name is always given
        let name = rev_segments.next().context("missing name")??;
        anyhow::ensure!(name != "", "name must not be empty");
        match rev_segments.next().transpose()? {
            Some(domain) => match rev_segments.next().transpose()? {
                Some(package) => {
                    // all three given
                    anyhow::ensure!(rev_segments.next().is_none(), "too many segments");
                    Ok(Self {
                        package,
                        domain,
                        name,
                    })
                }
                // name and domain given
                None => Ok(Self {
                    package: context.ensure_package()?,
                    domain,
                    name,
                }),
            },
            // only name explicitly given
            None => Ok(Self {
                package: context.ensure_package()?,
                domain: context.ensure_domain()?,
                name,
            }),
        }
    }

    pub fn to_static(&self) -> QualifiedId<'static> {
        QualifiedId {
            package: Cow::Owned(self.package.to_string()),
            domain: Cow::Owned(self.domain.to_string()),
            name: Cow::Owned(self.name.to_string()),
        }
    }
}

impl<'a> Display for QualifiedId<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "/{}/{}/{}",
            percent_encoding::utf8_percent_encode(&self.package, url_encoding::PATH_SEGMENT),
            percent_encoding::utf8_percent_encode(&self.domain, url_encoding::PATH_SEGMENT),
            percent_encoding::utf8_percent_encode(&self.name, url_encoding::PATH_SEGMENT),
        )
    }
}

mod url_encoding {
    //! Taken from <https://github.com/servo/rust-url/blob/54346fa288e16b25b71c45149d7067c752b450e0/url/src/parser.rs#L37>

    use percent_encoding::{AsciiSet, CONTROLS};

    /// https://url.spec.whatwg.org/#fragment-percent-encode-set
    const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

    /// https://url.spec.whatwg.org/#path-percent-encode-set
    const PATH: &AsciiSet = &FRAGMENT.add(b'#').add(b'?').add(b'{').add(b'}');

    pub const PATH_SEGMENT: &AsciiSet = &PATH.add(b'/').add(b'%');
}
