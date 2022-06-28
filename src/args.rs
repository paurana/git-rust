#![allow(non_camel_case_types)]

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    catFile(catFile),
    HashObject(HashObject),
    lsTree(lsTree),
    WriteTree,
    CommitTree(CommitTree),
}

#[derive(clap::Args)]
pub struct catFile {
    #[clap(short = 'p')]
    pub sha1: String,
}

#[derive(clap::Args)]
pub struct HashObject {
    #[clap(short = 'w')]
    pub path: PathBuf,
}

#[derive(clap::Args)]
pub struct lsTree {
    #[clap(long = "name-only")]
    pub sha1: String,
}

#[derive(clap::Args)]
pub struct CommitTree {
    pub tree_sha: String,
    #[clap(short = 'p')]
    pub commit_sha: String,
    #[clap(short = 'm')]
    pub message: String,
}
