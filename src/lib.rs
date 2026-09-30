#![warn(missing_docs)]

//! # aur-depend
//!
//! aur-depends is a dependency solving library for the AUR
//!
//! See [`Resolver`](struct.Resolver.html) for more info.
//!
//! [`Actions::resolved_dependencies`] exposes the selected source of each
//! consuming AUR or PKGBUILD package's dependencies. It includes automatic provider
//! choices and reused satisfiers. This is a resolution snapshot, not a guarantee of
//! successful builds or satisfaction in another environment. Explicit targets do not
//! rewrite earlier consumers' entries; normal resolver precedence still applies to
//! later consumers. Source identities distinguish build, repository, installed, and
//! assumed selections without selecting providers again.

mod actions;
mod base;
mod cb;
mod error;
mod pkgbuild;
mod resolve;
mod satisfies;
#[cfg(test)]
mod tests;
mod upgrade;

pub use crate::actions::*;
pub use crate::base::*;
pub use crate::error::*;
pub use crate::pkgbuild::*;
pub use crate::resolve::*;
pub use crate::upgrade::*;
