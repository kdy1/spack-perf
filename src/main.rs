use anyhow::{Context, Error};
use clap::{App, AppSettings, Arg};
use spack::{loaders::swc::SwcLoader, resolvers::NodeResolver};
use std::{fs, path::PathBuf, sync::Arc};
use swc::config::SourceMapsConfig;
use swc_bundler::{BundleKind, Bundler};
use swc_common::{
    errors::{ColorConfig, Handler},
    FileName, FilePathMapping, SourceMap,
};

fn main() -> Result<(), Error> {
    let cm = Arc::new(SourceMap::new(FilePathMapping::empty()));
    let handler = Arc::new(Handler::with_tty_emitter(
        ColorConfig::Always,
        true,
        false,
        Some(cm.clone()),
    ));
    let compiler = Arc::new(swc::Compiler::new(cm.clone(), handler));
    let loader = SwcLoader::new(
        compiler.clone(),
        swc::config::Options {
            swcrc: true,
            ..Default::default()
        },
    );
    let bundler = Bundler::new(
        compiler.globals(),
        cm.clone(),
        &loader,
        NodeResolver::new(),
        swc_bundler::Config {
            require: true,
            external_modules: vec![
                "assert",
                "buffer",
                "child_process",
                "console",
                "cluster",
                "crypto",
                "dgram",
                "dns",
                "events",
                "fs",
                "http",
                "http2",
                "https",
                "net",
                "os",
                "path",
                "perf_hooks",
                "process",
                "querystring",
                "readline",
                "repl",
                "stream",
                "string_decoder",
                "timers",
                "tls",
                "tty",
                "url",
                "util",
                "v8",
                "vm",
                "wasi",
                "worker",
                "zlib",
            ]
            .into_iter()
            .map(From::from)
            .collect(),
        },
    );

    let matches = App::new("spack")
        .arg(
            Arg::with_name("destination")
                .short("d")
                .takes_value(true)
                .required(true)
                .help("directory to place built files"),
        )
        .arg(
            Arg::with_name("entries")
                .value_name("ENTRIES")
                .help("entry files")
                .takes_value(true)
                .multiple(true)
                .required(true),
        )
        .global_setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    let entries = matches.values_of_lossy("entries").unwrap();
    let modules = bundler
        .bundle(
            entries
                .into_iter()
                .map(|path| (path.clone(), FileName::Real(PathBuf::from(path))))
                .collect(),
        )
        .context("failed to bundle")?;

    for bundled in modules {
        let code = compiler
            .print(&bundled.module, SourceMapsConfig::Bool(false), None, false)
            .expect("failed to print?")
            .code;

        let name = match bundled.kind {
            BundleKind::Named { name } | BundleKind::Lib { name } => PathBuf::from(name),
            BundleKind::Dynamic => format!("dynamic.{}.js", bundled.id).into(),
        };

        let output_path = PathBuf::from(matches.value_of_lossy("destination").unwrap().to_string())
            .join(name.file_name().unwrap());

        fs::write(&output_path, code).unwrap();
    }

    Ok(())
}
