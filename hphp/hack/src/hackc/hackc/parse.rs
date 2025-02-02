// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.

use aast_parser::{rust_aast_parser_types::Env as AastParserEnv, AastParser};
use anyhow::{Context, Result};
use clap::Parser;
use ocamlrep::rc::RcOc;
use oxidized::relative_path::{self, RelativePath};
use parser_core_types::{
    indexed_source_text::IndexedSourceText, parser_env::ParserEnv, source_text::SourceText,
};
use rayon::prelude::*;
use std::{
    fs,
    io::{stdin, BufRead, BufReader},
    path::PathBuf,
    sync::Arc,
};
use strum_macros::{Display, EnumString};

#[derive(Parser, Clone, Debug)]
pub struct Opts {
    #[clap(default_value_t, long)]
    parser: ParserKind,
}

#[derive(Clone, Copy, Debug, Display, EnumString)]
enum ParserKind {
    Aast,
    Positioned,
    PositionedByRef,
    PositionedWithFullTrivia,
    DirectDecl,
}

impl Default for ParserKind {
    fn default() -> Self {
        Self::Positioned
    }
}

pub fn run(opts: Opts) -> Result<()> {
    let files = BufReader::new(stdin())
        .lines()
        .collect::<std::io::Result<Vec<_>>>()
        .context("could not read line from input file list")?
        .into_iter()
        .map(|l| PathBuf::from(l.trim()))
        .collect::<Vec<_>>();

    files
        .into_par_iter()
        .try_for_each(|f| parse_file(opts.parser, f.clone()))
}

fn parse_file(parser: ParserKind, filepath: PathBuf) -> anyhow::Result<()> {
    let content = fs::read(&filepath)?;
    let ctx = &Arc::new((filepath, content));
    let new_ctx = Arc::clone(ctx);
    let env = ParserEnv::default();
    let (filepath, content) = new_ctx.as_ref();
    let path = RelativePath::make(relative_path::Prefix::Dummy, filepath.clone());
    let source_text = SourceText::make(RcOc::new(path.clone()), content.as_slice());
    match parser {
        ParserKind::PositionedWithFullTrivia => {
            let stdout = std::io::stdout();
            let w = stdout.lock();
            let mut s = serde_json::Serializer::new(w);
            let arena = bumpalo::Bump::new();
            let src = IndexedSourceText::new(source_text);
            positioned_full_trivia_parser::parse_script_to_json(&arena, &mut s, &src, env)?
        }
        ParserKind::PositionedByRef => {
            let arena = bumpalo::Bump::new();
            let (_, _, _) = positioned_by_ref_parser::parse_script(&arena, &source_text, env);
        }
        ParserKind::Positioned => {
            let (_, _, _) = positioned_parser::parse_script(&source_text, env);
        }
        ParserKind::DirectDecl => {
            let arena = bumpalo::Bump::new();
            let _ = direct_decl_parser::parse_decls(Default::default(), path, content, &arena);
        }
        ParserKind::Aast => {
            let indexed_source_text = IndexedSourceText::new(source_text);
            let env = AastParserEnv::default();
            let _ = AastParser::from_text(&env, &indexed_source_text);
        }
    }
    Ok(())
}
