fn run_sim_set<U: AsRef<Path>>(sim_set: SimSet, run_folder: U) -> Result<()> {
    let print_param = |sim: &SimParams, param: &str| println!("\t{}: {:?}", param, sim[param]);
    for (i, sim) in sim_set.iter().enumerate() {
        println!("{}:", i);
        if param_names.is_empty() {
            for param in sim.keys() {
                print_param(sim, param)
            }
        } else {
            for param in param_names.iter() {
                if !sim.contains_key(param) {
                    return Err(anyhow!("Parameter {} not present in parameter files!"));
                }
                print_param(sim, param)
            }
        }
    }
    Ok(())
}
