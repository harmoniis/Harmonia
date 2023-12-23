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