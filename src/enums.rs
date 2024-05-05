use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Technology {
    JavaScript,
    TypeScript,
    Java,
    Python,
    Ruby,
    CSharp,
    Go,
    Rust,
    Swift,
    Kotlin,
    Scala,
    PHP,
    CPlusPlus,
    C,
    Haskell,
    Lua,
    Perl,
    R,
    Shell,
    SQL,
    Git,
    Docker,
    Kubernetes,
    Jenkins,
    GitHubActions,
}

impl Technology {
    pub fn to_string(&self) -> &'static str {
        match *self {
            Technology::JavaScript => "JavaScript",
            Technology::TypeScript => "TypeScript",
            Technology::Java => "Java",
            Technology::Python => "Python",
            Technology::Ruby => "Ruby",
            Technology::CSharp => "C#",
            Technology::Go => "Go",
            Technology::Rust => "Rust",
            Technology::Swift => "Swift",
            Technology::Kotlin => "Kotlin",
            Technology::Scala => "Scala",
            Technology::PHP => "PHP",
            Technology::CPlusPlus => "C++",
            Technology::C => "C",
            Technology::Haskell => "Haskell",
            Technology::Lua => "Lua",
            Technology::Perl => "Perl",
            Technology::R => "R",
            Technology::Shell => "Shell",
            Technology::SQL => "SQL",
            Technology::Git => "Git",
            Technology::Docker => "Docker",
            Technology::Kubernetes => "Kubernetes",
            Technology::Jenkins => "Jenkins",
            Technology::GitHubActions => "GitHub Actions",
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum Language {
    Engilsh,
    Polski,
    Deutsch,
    Français,
    Español,
}

impl Language {
    pub fn to_string(&self) -> &'static str {
        match *self {
            Language::Engilsh => "English",
            Language::Polski => "Polski",
            Language::Deutsch => "Deutsch",
            Language::Français => "Français",
            Language::Español => "Español",
        }
    }
}