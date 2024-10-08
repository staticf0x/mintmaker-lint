pub fn get_available_managers() -> Vec<String> {
    vec![
        "ansible",
        "ansible-galaxy",
        "argocd",
        "asdf",
        "azure-pipelines",
        "batect",
        "batect-wrapper",
        "bazel",
        "bazel-module",
        "bazelisk",
        "bicep",
        "bitbucket-pipelines",
        "bitrise",
        "buildkite",
        "bun",
        "bundler",
        "cake",
        "cargo",
        "cdnurl",
        "circleci",
        "cloudbuild",
        "cocoapods",
        "composer",
        "conan",
        "copier",
        "cpanfile",
        "crossplane",
        "deps-edn",
        "devcontainer",
        "docker-compose",
        "dockerfile",
        "droneci",
        "fleet",
        "flux",
        "fvm",
        "git-submodules",
        "github-actions",
        "gitlabci",
        "gitlabci-include",
        "glasskube",
        "gleam",
        "gomod",
        "gradle",
        "gradle-wrapper",
        "helm-requirements",
        "helm-values",
        "helmfile",
        "helmsman",
        "helmv3",
        "hermit",
        "homebrew",
        "html",
        "jenkins",
        "jsonnet-bundler",
        "kotlin-script",
        "kubernetes",
        "kustomize",
        "leiningen",
        "maven",
        "maven-wrapper",
        "meteor",
        "mint",
        "mise",
        "mix",
        "nix",
        "nodenv",
        "npm",
        "nuget",
        "nvm",
        "ocb",
        "osgi",
        "pep621",
        "pip-compile",
        "pip_requirements",
        "pip_setup",
        "pipenv",
        "poetry",
        "rpm",
        "pre-commit",
        "pub",
        "puppet",
        "pyenv",
        "ruby-version",
        "runtime-version",
        "sbt",
        "scalafmt",
        "setup-cfg",
        "swift",
        "tekton",
        "terraform",
        "terraform-version",
        "terragrunt",
        "terragrunt-version",
        "tflint-plugin",
        "travis",
        "velaci",
        "vendir",
        "woodpecker",
    ]
    .into_iter()
    .map(|manager| manager.to_string())
    .collect()
}