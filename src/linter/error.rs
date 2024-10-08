use colored::Colorize;

#[derive(Debug)]
pub enum LinterError {
    UseOfUnsupportedManager { manager: String },
    DoesntExtendMintMaker,
}

impl std::fmt::Display for LinterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UseOfUnsupportedManager { manager } => {
                write!(f, "Use of unsupported manager {}", manager.bold().red())
            }
            Self::DoesntExtendMintMaker => {
                write!(f, "Config doesn't extend MintMaker's default config, please add the following to your config:
\"extends\": [
    \"github>konflux-ci/mintmaker//config/renovate/renovate.json\"
]")
            }
        }
    }
}
