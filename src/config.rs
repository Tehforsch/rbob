use camino::Utf8PathBuf;
use lazy_static::lazy_static;

use crate::config_file::ConfigFile;
use crate::job_params::SystemConfiguration;

lazy_static! {
    pub static ref CONFIG_FILE: ConfigFile = ConfigFile::read().unwrap().expanduser().unwrap();
    pub static ref RAXIOM_PATH: Utf8PathBuf = CONFIG_FILE.raxiom_path.clone();
    pub static ref RAXIOM_BUILD_PATH: Utf8PathBuf = CONFIG_FILE.raxiom_build_path.clone();
    pub static ref DEFAULT_SYSTYPE: String = CONFIG_FILE.default_systype.clone();
    pub static ref JOB_FILE_TEMPLATE: String = CONFIG_FILE.job_file_template.clone();
    pub static ref JOB_FILE_RUN_COMMAND: String = CONFIG_FILE.job_file_run_command.clone();
    pub static ref SYSTEM_CONFIG: SystemConfiguration = CONFIG_FILE.system_config.clone();
}

pub static DEFAULT_BOB_CONFIG_NAME: &str = "sims.bob";
pub static DEFAULT_PARAM_FILE_NAME: &str = "params.yml";
pub static DEFAULT_JOB_FILE_NAME: &str = "job";
pub static DEFAULT_RUN_PARAMS: &str = "--headless true -v";

pub static DEFAULT_RAXIOM_EXECUTABLE_NAME: &str = "arepo_postprocess";

pub static CONFIG_FILE_NAME: &str = "config.yaml";

pub static DEFAULT_RUN_PROGRAM: &str = "mpirun";

pub static DEFAULT_LOG_FILE: &str = "stdout.log";
pub static DEFAULT_JOB_NAME: &str = "raxiom";
pub static DEFAULT_WALL_TIME: &str = "23:00:00";
pub static DEFAULT_NUM_CORES: &i64 = &1;

pub static SPECIAL_PARAMS: &[&str] = &[
    "numCores",
    "runParams",
    "additionalCommands",
    "wallTime",
    "simType",
    "simLabel",
];
