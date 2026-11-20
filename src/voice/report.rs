use core::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct DependencyReport {
    pub opus: Option<String>,
    pub ffmpeg: Option<String>,
    pub dave: bool,
}

impl fmt::Display for DependencyReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "opus: {}", self.opus.as_deref().unwrap_or("missing"))?;
        writeln!(f, "ffmpeg: {}", self.ffmpeg.as_deref().unwrap_or("missing"))?;
        write!(f, "dave: {}", self.dave)
    }
}

#[must_use]
pub fn generate_dependency_report() -> DependencyReport {
    DependencyReport {
        opus: Some(opus2::version().to_owned()),
        ffmpeg: ffmpeg_version(),
        dave: true,
    }
}

fn ffmpeg_version() -> Option<String> {
    let output = std::process::Command::new("ffmpeg")
        .arg("-version")
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8(output.stdout).ok()?;
    text.lines().next().map(str::to_owned)
}

#[cfg(test)]
mod tests {
    use super::generate_dependency_report;

    #[test]
    fn the_report_names_every_dependency() {
        let report = generate_dependency_report();
        let opus = report.opus.as_deref().unwrap_or("missing");
        assert!(opus.to_lowercase().contains("opus"), "{opus}");
        assert!(report.dave);
        let text = report.to_string();
        assert!(text.contains("opus:"));
        assert!(text.contains("ffmpeg:"));
        assert!(text.contains("dave: true"));
    }
}
