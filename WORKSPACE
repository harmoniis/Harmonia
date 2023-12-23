# Copyright 2023 Harmoniis Inc.
# 
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
# 
#     http://www.apache.org/licenses/LICENSE-2.0
# 
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

workspace(name = "harmonia")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")


# https://github.com/bazelbuild/rules_rust/releases
http_archive(
    name = "rules_rust",
    sha256 = "36ab8f9facae745c9c9c1b33d225623d976e78f2cc3f729b7973d8c20934ab95",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.31.0/rules_rust-v0.31.0.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains", "rust_repositories")



# Call this function to define the standard Rust toolchains
rust_repositories()

# Load rust_toolchain function
load("@rules_rust//rust:toolchain.bzl", "rust_toolchain")

# Define custom toolchains

#FreeBSD x86_64
rust_toolchain(
    name = "freebsd-x86_64",
    target_triple = "x86_64-unknown-freebsd",
    # Other necessary attributes like rustc flags, linker, etc.
)

# macOS Apple Silicon
rust_toolchain(
    name = "macOS-apple-silicon",
    target_triple = "aarch64-apple-darwin",
    # Other necessary attributes like rustc flags, linker, etc.
)

# macOS Universal
rust_toolchain(
    name = "macOS-universal",
    target_triple = "x86_64-apple-darwin",
    # Other necessary attributes like rustc flags, linker, etc.
)

# iOS Simulator Apple Silicon
rust_toolchain(
    name = "ios-sim-apple-silicon",
    target_triple = "aarch64-apple-ios-sim",
    # Other necessary attributes like rustc flags, linker, etc.
)

# iOS Simulator Universal
rust_toolchain(
    name = "ios-sim-universal",
    target_triple = "x86_64-apple-ios",
    # Other necessary attributes like rustc flags, linker, etc.
)

# iOS
rust_toolchain(
    name = "iOS",
    target_triple = "aarch64-apple-ios",
    # Other necessary attributes like rustc flags, linker, etc.
)

# Android
rust_toolchain(
    name = "android",
    target_triple = "aarch64-linux-android",
    # Other necessary attributes like rustc flags, linker, etc.
)

load("@rules_rust//crate_universe:defs.bzl", "crates_repository")

rust_register_toolchains(
    edition = "2021",
    versions = [
        "1.74.1"
    ],
)

crates_repository(
    name = "crate_index",
    cargo_lockfile = "//:Cargo.lock",
    lockfile = "//:Cargo.Bazel.lock",
    manifests = ["//:Cargo.toml"],
)

load("@crate_index//:defs.bzl", "crate_repositories")

crate_repositories()

rules_rust_dependencies()

# Bindings for the C/C++ dependency
load("@rules_rust//bindgen:repositories.bzl", "rust_bindgen_dependencies", "rust_bindgen_register_toolchains")

rust_bindgen_dependencies()

rust_bindgen_register_toolchains()

load("@rules_rust//bindgen:transitive_repositories.bzl", "rust_bindgen_transitive_dependencies")

rust_bindgen_transitive_dependencies()

