workflow "Run tests" {
  on = "push"
  resolves = ["Check Rust 1.31"]
}

action "Check Rust 1.31" {
  uses = "actions/docker/cli@76ff57a6c3d817840574a98950b0c7bc4e8a13a8"
  runs = "build -f .github/Dockerfile . -t rhctrl:latest"
}
