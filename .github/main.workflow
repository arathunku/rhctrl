workflow "Run tests" {
  on = "push"
  resolves = ["Check Rust 1.31"]
}

action "Check Rust 1.31" {
  uses = "actions/docker/cli@latest"
  runs = "build -f .github/Dockerfile . -t rhctrl:latest"
}
