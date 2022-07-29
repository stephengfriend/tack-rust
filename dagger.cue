package tack

import (
  "dagger.io/dagger"
  "dagger.io/dagger/core"

  "universe.dagger.io/alpha/rust"
)

dagger.#Plan & {
	client: filesystem: "./target": write: contents: actions.build."rust".output
  client: network: "unix:///var/run/docker.sock": connect: dagger.#Socket

  actions: {
    _source: core.#Source & {
      path: "."
      exclude: [
        // Folders
        ".devcontainer",
        ".vscode",
        "cue.mod",
        "target",
        // Files
        "*.cue",
        "*.md",
        ".git",
      ]
    }
    build: {
      "rust": {
        container: rust.#Container & {
          source: actions._source.output
          "env": {
            CARGO_TARGET_DIR: "/output/"
          }
          command: flags: build: true
          export: directories: "/output": _
        }
        output: container.export.directories."/output"
      }
    }
  }
}