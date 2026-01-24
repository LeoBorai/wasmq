use std::{fmt::Display, str::FromStr};

use anyhow::{Error, Result, bail};
use semver::Version;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TaskIdentifier {
    pub namespace: String,
    pub name: String,
    pub version: Version,
}

impl TaskIdentifier {
    pub fn new(namespace: String, name: String) -> Self {
        let version = Version::new(0, 1, 0);

        Self {
            namespace,
            name,
            version,
        }
    }
}

impl Display for TaskIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}@{}", self.namespace, self.name, self.version)
    }
}

impl FromStr for TaskIdentifier {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("/").collect();

        if parts.len() == 2 {
            let namespace = parts[0].to_string();
            let name = parts[1].to_string();

            if namespace.is_empty() || name.is_empty() {
                bail!("Invalid task identifier format. Namespace and name cannot be empty.");
            }

            let name_parts: Vec<&str> = name.split("@").collect();

            if name_parts.len() == 2 {
                let name = name_parts[0].to_string();
                let version = Version::parse(name_parts[1]);

                if let Ok(version) = version {
                    return Ok(TaskIdentifier {
                        namespace,
                        name,
                        version,
                    });
                } else {
                    bail!("Invalid version format: {}", name_parts[1]);
                }
            }
        }

        bail!("Invalid Task Identifier. Expected format: <namespace>/<name>@<version>");
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use semver::Version;

    use super::TaskIdentifier;

    #[test]
    fn test_task_identifier_from_str() {
        let id_str = "leo/test@1.0.0";
        let id = TaskIdentifier::from_str(id_str).unwrap();
        assert_eq!(id.namespace, "leo");
        assert_eq!(id.name, "test");
        assert_eq!(id.version, Version::parse("1.0.0").unwrap());
    }
}
