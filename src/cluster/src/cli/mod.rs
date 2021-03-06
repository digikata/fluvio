use std::sync::Arc;
use structopt::StructOpt;

mod group;
mod spu;
mod start;
mod delete;
mod util;
mod check;
mod releases;
mod error;

use start::StartOpt;
use delete::DeleteOpt;
use check::CheckOpt;
use releases::ReleasesCmd;
use group::SpuGroupCmd;
use spu::SpuCmd;

pub use self::error::ClusterCliError;

use fluvio_runner_local::RunCmd;
use fluvio_extension_common as common;
use common::target::ClusterTarget;
use common::output::Terminal;
use common::PrintTerminal;

/// Manage and view Fluvio clusters
#[derive(StructOpt, Debug)]
pub struct ClusterOpt {
    #[structopt(flatten)]
    target: ClusterTarget,

    #[structopt(subcommand)]
    cmd: ClusterCmd,
}

impl ClusterOpt {
    /// Execute a cluster command
    pub async fn process(self) -> Result<(), ClusterCliError> {
        let out = Arc::new(PrintTerminal {});
        self.cmd.process(out, self.target).await?;
        Ok(())
    }
}

/// Manage and view Fluvio clusters
#[derive(StructOpt, Debug)]
pub enum ClusterCmd {
    /// Start a Fluvio cluster, locally or on Minikube
    #[structopt(name = "start")]
    Start(Box<StartOpt>),

    /// Delete a Fluvio cluster from the local machine or Minikube
    #[structopt(name = "delete")]
    Delete(DeleteOpt),

    /// Check that all requirements for cluster startup are met
    #[structopt(name = "check")]
    Check(CheckOpt),

    /// Print information about various Fluvio releases
    #[structopt(name = "releases")]
    Releases(ReleasesCmd),

    /// Manage and view Streaming Processing Units (SPUs)
    ///
    /// SPUs make up the part of a Fluvio cluster which is in charge
    /// of receiving messages from producers, storing those messages,
    /// and relaying them to consumers. This command lets you see
    /// the status of SPUs in your cluster.
    #[structopt(name = "spu")]
    SPU(SpuCmd),

    /// Manage and view SPU Groups (SPGs)
    ///
    /// SPGs are groups of SPUs in a cluster which are managed together.
    #[structopt(name = "spg")]
    SPUGroup(SpuGroupCmd),

    /// Run a Streaming Controller (SC) or SPU
    #[structopt(name = "run")]
    Run(RunCmd),
}

impl ClusterCmd {
    /// process cluster commands
    pub async fn process<O: Terminal>(
        self,
        out: Arc<O>,
        target: ClusterTarget,
    ) -> Result<(), ClusterCliError> {
        match self {
            Self::Start(start) => {
                start.process().await?;
            }
            Self::Delete(uninstall) => {
                uninstall.process().await?;
            }
            Self::Check(check) => {
                check.process().await?;
            }
            Self::Releases(releases) => {
                releases.process().await?;
            }
            Self::SPU(spu) => {
                let fluvio = target.connect().await?;
                spu.process(out, &fluvio).await?;
            }
            Self::SPUGroup(group) => {
                let fluvio = target.connect().await?;
                group.process(out, &fluvio).await?;
            }
            Self::Run(run) => {
                run.process().await?;
            }
        }

        Ok(())
    }
}
