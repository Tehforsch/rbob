use camino::Utf8PathBuf;
use lazy_static::lazy_static;

use crate::config_file::ConfigFile;
use crate::job_params::SystemConfiguration;

lazy_static! {
    pub static ref CONFIG_FILE: ConfigFile = ConfigFile::read().unwrap().expanduser().unwrap();
    pub static ref SUBSWEEP_PATH: Utf8PathBuf = CONFIG_FILE.subsweep_path.clone();
    pub static ref SUBSWEEP_BUILD_PATH: Utf8PathBuf = CONFIG_FILE.subsweep_build_path.clone();
    pub static ref JOB_FILE_TEMPLATE: String = CONFIG_FILE.job_file_template.clone();
    pub static ref JOB_FILE_RUN_COMMAND: String = CONFIG_FILE.job_file_run_command.clone();
    pub static ref DEFAULT_FEATURES: Vec<String> = CONFIG_FILE.default_features.clone();
    pub static ref SYSTEM_CONFIG: SystemConfiguration = CONFIG_FILE.system_config.clone();
}

pub static DEFAULT_BOB_CONFIG_NAME: &str = "sims.yml";
pub static DEFAULT_PARAM_FILE_NAME: &str = "params.yml";
pub static DEFAULT_JOB_FILE_NAME: &str = "job";
pub static DEFAULT_RUN_PARAMS: &str = "-v";

pub static DEFAULT_SUBSWEEP_EXECUTABLE_NAME: &str = "subsweep";

pub static CONFIG_FILE_NAME: &str = "config.yaml";

pub static DEFAULT_RUN_PROGRAM: &str = "mpirun";

pub static DEFAULT_LOG_FILE: &str = "stdout.log";
pub static DEFAULT_JOB_NAME: &str = "subsweep";
pub static DEFAULT_WALL_TIME: &str = "23:00:00";
pub static DEFAULT_NUM_CORES: &i64 = &1;

pub static SPECIAL_PARAMS: &[&str] = &[
    "num_cores",
    "run_params",
    "additional_commands",
    "wall_time",
    "sim_type",
    "sim_label",
];
