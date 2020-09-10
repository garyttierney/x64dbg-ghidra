extern crate bindgen;

use std::env;
use std::path::PathBuf;

struct BindingHeader {
    file: String,
    function_pattern: Option<String>,
    type_pattern: Option<String>,
    var_pattern: Option<String>,
}

impl BindingHeader {
    pub fn new<S: Into<String>>(file: S) -> Self {
        BindingHeader {
            file: file.into(),
            function_pattern: None,
            type_pattern: None,
            var_pattern: None,
        }
    }

    pub fn with_var_pattern<S: Into<String>>(mut self, pattern: S) -> Self {
        self.var_pattern = Some(pattern.into());
        self
    }

    pub fn with_type_pattern<S: Into<String>>(mut self, pattern: S) -> Self {
        self.type_pattern = Some(pattern.into());
        self
    }

    pub fn with_function_pattern<S: Into<String>>(mut self, pattern: S) -> Self {
        self.function_pattern = Some(pattern.into());
        self
    }
}

fn main() {
    let x64dbg_dir = env!("X64DBG_SDK_DIR");

    println!("cargo:rustc-link-lib=x64bridge");
    println!("cargo:rustc-link-lib=x64dbg");
    println!("cargo:rustc-link-lib=TitanEngine_x64");
    println!("cargo:rustc-link-search=native={}", x64dbg_dir);
    println!("cargo:rustc-link-search=native={}/TitanEngine", x64dbg_dir);

    let bindings = vec![
        BindingHeader::new("bridge.h").with_function_pattern("(Dbg|Bridge).*"),
        BindingHeader::new("plugin.h")
            .with_type_pattern("PLUG.*")
            .with_function_pattern("_plugin.*")
            .with_var_pattern("PLUG.*"),
    ];

    for config in &bindings {
        println!("cargo:rerun-if-changed=includes/{}", &config.file);

        let mut bindings = bindgen::Builder::default()
            .header(format!("includes/{}", &config.file))
            .clang_args(&["-x", "c++"])
            .clang_arg(format!("-I{}", x64dbg_dir));

        if let Some(fn_whitelist) = &config.function_pattern {
            bindings = bindings.whitelist_function(fn_whitelist.as_str());
        }

        if let Some(ty_whitelist) = &config.type_pattern {
            bindings = bindings.whitelist_type(ty_whitelist.as_str());
        }

        if let Some(var_whitelist) = &config.var_pattern {
            bindings = bindings.whitelist_var(var_whitelist.as_str());
        }

        let out_path = PathBuf::from("src/generated");

        bindings
            .whitelist_recursively(true)
            .generate()
            .expect("Unable to generate bindings")
            .write_to_file(out_path.join(config.file.replace(".h", ".rs")))
            .expect("Couldn't write bindings");
    }
}
