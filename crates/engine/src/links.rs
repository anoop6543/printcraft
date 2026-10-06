//! Where Split Happens lives on the web. One table, so the Help menu, the
//! About dialog, the home screen, the CLI and the README agree.

/// The app's name in URLs.
pub const APP: &str = "split-happens";

pub const WEBSITE: &str = "https://anoop6543.github.io/printcraft/";
pub const APP_PAGE: &str = "https://anoop6543.github.io/printcraft/mobile/";
pub const GITHUB: &str = "https://github.com/anoop6543/printcraft";

/// A link and the registry command that opens it.
#[derive(Clone, Copy, Debug)]
pub struct Link {
    pub command: &'static str,
    pub label: &'static str,
    pub url: &'static str,
    /// Lucide icon name.
    pub icon: &'static str,
}

/// In the order they are shown.
pub const LINKS: &[Link] = &[
    Link { command: "help.app_page", label: "Split Happens Mobile", url: APP_PAGE, icon: "smartphone" },
    Link { command: "help.github", label: "Split Happens on GitHub", url: GITHUB, icon: "code-xml" },
    Link { command: "help.website", label: "Split Happens website", url: WEBSITE, icon: "globe" },
];

pub fn for_command(id: &str) -> Option<&'static Link> {
    LINKS.iter().find(|l| l.command == id)
}

#[cfg(test)]
mod tests {
    #[test]
    fn urls_are_well_formed() {
        for l in super::LINKS {
            assert!(l.url.starts_with("https://"), "{}", l.url);
            assert!(crate::commands::command(l.command).is_some(), "{} is a registered command", l.command);
        }
    }
}
