use serde::{Serialize, Deserialize};
use std::{
    fmt::Display,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemVer {
    major: u16,
    minor: u16,
    patch: u16,
}

impl SemVer {
    fn new(major: u16, minor: u16, patch: u16) -> SemVer {
        SemVer {
            major,
            minor,
            patch,
        }
    }

    fn new_short(major: u16) -> SemVer {
        Self::new(major, 0, 0)
    }
}

impl Default for SemVer {
    fn default() -> Self {
        Self::new_short(1)
    }
}

impl Display for SemVer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl From<&str> for SemVer {
    fn from(s: &str) -> Self {
        let vs: Vec<u16> = s.split(".").filter_map(|item| item.parse().ok()).collect();
        assert!(vs.len() == 3);
        SemVer {
            major: vs[0],
            minor: vs[1],
            patch: vs[2],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Program {
    name: String,
    release_history: Vec<SemVer>,
}

impl Program {
    pub fn new(name: String, release_history: Vec<SemVer>) -> Self { Self { name, release_history } }
}
