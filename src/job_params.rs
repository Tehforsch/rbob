use serde::Deserialize;
use serde::Serialize;

use crate::config;

#[derive(Deserialize, Serialize, Debug)]
pub struct JobParams {
    pub num_nodes: i64,
    pub num_cores: i64,
    pub num_cores_per_node: i64,
    pub partition: String,
    pub wall_time: String,
    pub job_name: String,
    pub log_file: String,
    pub run_params: String,
    pub executable_name: String,
    pub param_file: String,
    pub run_program: String,
}
impl JobParams {
    pub fn set_core_nums(&mut self) {
        let num_cores = self.num_cores;
        (self.num_nodes, self.num_cores_per_node, self.partition) =
            get_core_distribution(num_cores, &config::SYSTEM_CONFIG);
    }
}

pub fn get_core_distribution(
    num_cores: i64,
    system_conf: &SystemConfiguration,
) -> (i64, i64, String) {
    if num_cores > system_conf.max_num_cores {
        panic!(
            "Number of cores ({}) exceeds maximum amount for this system ({})",
            num_cores, system_conf.max_num_cores
        );
    }
    let num_cores_per_node = num_cores.min(system_conf.max_num_cores_per_node);
    let num_nodes = num_cores / num_cores_per_node;
    let partition = system_conf.get_partition(num_nodes).into();
    (num_nodes, num_cores_per_node, partition)
}

impl Default for JobParams {
    fn default() -> Self {
        let num_cores = *config::DEFAULT_NUM_CORES;
        let (num_nodes, num_cores_per_node, partition) =
            get_core_distribution(num_cores, &config::SYSTEM_CONFIG);
        Self {
            num_cores,
            num_nodes,
            num_cores_per_node,
            partition,
            wall_time: config::DEFAULT_WALL_TIME.into(),
            job_name: config::DEFAULT_JOB_NAME.into(),
            log_file: config::DEFAULT_LOG_FILE.into(),
            run_params: config::DEFAULT_RUN_PARAMS.into(),
            param_file: config::DEFAULT_PARAM_FILE_NAME.into(),
            run_program: config::DEFAULT_RUN_PROGRAM.into(),
            executable_name: config::DEFAULT_SUBSWEEP_EXECUTABLE_NAME.into(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SystemConfiguration {
    pub max_num_cores: i64, // Locally this is the real maximum, remotely this should be a sanity limit so we never launch a truly huge job ...
    pub max_num_cores_per_node: i64,
    pub partitions: Vec<(i64, String)>,
}

impl SystemConfiguration {
    pub fn dependencies_allowed(&self) -> bool {
        // Temporary solution
        if self.max_num_cores > 6 {
            true
        } else {
            false
        }
    }

    pub fn get_partition(&self, num_nodes: i64) -> &str {
        let mut partition = &self.partitions[0].1;
        for (num_nodes_this_partition, this_partition) in self.partitions.iter() {
            if num_nodes >= *num_nodes_this_partition {
                partition = this_partition
            }
        }
        partition
    }
}
