// Copyright 2023 Harmoniis Inc.
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     http://www.apache.org/licenses/LICENSE-2.0
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

fn main() {
    // Add the bindgen crate as a build dependency
    let bindgen = bindgen::Builder::default()
        .header("path/to/header/file.h")
        .generate_comments(true)
        .generate_bindings();

    // Generate bindgen output into a separate file
    bindgen.write_to_file("bindings.rs");
}