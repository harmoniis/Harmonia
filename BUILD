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
# limitations under the License

load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")
load("@rules_cc//cc:defs.bzl", "cc_library")
load("@rules_rust//bindgen:bindgen.bzl", "rust_bindgen_library")
load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test")

config_setting(
    name = "freebsd-x86_64",
    values = {
        "cpu": "x86_64",
        "os": "freebsd",
    },
)

config_setting(
    name = "macos-apple-silicon",
    values = {
        "apple_platform_type": "macos",
        "cpu": "arm64",
    },
)

config_setting(
    name = "macos-universal",
    values = {
        "apple_platform_type": "macos",
        "cpu": "x86_64",
    },
)

config_setting(
    name = "ios",
    values = {
        "apple_platform_type": "ios",
        "cpu": "arm64",
    },
)

config_setting(
    name = "ios-simulator-universal",
    values = {
        "apple_platform_type": "ios",
        "cpu": "ios_x86_64",
    },
)

config_setting(
    name = "ios-simulator-apple-silicon",
    values = {
        "apple_platform_type": "ios",
        "cpu": "ios_arm64",
    },
)

config_setting(
    name = "android",
    values = {
        "cpu": "aarch64",
        "os": "android",
    },
)


rust_library(
    name = "lib",
    aliases = aliases(),
    deps = all_crate_deps(
        normal = True,
    ),
    proc_macro_deps = all_crate_deps(
        proc_macro = True,
    ),
)

rust_test(
    name = "unit_test",
    crate = ":lib",
    aliases = aliases(
        normal_dev = True,
        proc_macro_dev = True,
    ),
    deps = all_crate_deps(
        normal_dev = True,
    ),
    proc_macro_deps = all_crate_deps(
        proc_macro_dev = True,
    ),
)


cc_library(
    name = "sys",
    srcs = select({
        ":freebsd-x86_64": ["sys.h"],
        ":android_aarch64": ["source_android_aarch64.cpp"],
        ":ios_aarch64": ["source_ios_aarch64.cpp"],
        ":macos_aarch64": ["source_macos_aarch64.cpp"],
        "//conditions:default": ["source_default.cpp"],
    }),
    srcs = ["sys.h"],
)

rust_bindgen_library(
    name = "sys_bindgen",
    bindgen_flags = [
        "--allowlist-var=sys_.*",
    ],
    cc_lib = ":sys",
    header = "sys.h",
)