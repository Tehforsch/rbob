use super::plot_params::PlotParams;
use crate::array_utils::FArray2;

pub enum PostFnKind {
    Snap,
    Sim,
    Set,
    NoPlotSet,
}

pub struct PostResult {
    pub params: PlotParams,
    pub data: Vec<FArray2>,
}

impl PostResult {
    pub fn new(params: PlotParams, data: Vec<FArray2>) -> Self {
        Self { params, data }
    }

    pub fn join(results: Vec<PostResult>) -> PostResult {
        let mut final_result = PostResult {
            params: PlotParams::default(),
            data: vec![],
        };
        for (i, result) in results.into_iter().enumerate() {
            final_result.data.extend(result.data);
            for (k, v) in result.params.0.iter() {
                final_result
                    .params
                    .0
                    .insert(format!("{}_{}", k, i), v.into());
            }
        }
        final_result
    }
}

#[macro_export]
macro_rules! snap_function {
    ($i:ident, $code:block) => {
        pub fn run($i: &Self, sim_set: &SimSet, plot_template: Option<&str>) -> impl Iterator<Item=Result<DataPlotInfo>>  {
            use crate::postprocess::data_plot_info::DataPlotInfo;
            use crate::postprocess::snapshot::Snapshot;
            use crate::thread_pool::ThreadPool;
            use crate::config::MAX_NUM_POST_THREADS;
            use crate::postprocess::get_snapshot_files;
            let mut pool = ThreadPool::new(MAX_NUM_POST_THREADS);
            let mut infos = vec![];
            for sim in sim_set.iter() {
                for snap_path in get_snapshot_files(sim).unwrap() {
                    let snap = Snapshot::from_file(&sim, &snap_path).unwrap();
                    let info = $i.get_plot_info(Some(&sim), Some(&snap), plot_template).unwrap();
                    let sim = sim.clone();
                    let cloned = $i.clone();
                    infos.push(info);
                    pool.add_job(move || {
                        let snap=Snapshot::from_file(&sim, &snap_path).unwrap();
                        println!("Running on {}", snap_path.as_str());
                        let closure = $code;
                        closure(cloned, snap)
                    });
                }
            }
            infos.into_iter().zip(pool).map(|(info, result)|
                                            result.map(|result| {
                                                DataPlotInfo::new(info, result)}
                                            )
            )
        }
    }
}

#[macro_export]
macro_rules! no_plot_set_function {
    ($i:ident, $code:block) => {
        pub fn run($i: &Self, sim_set: &SimSet) -> impl Iterator<Item = Result<DataPlotInfo>> {
            let result = $code(sim_set);
            if result.is_err() {
                vec![Err(result.err().unwrap())]
            } else {
                vec![]
            }
            .into_iter()
        }
    };
}

#[macro_export]
macro_rules! set_function {
    ($i:ident, $code:block) => {
        pub fn run(
            $i: &Self,
            sim_set: &SimSet,
            plot_template: Option<&str>,
        ) -> impl Iterator<Item = Result<DataPlotInfo>> {
            let result = $code(sim_set);
            let info = $i.get_plot_info(None, None, plot_template).unwrap();
            vec![result.map(|result| DataPlotInfo::new(info, result))].into_iter()
        }
    };
}
