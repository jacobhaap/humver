# Human Versioning
Represents human age at different time scales through semantic human versioning.

Given a version number MAJOR.MINOR.PATCH, increment the:

 1. MAJOR version every 10 years for the number of decades lived
 2. MINOR version for each year within the current decade (permitted range 0-9)
 3. PATCH version for each month within the current year (permitted range 0-11)

Human Versioning (HumVer) is based on [Semantic Versioning (SemVer)](https://semver.org/).

Unlike SemVer, HumVer does not support labels for pre-release and build metadata, as a connection between these components and human age has yet to be established.
