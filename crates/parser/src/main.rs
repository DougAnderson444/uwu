// https://www.christopherbiscardi.com/how-to-print-a-javascript-ast-using-swc-and-rust
use std::{path::Path, sync::Arc};
use swc::config::IsModule;
use swc_common::{
    errors::{ColorConfig, Handler},
    SourceMap,
};
use swc_ecma_ast::EsVersion;
use swc_ecma_parser::{EsConfig, Syntax};

fn main() {
    let cm = Arc::<SourceMap>::default();
    // let handler = Arc::new(
    let handler = Handler::with_tty_emitter(
        ColorConfig::Auto,
        true,
        false,
        Some(cm.clone()), // )
    );
    let c = swc::Compiler::new(cm.clone()); // , handler.clone()
    let fm = cm
        .load_file(Path::new("./foo.js"))
        .expect("failed to load file");

    let result = c.parse_js(
        fm,
        &handler,
        EsVersion::Es2020,
        Syntax::Es(EsConfig::default()),
        IsModule::Bool(true),
        false,
    );
    dbg!(result).unwrap();
}
