use std::collections::HashMap;

use anyhow::Result;

use crate::config;
use crate::sim_params::SimParams;

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
}

impl JobParams {
    pub fn new(sim: &SimParams) -> Result<JobParams> {
        let num_cores = sim.get_default_i64("numCores", config::DEFAULT_NUM_CORES);
        let (num_nodes, num_cores_per_node, partition) =
            JobParams::get_core_distribution(num_cores, config::SYSTEM_CONFIG);
        Ok(JobParams {
            num_cores,
            num_nodes,
            num_cores_per_node,
            partition,
            wall_time: sim.get_default_string("wallTime", config::DEFAULT_WALL_TIME),
            job_name: sim.get_default_string("jobName", config::DEFAULT_JOB_NAME),
            log_file: sim.get_default_string("logFile", config::DEFAULT_LOG_FILE),
            run_params: sim.get_default_string("runParams", config::DEFAULT_RUN_PARAMS),
            param_file: sim.get_default_string("paramFile", config::DEFAULT_PARAM_FILE_NAME),
            executable_name: sim
                .get_default_string("executableName", config::DEFAULT_AREPO_EXECUTABLE_NAME),
        })
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
        let partition = match num_nodes > 1 {
            true => "multi".to_owned(),
            false => "single".to_owned(),
        };
        (num_nodes, num_cores_per_node, partition)
    }

    pub fn to_hashmap(&self) -> HashMap<String, String> {
        let mut res = HashMap::new();
        res.insert("numNodes".to_owned(), self.num_nodes.to_string());
        res.insert("numCores".to_owned(), self.num_cores.to_string());
        res.insert(
            "numCoresPerNode".to_owned(),
            self.num_cores_per_node.to_string(),
        );
        res.insert("partition".to_owned(), self.partition.to_string());
        res.insert("wallTime".to_owned(), self.wall_time.to_string());
        res.insert("jobName".to_owned(), self.job_name.to_string());
        res.insert("logFile".to_owned(), self.log_file.to_string());
        res.insert("runParams".to_owned(), self.run_params.to_string());
        res.insert("paramFile".to_owned(), self.param_file.to_string());
        res.insert(
            "executableName".to_owned(),
            self.executable_name.to_string(),
        );
        res
    }
}

pub struct SystemConfiguration {
    pub max_num_cores: i64, // Locally this is the real maximum, remotely this should be a sanity limit so we never launch a truly huge job ...
    pub max_num_cores_per_node: i64,
}
