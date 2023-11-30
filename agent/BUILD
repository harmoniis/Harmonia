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


load("@rules_rust//rust:rust.bzl", "rust_binary", "rust_library")

def select_features(features: List[str]) -> List[str]:
    # Process the user provided features here and return the selected features
    return features

rust_library(
    name = "harmolib",
    crate_type = "lib",
    srcs = glob(["src/**/*.rs"]),
    features = select_features(features=["default", "environment", "integration", "backend"])
)

rust_binary(
    name = "harmonia",
    crate_type = "binary",
    srcs = glob(["src/**/*.rs"]),
    features = select_features(features=["default", "environment", "integration", "backend"])
)