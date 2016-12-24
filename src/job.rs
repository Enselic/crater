use std::path::{Path, PathBuf};
use JOB_DIR;
use std::fs;
use rand;
use file;
use docker::Container;
use errors::*;
use model::Cmd;
use bmk::Arguable;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct JobId(u64);

#[derive(Serialize, Deserialize)]
struct Job {
    id: JobId,
    cmd: Cmd,
    kind: JobKind,
    state: JobState,
}

#[derive(Serialize, Deserialize)]
enum JobKind {
    Docker(Option<Container>),
    Ec2,
}

#[derive(Serialize, Deserialize)]
enum JobState {
    Created,
    Running,
    Done
}

use std::fmt::{self, Display, Formatter};

impl Display for JobId {
    fn fmt(&self, f: &mut Formatter) -> ::std::result::Result<(), fmt::Error> {
        let s = format!("{:x}", self.0);
        s.fmt(f)
    }
}

fn job_path(job: JobId) -> PathBuf {
    Path::new(JOB_DIR).join(&format!("{}.json", job))
}

fn write_job(job: &Job) -> Result<()> {
    let ref job_path = job_path(job.id);
    fs::create_dir_all(&Path::new(JOB_DIR))?;

    file::write_json(job_path, job)
}

fn read_job(job: JobId) -> Result<Job> {
    let ref job_path = job_path(job);
    file::read_json(job_path)
}

pub fn create_local(cmd: Cmd) -> Result<()> {
    log!("create local job: {}", cmd.clone().to_args().join(" "));

    let ref job = Job {
        id: JobId(rand::random()),
        cmd: cmd,
        kind: JobKind::Docker(None),
        state: JobState::Created,
    };

    write_job(job)?;

    log!("job {} created in {}", job.id, job_path(job.id).display());

    Ok(())
}

pub fn run(job: JobId) -> Result<()> {
    let job = read_job(job)?;
    panic!("")
}

pub fn wait(job: JobId) -> Result<()> {
    panic!("")
}
