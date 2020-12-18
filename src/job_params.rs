use anyhow::Result;
use std::collections::HashMap;

use crate::config;

use crate::sim_params::SimParams;

pub struct JobParams {
    num_nodes: i64,
    num_cores: i64,
    num_cores_per_node: i64,
    partition: String,
    wall_time: String,
    job_name: String,
    log_file: String,
    run_command: String,
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
            run_command: sim.get_default_string("runCommand", config::DEFAULT_RUN_COMMAND),
        })
    }

    fn get_core_distribution(
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
        res.insert("runCommand".to_owned(), self.run_command.to_string());
        res
    }
}

pub struct SystemConfiguration {
    pub max_num_cores: i64, // Locally this is the real maximum, remotely this should be a sanity limit so we never launch a truly huge job ...
    pub max_num_cores_per_node: i64,
}
