
#[derive(Debug, PartialEq)]
struct VersionManager {
    major: u32,
    minor: u32,
    patch: u32,
    history: Vec<(u32, u32, u32)>,
}

#[derive(Debug, PartialEq)]
pub enum VMError {
    InvalidVersion, // for the `from_version` function
    NoHistory,      // for the `rollback` method
}

impl VersionManager {

    fn new() -> Self {
        return Self {
            major: 0,
            minor: 0,
            patch: 1,
            history: Vec::new()
        }
    }

    fn from_version(version: &str) -> Result<Self, VMError> {
        if version == "" {
            return Ok(Self::new());
        }

        let mut parts = version.split(".").into_iter();
        let major = parts.next().unwrap_or("0").parse::<u32>().or_else(|_| Err(VMError::InvalidVersion))?;
        let minor = parts.next().unwrap_or("0").parse::<u32>().or_else(|_| Err(VMError::InvalidVersion))?;
        let patch = parts.next().unwrap_or("0").parse::<u32>().or_else(|_| Err(VMError::InvalidVersion))?;
        let history = vec![];

        Ok(Self {
            major,
            minor,
            patch,
            history
        })
    }

    fn major(&mut self) -> &mut Self {
        self.history.push((self.major, self.minor, self.patch));
        self.major += 1;
        self.minor = 0;
        self.patch = 0;
        self
    }

    fn minor(&mut self) -> &mut Self {
        self.history.push((self.major, self.minor, self.patch));
        self.minor += 1;
        self.patch = 0;
        self
    }

    fn patch(&mut self) -> &mut Self {
        self.history.push((self.major, self.minor, self.patch));
        self.patch += 1;
        self
    }

    fn rollback(&mut self) -> Result<&mut Self, VMError> {
        if self.history.len() == 0 {
            return Err(VMError::NoHistory);
        }

        let (major, minor, patch) = self.history.pop().unwrap();
        self.major = major;
        self.minor = minor;
        self.patch = patch;
        Ok(self)
    }

    fn release(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod sample_tests {
    use super::*;

    #[test]
    fn invalid_rollbacks() {
        assert_eq!(VersionManager::new().rollback(), Err(VMError::NoHistory));
        assert_eq!(VersionManager::new().major().rollback()
            // double rollback
            .and_then(|vm| vm.rollback()),
            Err(VMError::NoHistory)
        );

        assert_eq!(VersionManager::from_version("8.2.4").unwrap().patch().rollback()
            // double rollback
            .and_then(|vm| vm.rollback()),
            Err(VMError::NoHistory)
        );
    }

}
