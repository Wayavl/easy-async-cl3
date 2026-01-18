use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OpenCLVersion {
    V1_0,
    V1_1,
    V1_2,
    V2_0,
    V2_1,
    V2_2,
    V3_0,
}

impl FromStr for OpenCLVersion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Expected format: "OpenCL X.Y <extra info>"
        if !s.starts_with("OpenCL ") {
            return Err(());
        }

        let version_part = s.split_whitespace().nth(1).ok_or(())?;
        let mut parts = version_part.split('.');
        let major = parts.next().ok_or(())?.parse::<u32>().map_err(|_| ())?;
        let minor = parts.next().unwrap_or("0").parse::<u32>().map_err(|_| ())?;

        match (major, minor) {
            (1, 0) => Ok(Self::V1_0),
            (1, 1) => Ok(Self::V1_1),
            (1, 2) => Ok(Self::V1_2),
            (2, 0) => Ok(Self::V2_0),
            (2, 1) => Ok(Self::V2_1),
            (2, 2) => Ok(Self::V2_2),
            (3, 0) => Ok(Self::V3_0),
            (m, _) if m > 3 => Ok(Self::V3_0), // Cap at 3.0 for now
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for OpenCLVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::V1_0 => "1.0",
            Self::V1_1 => "1.1",
            Self::V1_2 => "1.2",
            Self::V2_0 => "2.0",
            Self::V2_1 => "2.1",
            Self::V2_2 => "2.2",
            Self::V3_0 => "3.0",
        };
        write!(f, "{}", s)
    }
}
