use camino::Utf8PathBuf;
use lazy_static::lazy_static;

use crate::config_file::ConfigFile;
use crate::job_params::SystemConfiguration;

lazy_static! {
    pub static ref CONFIG_FILE: ConfigFile = ConfigFile::read().unwrap().expanduser().unwrap();
    pub static ref AREPO_PATH: Utf8PathBuf = CONFIG_FILE.arepo_path.clone();
    pub static ref PLOT_TEMPLATE_FOLDER: Utf8PathBuf = CONFIG_FILE.plot_template_folder.clone();
    pub static ref DEFAULT_SYSTYPE: String = CONFIG_FILE.default_systype.clone();
    pub static ref JOB_FILE_TEMPLATE: String = CONFIG_FILE.job_file_template.clone();
    pub static ref JOB_FILE_RUN_COMMAND: String = CONFIG_FILE.job_file_run_command.clone();
    pub static ref SYSTEM_CONFIG: SystemConfiguration = CONFIG_FILE.system_config.clone();
}

pub static DEFAULT_BOB_CONFIG_NAME: &str = "sims.bob";
pub static DEFAULT_PARAM_FILE_NAME: &str = "param.txt";
pub static DEFAULT_CONFIG_FILE_NAME: &str = "Config.sh";
pub static DEFAULT_JOB_FILE_NAME: &str = "job";
pub static DEFAULT_GRID_JOB_FILE_NAME: &str = "gridJob";
pub static DEFAULT_GRID_FILE_NAME: &str = "grid.dat";
pub static DEFAULT_BOB_PARAM_FILE_NAME: &str = "bobParams.yaml";

pub static DEFAULT_AREPO_EXECUTABLE_NAME: &str = "./Arepo";
pub static DEFAULT_AREPO_SOURCE_FOLDER: &str = "src";
pub static DEFAULT_AREPO_CONFIG_BUILD_FILE: &str = "build/arepoconfig.h";
pub static DEFAULT_AREPO_CONFIG_SOURCE_FILE: &str = "src/arepoconfig.h";

pub static DEFAULT_PIC_FOLDER: &str = "pics";

pub static CONFIG_FILE_NAME: &str = "config.yaml";

pub static MAX_NUM_VORONOI_SWIM_THREADS: usize = 8;
pub static MAX_NUM_POST_THREADS: usize = 1;

pub static DEFAULT_RUN_PROGRAM: &str = "mpirun";

pub static DEFAULT_LOG_FILE: &str = "stdout.log";
pub static DEFAULT_SIMPLEX_LOG_FILE: &str = "simplex.txt";
pub static DEFAULT_JOB_NAME: &str = "arepoTest";
pub static DEFAULT_WALL_TIME: &str = "23:00:00";
pub static DEFAULT_NUM_CORES: &i64 = &1;
pub static DEFAULT_RUN_PARAMS: &str = "0";
pub static DEFAULT_NUM_CORES_TO_COMPILE: &i64 = &12;

pub static NX_SLICE: usize = 600;
pub static NY_SLICE: usize = 600;

pub static DEFAULT_PLOT_FILE_NAME: &str = "plot.gp";
pub static DEFAULT_PLOT_EXTENSION: &str = "gp";
pub static DEFAULT_PLOT_CONFIG_FOLDER_NAME: &str = "config";
pub static DEFAULT_PLOT_INFO_FILE_NAME: &str = "plot.info";

pub static CASCADE_IDENTIFIER: &str = "cascade";

pub static SPECIAL_PARAMS: &[&str] = &[
    "numCores",
    "runParams",
    "executableName",
    "paramFile",
    "arepoCommit",
    "runProgram",
    "additionalCommands",
    "cascade",
    "wallTime",
    "simType",
    "simLabel",
];

pub static CALC_PARAMS: &[&str] = &["timeUnit"];

pub static H_IONIZATION_RATE_INDEX: usize = 2;
pub static SWEEP_NFREQ: usize = 6;

pub static CONFIG_FILE_PARAMS: &[&str] = &[
    "NTYPES",
    "PERIODIC",
    "TWODIMS",
    "AXISYMMETRY",
    "ONEDIMS",
    "LONG_X",
    "LONG_Y",
    "LONG_Z",
    "REFLECTIVE_X",
    "REFLECTIVE_Y",
    "REFLECTIVE_Z",
    "MHD",
    "MHD_DONT_PRINT_BMAGSUM",
    "MHD_CT",
    "MHD_CT_IC",
    "MHD_CT_PERTURB_POSITIONS",
    "MHD_DIVBCLEANING",
    "MHD_POWELL",
    "MHD_POWELL_LIMIT_TIMESTEP",
    "MHD_SEEDFIELD",
    "MHD_SEEDPSPEC",
    "MHD_THERMAL_ENERGY_SWITCH",
    "COOLING",
    "UVB_SELF_SHIELDING",
    "USE_SFR",
    "QUICK_LYALPHA",
    "QUICK_LYALPHA_LATETIMEONLY",
    "SFR_KEEP_CELLS",
    "GAMMA",
    "ISOTHERM_EQS",
    "USE_ENTROPY_FOR_COLD_FLOWS",
    "ENTROPY_MACH_THRESHOLD",
    "PREHEATING",
    "NOHYDRO",
    "SPECIAL_RELATIVITY",
    "SPECIAL_RELATIVITY_HLLC",
    "SR_HLLC_ZERO_COMPVEL",
    "GENERAL_RELATIVITY",
    "METRIC_TYPE",
    "ATMOSPHERE_GENERAL_RELATIVITY",
    "ADIABATIC_GENERAL_RELATIVITY",
    "NON_IDEAL_MHD",
    "OHMIC_DIFFUSION",
    "IMPLICIT_OHMIC_DIFFUSION",
    "ONLY_OHMIC_DIFFUSION",
    "DIFFUSION",
    "COSMIC_RAYS",
    "COSMIC_RAYS_COOLING",
    "COSMIC_RAYS_ALFVEN_COOLING",
    "COSMIC_RAYS_STREAMING",
    "COSMIC_RAYS_STREAMING_ANISOTROPIC",
    "COSMIC_RAYS_DIFFUSION",
    "COSMIC_RAYS_DIFFUSION_CONSTANT_TIMESTEP",
    "COSMIC_RAYS_DIFFUSION_GLOBAL_TIMESTEP",
    "COSMIC_RAYS_DIFFUSION_EXPLICIT",
    "COSMIC_RAYS_DIFFUSION_FULL_NORMAL_GRADIENT",
    "COSMIC_RAYS_DIFFUSION_ALWAYS_USE_PRECONDITIONER",
    "COSMIC_RAYS_DIFFUSION_ANISOTROPIC",
    "COSMIC_RAYS_DIFFUSION_LIMITER",
    "COSMIC_RAYS_DIFFUSION_OLD",
    "COSMIC_RAYS_SN_INJECTION",
    "COSMIC_RAYS_SHOCK_ACCELERATION",
    "COSMIC_RAYS_IN_ICS",
    "COSMIC_RAYS_MAGNETIC_OBLIQUITY",
    "OUTPUT_CR_PRESSURE_GRADIENT",
    "NUM_THREADS",
    "IMPOSE_PINNING",
    "IMPOSE_PINNING_OVERRIDE_MODE",
    "GENERIC_ASYNC",
    "AMR",
    "VORONOI",
    "VARIABLE_GAMMA",
    "RIEMANN_HLL",
    "RIEMANN_HLLC",
    "RIEMANN_ROSUNOV",
    "RIEMANN_HLLD",
    "RIEMANN_GAMMA",
    "AMR_CONNECTIONS",
    "AMR_GRADIENTS",
    "AMR_REDUCE_DOMAIN_DECOMPOISTION",
    "TVD_SLOPE_LIMITER",
    "TVD_SLOPE_LIMITER_VANLEER",
    "TVD_SLOPE_LIMITER_SUPERBEE",
    "TVD_SLOPE_LIMITER_ALBADA",
    "TVD_SLOPE_LIMITER_MINBEE",
    "GRADIENT_LIMITER_DUFFELL",
    "DISABLE_TIME_EXTRAPOLATION",
    "DISABLE_SPATIAL_EXTRAPOLATION",
    "NO_SCALAR_GRADIENTS",
    "GRADIENTS_GREEN_GAUSS",
    "VORONOI_STATIC_MESH",
    "VORONOI_STATIC_MESH_DO_DOMAIN_DECOMPOSITION",
    "REGULARIZE_MESH_CM_DRIFT",
    "REGULARIZE_MESH_CM_DRIFT_USE_SOUNDSPEED",
    "REGULARIZE_MESH_FACE_ANGLE",
    "REGULARIZE_MESH_OPTIMAL",
    "OUTPUT_MESH_FACE_ANGLE",
    "CALCULATE_VERTEX_VELOCITY_DIVERGENCE",
    "STICKY_POINTS_ON_REFLECTIVE_SURFACE",
    "FORCE_EQUAL_TIMESTEPS",
    "TREE_BASED_TIMESTEPS",
    "DECOUPLE_TIMESTEPS",
    "MUSCL_HANCOCK",
    "RUNGE_KUTTA_FULL_UPDATE",
    "DVR_RENDER",
    "DVR_RENDER_SMOOTH",
    "DVR_RENDER_ORTHOGONAL",
    "DVR_NUM_FIELDS",
    "DVR_STAY_IN_BOX",
    "VORONOI_MESHOUTPUT",
    "VORONOI_IMAGES_FOREACHSNAPSHOT",
    "VORONOI_FREQUENT_IMAGES",
    "VORONOI_FIELD_DUMP_PIXELS_X",
    "VORONOI_FIELD_DUMP_PIXELS_Y",
    "VORONOI_VELOCITY_FIELD_2D",
    "VORONOI_FIELD_COMPENSATE_VX",
    "VORONOI_FIELD_COMPENSATE_VY",
    "VORONOI_NEW_IMAGE",
    "VORONOI_PROJ_TEMP",
    "VORONOI_PROJ",
    "VORONOI_MULTIPLE_PROJECTIONS",
    "VORONOI_NOGRADS",
    "REFINEMENT_SPLIT_CELLS",
    "REFINEMENT_MERGE_CELLS",
    "REFINEMENT_MERGE_PAIRS",
    "REFINEMENT_VOLUME_LIMIT",
    "REFINEMENT_HIGH_RES_GAS",
    "REFINEMENT_AROUND_BH",
    "DEREFINE_ONLY_DENSE_GAS",
    "NODEREFINE_BACKGROUND_GRID",
    "DEREFINE_GENTLY",
    "OPTIMIZE_MESH_MEMORY_FOR_REFINEMENT",
    "REFINEMENT_AROUND_DM",
    "MESHRELAX",
    "MESHRELAX_DENSITY_IN_INPUT",
    "ADDBACKGROUNDGRID",
    "AMR_REMAP",
    "SELFGRAVITY",
    "VALPOTENTIAL",
    "HIERARCHICAL_GRAVITY",
    "CELL_CENTER_GRAVITY",
    "NO_GAS_SELFGRAVITY",
    "GRAVITY_NOT_PERIODIC",
    "GRAVITY_TALLBOX",
    "NO_SELFGRAVITY_TYPE",
    "ALLOW_DIRECT_SUMMATION",
    "DIRECT_SUMMATION_THRESHOLD",
    "EXACT_GRAVITY_FOR_PARTICLE_TYPE",
    "EXTERNALGRAVITY",
    "EXTERNALGY",
    "EXTERNALDISKPOTENTIAL",
    "EXTERNALSHEARBOX",
    "EXTERNALSHEARBOX_KSRATE_RANDOM",
    "EXTERNALSHEARBOX_KSRATE_UPDATE_PARAM",
    "ENFORCE_JEANS_STABILITY_OF_CELLS_EEOS",
    "ENFORCE_JEANS_STABILITY_OF_CELLS",
    "EVALPOTENTIAL",
    "EXTERNALSHEETY",
    "COMPUTE_POTENTIAL_ENERGY",
    "RANDOMIZE_DOMAINCENTER",
    "DO_NOT_RANDOMIZE_DOMAINCENTER",
    "NSOFTTYPES",
    "MULTIPLE_NODE_SOFTENING",
    "INDIVIDUAL_GRAVITY_SOFTENING",
    "ADAPTIVE_HYDRO_SOFTENING",
    "NSOFTTYPES_HYDRO",
    "PMGRID",
    "ASMTH",
    "RCUT",
    "PLACEHIGHRESREGION",
    "ENLARGEREGION",
    "GRIDBOOST",
    "ONLY_PM",
    "FFT_COLUMN_BASED",
    "PM_ZOOM_OPTIMIZED",
    "AUTO_SWAP_ENDIAN_READIC",
    "CHUNKING",
    "DOUBLEPRECISION",
    "DOUBLEPRECISION_FFTW",
    "OUTPUT_IN_DOUBLEPRECISION",
    "INPUT_IN_DOUBLEPRECISION",
    "OUTPUT_COORDINATES_IN_DOUBLEPRECISION",
    "NGB_TREE_DOUBLEPRECISION",
    "FOF",
    "FOF_PRIMARY_LINK_TYPES",
    "FOF_SECONDARY_LINK_TYPES",
    "FOF_SECONDARY_LINK_TARGET_TYPES",
    "FOF_GROUP_MIN_LEN",
    "FOF_LINKLENGTH",
    "FOF_FUZZ_SORT_BY_NEAREST_GROUP",
    "FOF_STOREIDS",
    "USE_AREPO_FOF_WITH_GADGET_FIX",
    "ADD_GROUP_PROPERTIES",
    "SUBFIND",
    "SAVE_HSML_IN_SNAPSHOT",
    "SUBFIND_MEASURE_H2MASS",
    "SUBFIND_CALC_MORE",
    "SUBFIND_EXTENDED_PROPERTIES",
    "METALS",
    "MIN_METALLICITY_ON_STARTUP",
    "STELLARAGE",
    "SOFTEREQS",
    "MODIFIED_EOS",
    "SLOW_RELAX_TO_EOS",
    "BLACK_HOLES",
    "BH_THERMALFEEDBACK",
    "BH_THERMALFEEDBACK_ACC",
    "BH_NF_RADIO",
    "DRAINGAS",
    "BH_EXACT_INTEGRATION",
    "BH_BONDI_DEFAULT",
    "BH_BONDI_DENSITY",
    "BH_BONDI_CAPTURE",
    "BH_BONDI_DISK_VORTICITY",
    "BH_DO_NOT_PREVENT_MERGERS",
    "BH_USE_GASVEL_IN_BONDI",
    "BH_USE_ALFVEN_SPEED_IN_BONDI",
    "MASSIVE_SEEDS",
    "MASSIVE_SEEDS_MERGER",
    "BH_NEW_CENTERING",
    "BH_FRICTION",
    "BH_FRICTION_AGGRESSIVE",
    "BH_HARMONIC_OSCILLATOR_FORCE",
    "BH_DRAG",
    "REPOSITION_ON_POTMIN",
    "BH_PRESSURE_CRITERION",
    "BH_RELATIVE_NGB_DEVIATION",
    "OUTPUT_BLACK_HOLE_TIMESTEP",
    "UNIFIED_FEEDBACK",
    "BH_BUBBLES",
    "BH_MAGNETIC_BUBBLES",
    "BH_MAGNETIC_DIPOLAR_BUBBLES",
    "BH_ADIOS_WIND",
    "BH_ADIOS_DENS_DEP_EFFICIANCY",
    "BH_ADIOS_WIND_WITH_QUASARTHRESHOLD",
    "BH_ADIOS_WIND_WITH_VARIABLE_QUASARTHRESHOLD",
    "BH_ADIOS_WIND_DIRECTIONAL",
    "BH_ADIOS_RANDOMIZED",
    "BH_ADIOS_ONLY_ABOVE_MINIMUM_DENSITY",
    "BH_CONTINOUS_MODE_SWITCH",
    "REFINEMENT_AROUND_BH_FIXED",
    "SUPPRESS_SF_IN_REFINEMENT_REGION",
    "BH_BIPOLAR_FEEDBACK",
    "TRACER_FIELD",
    "TRACER_PARTICLE",
    "TRACER_MC",
    "TRACER_MC_CHECKS",
    "GENERATE_TRACER_PARTICLE_IN_ICS",
    "GENERATE_TRACER_MC_IN_ICS",
    "TRACER_PART_NUM_FLUID_QUANTITIES",
    "TRACER_PART_STORE_WHAT",
    "TRACER_MC_NUM_FLUID_QUANTITIES",
    "TRACER_MC_STORE_WHAT",
    "TRACER_NO_RESET_EACH_SNAP",
    "TRACER_MC_SKIPLOAD",
    "FOF_DISABLE_SNAP_REWRITE",
    "TRACER_TRAJECTORY",
    "TRACER_TRAJECTORY_GENERATE",
    "READ_DM_AS_GAS",
    "NO_ID_UNIQUE_CHECK",
    "RUNNING_SAFETY_FILE",
    "LOAD_TYPES",
    "READ_COORDINATES_IN_DOUBLE",
    "IDS_OFFSET",
    "TILE_ICS",
    "COMBINETYPES",
    "MULTIPLE_RESTARTS",
    "TOLERATE_WRITE_ERROR",
    "OPTIMIZE_MEMORY_USAGE",
    "SUBBOX_SNAPSHOTS",
    "PROCESS_TIMES_OF_OUTPUTLIST",
    "EXTENDED_GHOST_SEARCH",
    "DOUBLE_STENCIL",
    "TETRA_INDEX_IN_FACE",
    "VORONOI_DYNAMIC_UPDATE",
    "COFFEE_PROBLEM",
    "NOH_PROBLEM",
    "SHIFT_BY_HALF_BOX",
    "DISABLE_VELOCITY_CSND_SLOPE_LIMITING",
    "NO_MPI_IN_PLACE",
    "NO_ISEND_IRECV_IN_DOMAIN",
    "FIX_PATHSCALE_MPI_STATUS_IGNORE_BUG",
    "USE_MPIALLTOALLV_IN_DOMAINDECOMP",
    "MPI_HYPERCUBE_ALLGATHERV",
    "MPISENDRECV_CHECKSUM",
    "NOTREERND",
    "ENLARGE_DYNAMIC_RANGE_IN_TIME",
    "NOSTOP_WHEN_BELOW_MINTIMESTEP",
    "DO_NOT_CREATE_STAR_PARTICLES",
    "DMPIC",
    "ALLOWEXTRAPARAMS",
    "RADIATIVE_RATES",
    "FIX_SPH_PARTICLES_AT_IDENTICAL_COORDINATES",
    "VEL_POWERSPEC",
    "ADJ_BOX_POWERSPEC",
    "DISABLE_OPTIMIZE_DOMAIN_MAPPING",
    "RECOMPUTE_POTENTIAL_IN_SNAPSHOT",
    "CUDA",
    "CUDA_INSTRUMENT",
    "USE_DSDE",
    "USE_NBC_FOR_IBARRIER",
    "HUGEPAGES",
    "DETAILEDTIMINGS",
    "PERFORMANCE_TEST_SPARSE_MPI_ALLTOALL",
    "TIMESTEP_OUTPUT_LIMIT",
    "UPDATE_GRADIENTS_FOR_OUTPUT",
    "REDUCE_FLUSH",
    "OUTPUT_REFBHCOUNTER",
    "OUTPUT_EVERY_STEP",
    "GODUNOV_STATS",
    "OUTPUT_CPU_CSV",
    "OUTPUT_TASK",
    "OUTPUT_TIMEBIN_HYDRO",
    "OUTPUT_PRESSURE_GRADIENT",
    "OUTPUT_DENSITY_GRADIENT",
    "OUTPUT_VELOCITY_GRADIENT",
    "OUTPUT_BFIELD_GRADIENT",
    "OUTPUT_VERTEX_VELOCITY",
    "OUTPUT_VERTEX_VELOCITY_DIVERGENCE",
    "OUTPUT_VOLUME",
    "OUTPUT_CENTER_OF_MASS",
    "OUTPUT_SURFACE_AREA",
    "OUTPUT_PRESSURE",
    "OUTPUTPOTENTIAL",
    "OUTPUTACCELERATION",
    "OUTPUTTIMESTEP",
    "OUTPUT_SOFTENINGS",
    "OUTPUTGRAVINTERACTIONS",
    "HAVE_HDF5",
    "HDF5_FILTERS",
    "OUTPUT_XDMF",
    "OUTPUTCOOLRATE",
    "OUTPUT_DIVVEL",
    "OUTPUT_CURLVEL",
    "OUTPUT_COOLHEAT",
    "OUTPUT_VORTICITY",
    "OUTPUT_CELL_SPIN",
    "MEASURE_DISSIPATION_RATE",
    "OUTPUT_MACHNUM",
    "DEBUG",
    "DEBUG_ENABLE_FPU_EXCEPTIONS",
    "CHECKSUM_DEBUG",
    "RESTART_DEBUG",
    "VERBOSE",
    "HOST_MEMORY_REPORTING",
    "VTUNE_INSTRUMENT",
    "FORCETEST",
    "FORCETEST_TESTFORCELAW",
    "DISK_POTENTIAL",
    "DISK_MASS_M0",
    "DISK_SCALE_R0",
    "STATICNFW",
    "NFW_C",
    "NFW_M200",
    "NFW_E",
    "NFW_DARKFRACTION",
    "STATICISO",
    "ISO_M200",
    "ISO_R200",
    "ISO_E",
    "ISO_FRACTION",
    "STATICHQ",
    "HQ_M200",
    "HQ_C",
    "HQ_DARKFRACTION",
    "GROWING_DISK_POTENTIAL",
    "DARKENERGY",
    "TIMEDEPDE",
    "RESCALEVINI",
    "EXTERNALHUBBLE",
    "TIMEDEPGRAV",
    "DARKENERGY_DEBUG",
    "SECOND_ORDER_ICS",
    "LONGIDS",
    "OFFSET_FOR_NON_CONTIGUOUS_IDS",
    "GENERATE_GAS_IN_ICS",
    "SPLIT_PARTICLE_TYPE",
    "NTYPES_ICS",
    "VS_TURB",
    "POWERSPEC_GRID",
    "AB_TURB",
    "READ_LEGACY_ICS",
    "EOS_DEGENERATE",
    "EOS_COULOMB_CORRECTIONS",
    "EOS_NSPECIES",
    "RELAXOBJECT",
    "RELAXOBJECT_COOLING",
    "RELAXOBJECT_BINARY",
    "PASSIVE_SCALARS",
    "NUCLEAR_NETWORK",
    "NETWORK_NSE",
    "NETWORK_PARDISO",
    "NETWORK_SCREENING",
    "REACLIB1",
    "NUCLEAR_NETWORK_DETONATE",
    "EOS_OPAL",
    "RT_ENABLE",
    "RT_COOLING_PHOTOHEATING",
    "RT_ADVECT",
    "RT_CGMETHOD",
    "RT_SLOWLIGHT",
    "RT_N_DIR",
    "RT_COMBINE_N_DIR_IN_OUTPUT",
    "RT_ALLOW_ABSORBING_CELLS",
    "RT_SPREAD_SOURCE",
    "RT_STELLAR_SOURCES",
    "RT_HEALPIX_NSIDE",
    "RT_INCLUDE_HE",
    "SOURCE_PERIODIC",
    "DO_NOT_MOVE_GAS",
    "HYDROGEN_ONLY",
    "SECOND_DERIVATIVES",
    "SLOPE_LIMIT_HESSIANS",
    "RECONSTRUCT_GRADIENTS",
    "GLOBAL_VISCOSITY",
    "USE_KINEMATIC_VISCOSITY",
    "ALPHA_VISCOSITY",
    "LOCAL_VISCOSITY",
    "THERMAL_CONDUCTION",
    "TRACER_DIFFUSION",
    "CIRCUMSTELLAR",
    "CIRCUMSTELLAR_WBOUNDARIES",
    "CIRCUMSTELLAR_IRRADIATION",
    "CIRCUMSTELLAR_SINKS",
    "CIRCUMSTELLAR_PLANET_GROWTH",
    "GRAVITY_FROM_STARS_PLANETS_ONLY",
    "CENTRAL_MASS_POTENTIAL",
    "BINARY_POTENTIAL",
    "LOCALLY_ISOTHERM_DISK",
    "SPECIAL_BOUNDARY",
    "COAXIAL_BOUNDARIES",
    "WINDTUNNEL",
    "WINDTUNNEL_COORD",
    "WINDTUNNEL_EXTERNAL_SOURCE",
    "BOUNDARY_INFLOWOUTFLOW_MINID",
    "BOUNDARY_INFLOWOUTFLOW_MAXID",
    "BOUNDARY_REFL_FLUIDSIDE_MINID",
    "BOUNDARY_REFL_FLUIDSIDE_MAXID",
    "BOUNDARY_REFL_SOLIDSIDE_MINID",
    "BOUNDARY_REFL_SOLIDSIDE_MAXID",
    "BOUNDARY_REFL_ACTS_AS_SOURCE",
    "BOUNDARY_STICKY_MINID",
    "BOUNDARY_STICKY_MAXID",
    "GFM",
    "GFM_STELLAR_EVOLUTION",
    "GFM_CONST_IMF",
    "GFM_VARIABLE_IMF",
    "GFM_PREENRICH",
    "GFM_EXACT_NUMNGB",
    "GFM_WINDS",
    "GFM_WINDS_VARIABLE",
    "GFM_WINDS_VARIABLE_HUBBLE",
    "GFM_WINDS_HUBBLESCALING",
    "GFM_WINDS_MASSSCALING",
    "GFM_WIND_ENERGY_METAL_DEPENDENCE",
    "GFM_WIND_ENERGY_METAL_DEPENDENCE_TANH",
    "GFM_WINDS_STRIPPING",
    "GFM_WINDS_THERMAL",
    "GFM_WINDS_THERMAL_NEWDEF",
    "GFM_BIPOLAR_WINDS",
    "GFM_WINDS_LOCAL",
    "GFM_STELLAR_FEEDBACK",
    "GFM_PRIMORDIAL_RATES",
    "GFM_COOLING_METAL",
    "GFM_UVB_CORRECTIONS",
    "GFM_AGN_RADIATION",
    "GFM_STELLAR_PHOTOMETRICS",
    "GFM_OUTPUT_MASK",
    "GFM_DUST",
    "GFM_DUST_DESTMODE",
    "GFM_DUST_SPUTTERING",
    "GFM_CHECKS",
    "GFM_DISCARD_ENRICHMENT_GRADIENTS",
    "GFM_NORMALIZED_METAL_ADVECTION",
    "GFM_OUTPUT_BIRTH_POS",
    "GFM_CHEMTAGS",
    "GFM_WINDS_SAVE_PARTTYPE",
    "GFM_DISCRETE_ENRICHMENT",
    "GFM_SPLITFE",
    "GFM_RPROCESS",
    "RADCOOL",
    "RADCOOL_HOTHALO",
    "RADCOOL_HOTHALO_METAL_BOOST",
    "FM_SFR",
    "FM_STAR_FEEDBACK",
    "FM_STAR_FEEDBACK_KICK_TYPE",
    "NON_STOCHASTIC_MOMENTUM_FEEDBACK",
    "INJECT_INTO_SINGLE_CELL",
    "DIRECT_MOMENTUM_INJECTION_FEEDBACK",
    "OUTPUT_SF_PROBABILITY",
    "TEST_SFR",
    "USE_POLYTROPIC_EQSTATE",
    "DELAYED_COOLING",
    "SHUTOFFTIME_UPDATE",
    "DELAYED_COOLING_TURB",
    "INSTANTANEOUS_DEPOSITION",
    "EXPLICIT_COOLING",
    "COMPUTE_SFR_FROM_H2",
    "TEST_COOLING_METAL",
    "OUTPUT_STELLAR_FEEDBACK",
    "OUTPUT_MOLECULAR_FRACTION",
    "OUTPUT_OPTICAL_DEPTH",
    "RADPRESS_OPT_THIN",
    "RADPRESS_OPT_THIN_LUMPERMASS",
    "RADPRESS_OPT_THICK",
    "FM_RADIATION_FEEDBACK",
    "FM_RADIATION_FEEDBACK_DEBUG",
    "FM_EARLY_STAR_FEEDBACK",
    "FM_EARLY_STAR_FEEDBACK_KICK_TYPE",
    "OUTPUT_EARLY_STELLAR_FEEDBACK",
    "FM_MASS_WEIGHT_SN",
    "FM_VAR_SN_EFF",
    "MONOTONE_CONDUCTION",
    "CONDUCTION_ISOTROPIC",
    "CONDUCTION_ANISOTROPIC",
    "CONDUCTION_CONSTANT",
    "CONDUCTION_SATURATION",
    "IMPLICIT_TI",
    "SEMI_IMPLICIT_TI",
    "RESTRICT_KAPPA",
    "MULTIPLE_TIMESTEP",
    "NON_LINEAR_SLOPE_LIMITERS",
    "OTVET",
    "OTVET_CHEMISTRY_PS2009",
    "OTVET_CHEMISTRY_PS2011",
    "OTVET_NOGRAVITY",
    "OTVET_NOTMOVEGAS",
    "OTVET_OUTPUT_ET",
    "EDDINGTON_TENSOR_STARS",
    "OTVET_MODIFY_EDDINGTON_TENSOR",
    "OTVET_FLUXLIMITER",
    "OTVET_CHANGEFLUXLIMITER",
    "OTVET_FIXTIMESTEP",
    "OTVET_COOLING_HEATING",
    "OTVET_MULTI_FREQUENCY",
    "OTVET_INCLUDE_HE",
    "OTVET_SCATTER_SOURCE",
    "OTVET_OUTPUT_SOURCEHSML",
    "OTVET_CHECK_PHOTONCOUNT",
    "OTVET_NOCOLLISION_IONIZATION",
    "OTVET_SILENT",
    "TGSET",
    "TGCHEM",
    "HEALRAY",
    "SINKS",
    "SIDM",
    "SIDM_CONST_CROSS",
    "SIDM_STATES",
    "SIDM_REACTIONS",
    "SIDM_READ_ICS",
    "SIDM_NO_SCATTER",
    "SIDM_NO_TIMESTEP",
    "SIDM_NO_KINEMATICS",
    "SIDM_NO_NGB_SEL",
    "SIDM_NO_MASSCHANGE",
    "ISM",
    "ISM_LOCAL_RADIATION_PRESSURE",
    "ISM_LONG_RANGE_RADIATION_PRESSURE",
    "ISM_HII_PHOTO_HEATING",
    "ISM_H2_SFR",
    "ISM_OUTPUT_FIELDS",
    "SHOCK_FINDER_BEFORE_OUTPUT",
    "SHOCK_FINDER_ON_THE_FLY",
    "SHOCK_FINDER_POST_PROCESSING",
    "SHOCK_FINDER_AREPO",
    "UNLIMITED_GRADIENTS",
    "ZONE_JUMP_P",
    "ZONE_JUMP_T",
    "SHOCK_DIR_GRAD_T",
    "SHOCK_JUMP_T",
    "SURFACE_SPHERE_APPROX",
    "SURFACE_ANGLE_APPROX",
    "RESET_WRONG_JUMPS",
    "RESET_WRONG_RHO_JUMPS",
    "RESET_WRONG_P_JUMPS",
    "OUTPUT_GRAVITY_FRACTION",
    "SKIP_BORDER",
    "ATOMIC_DM",
    "STAR_INDEX",
    "BINARYLOG",
    "SPECIAL_SOFTENINGS",
    "REDUCE_SOFTENINGS",
    "ID_RGCORE",
    "DMLOWESTTIMEBIN",
    "DMFIXED",
    "FLD",
    "FLD_CONES",
    "FLD_NCONES",
    "FLD_CONST_KAPPA",
    "FLD_MARSHAK",
    "FLD_HYPRE",
    "FLD_HYPRE_IJ1",
    "FLD_HYPRE_IJ2",
    "HYPRE_PCG",
    "FLD_MG",
    "FLD_MG_GS",
    "FLD_ANISOTROPIC_CIRCULAR",
    "FLD_NO_TEMP_UPDATE",
    "FLD_SILENT",
    "FLD_TEST_BOUNDARY",
    "FLD_UPPER_BOUNDARY_MINID",
    "FLD_UPPER_BOUNDARY_MAXID",
    "FLD_LOWER_BOUNDARY_MINID",
    "FLD_LOWER_BOUNDARY_MAXID",
    "DG",
    "DG_SET_IC_FROM_AVERAGES",
    "DG_TEST_PROBLEM",
    "DG_VERBOSE",
    "DG_DEBUG",
    "DEGREE_K",
    "CALC_QUADRATURE_DATA",
    "MINMOD_B",
    "DISCONTINUITY_DETECTION",
    "OUTPUT_DG_DISCONTINUITIES",
    "OUTPUT_DG_INFLOW_BOUNDARIES",
    "ANGLE_BOUND",
    "CHARACTERISTIC_LIMITER",
    "CONSERVED_LIMITER",
    "POSITIVITY_LIMITER",
    "FIX_MEAN_VALUES",
    "CONDUCTION",
    "GRACKLE",
    "GRACKLE_H2",
    "GRACKLE_D",
    "GRACKLE_TAB",
    "GRACKLE_ABUNDANCE_IN_ICS",
    "GRACKLE_VERBOSE",
    "SINK_PARTICLES",
    "DUMP_SINK_PARTICLE_INFO",
    "SINK_PARTICLES_SKIM_CELL_MASS",
    "SINK_SIMPLEX",
    "SINK_PARTICLES_VARIABLE_ACC_RADIUS",
    "SINK_PARTICLES_FEEDBACK",
    "SINK_PARTICLES_FORCE_FORMATION",
    "SGCHEM",
    "SGCHEM_NO_HIGHN_DCHEM",
    "SGCHEM_NO_MOLECULES",
    "CHEMISTRYNETWORK",
    "JEANS_REFINEMENT",
    "NO_TARGET_MASS_CONDITION",
    "STATIC_CHEMISTRY_TEST",
    "ADVECTION_ONLY",
    "CHEM_IMAGE",
    "ABHE",
    "SG_HEADER_FLAG",
    "DEBUG_EVOLVE",
    "DEBUG_RATE_EQ",
    "DEBUG_PARTICLE_ID",
    "SGCHEM_TEMPERATURE_FLOOR",
    "SGCHEM_ACCRETION_LUMINOSITY",
    "SGCHEM_CONSTANT_ALPHAB",
    "SGCHEM_DISABLE_COMPTON_COOLING",
    "TREE_RAD",
    "TREE_RAD_H2",
    "TREE_RAD_CO",
    "NSIDE",
    "OUTPUTCOL",
    "TREE_RAD_VEL",
    "SWEEP_NDIR",
    "SWEEP_SOURCES",
    "SWEEP_NUM_ROTATIONS",
    "SWEEP_HYDROGEN_ONLY",
    "SWEEP_OUTPUT_FLUX",
    "SWEEP_SGCHEM",
    "SWEEP_SGCHEM_RECOMBINE",
    "MAX_VARIATION_TOLERANCE",
    "MRT",
    "MRT_COMOVING",
    "MRT_INIT_IONIZATION",
    "MRT_OUTPUT_FLUX",
    "MRT_TIME_EXTRAPOLATION",
    "MRT_FLUX_EXTRAPOLATION",
    "MRT_COOLING_HEATING",
    "MRT_RADIATION_PRESSURE",
    "MRT_INCLUDE_HE",
    "MRT_LSF_GRADIENTS",
    "MRT_RIEMANN_ROSUNOV",
    "MRT_RIEMANN_ROSUNOV_NEW",
    "MRT_RIEMANN_HLLE",
    "MRT_RIEMANN_HLLE_NEW",
    "MRT_MULTI_FREQUENCY",
    "MRT_CHEMISTRY_PS2009",
    "MRT_CHEMISTRY_PS2011",
    "MRT_COUPLED_THERMOCHEMISTRY",
    "MRT_NO_OTSA",
    "MRT_NOCOLLISION_IONIZATION",
    "MRT_SLOWLIGHT",
    "MRT_CONSTANT_KAPPA",
    "MRT_IR",
    "MRT_IR_ONLY_CHEMISTRY",
    "MRT_IR_LTE",
    "MRT_IR_LTE_SEMI_IMPLICIT",
    "MRT_IR_LTE_GSL",
    "MRT_IR_PHOTON_TRAPPING",
    "MRT_IR_GRAIN_KAPPA",
    "MRT_UV_ONLY_DUST",
    "MRT_NO_UV",
    "MRT_SETUP_SPECIAL_BOUNDARIES",
    "MRT_LEVITATION_TEST",
    "MRT_REDUCE_OUTPUT",
    "MRT_EQUIL_CHEM_COOL",
    "MRT_MOLECULAR_COOLING",
    "MRT_PHOTOELECTRIC_HEATING",
    "MRT_METAL_COOLING",
    "MRT_UVB",
    "MRT_UPDATE_AT_END_OF_STEP",
    "MRT_STARS",
    "MRT_STARS_EXACT_NGB",
    "MRT_BH",
    "MRT_BH_EXACT_NGB",
    "MRT_BH_PULSED",
    "MRT_BH_UV_INJECTION",
    "MRT_BH_IR_INJECTION",
    "MRT_BH_BIPOLAR",
    "MRT_BH_BIPOLAR_SET_FLUX",
    "MRT_BH_OMEGA_WEIGHT",
    "MRT_LOCAL_FEEDBACK",
    "MRT_CHEM_SG",
    "MRT_SOURES",
    "OUTPUT_MCTRNUM",
    "SGCHEM_SUPPRESS_DVODE_WARNING",
    "SNE_FEEDBACK",
    "INJECT_TRACER_INTO_SN",
    "POPIII_SNE",
    "INSTANT_EXPLOSIONS",
    "SINK_PARTICLES_FEEDBACK_RETURN_MASS",
    "SINK_FEEDBACK_SINGLE_STAR",
    "SINK_PARTICLES_OUTPUT_EVERY_NEW_SINK",
    "SX_CLEAR_PHOTONS_BEFORE_RUN",
    "SX_LOAD_BALANCE",
    "SX_DISPLAY_LOAD",
    "SWEEP",
    "SWEEP_PERIODIC",
    "SWEEP_SCATTER",
    "SWEEP_NO_WARMSTARTING",
    "SWEEP_OUTPUT_IONIZATION_TIME",
    "L25n512_5001_COMMIT_61d5e16",
    "L25n512_5002_COMMIT_79dcf6f",
    "L25n512_5003_COMMIT_f17c79d",
    "FREYA_TREEFIND_CRASH_FIX",
    "L25n512_5004_COMMITS_d17aee4_795679d",
    "L25n512_5005_COMMIT_fd10b24",
    "L25n512_5008_COMMIT_6f1e66c",
    "SIMPLEX",
    "SX_CHEMISTRY",
    "SX_NDIR",
    "SX_NUM_ROT",
    "SX_RECOMBINE",
    "SX_SOURCES",
    "SX_HYDROGEN_ONLY",
];

pub static PARAM_FILE_PARAMS: &[&str] = &[
    "ActivePartFracForNewDomainDecomp",
    "AtomicCoolOption",
    "BoxSize",
    "CarbAbund",
    "CellMaxAngleFactor",
    "CellShapingSpeed",
    "ComovingIntegrationOn",
    "CoolingOn",
    "CosmicRayIonRate",
    "CourantFac",
    "CpuTimeBetRestartFile",
    "DerefinementCriterion",
    "DesNumNgb",
    "DeutAbund",
    "DustToGasRatio",
    "ErrTolForceAcc",
    "ErrTolIntAccuracy",
    "ErrTolTheta",
    "ExternalDustExtinction",
    "GasSoftFactor",
    "GravityConstantInternal",
    "H2FormEx",
    "H2FormKin",
    "H2OpacityOption",
    "HubbleParam",
    "ICFormat",
    "InitCondFile",
    "InitDustTemp",
    "InitGasTemp",
    "InitRedshift",
    "ISRFOption",
    "LimitUBelowCertainDensityToThisValue",
    "LimitUBelowThisDensity",
    "LWBGStartRedsh",
    "LWBGType",
    "MAbund",
    "MaxMemSize",
    "MaxNumNgbDeviation",
    "MaxSizeTimestep",
    "MinEgySpec",
    "MinGasTemp",
    "MinimumDensityOnStartUp",
    "MinNumPhotons",
    "MinSizeTimestep",
    "MultipleDomains",
    "NumFilesPerSnapshot",
    "NumFilesWrittenInParallel",
    "Omega0",
    "OmegaBaryon",
    "OmegaLambda",
    "OutputDir",
    "OutputListFilename",
    "OutputListOn",
    "OxyAbund",
    "PeriodicBoundariesOn",
    "PhotoApprox",
    "PicXaxis",
    "PicXmax",
    "PicXmin",
    "PicXpixels",
    "PicYaxis",
    "PicYmax",
    "PicYmin",
    "PicYpixels",
    "PicZaxis",
    "PicZmax",
    "PicZmin",
    "ReferenceGasPartMass",
    "RefinementCriterion",
    "ResubmitCommand",
    "ResubmitOn",
    "SGChemConstInitAbundances",
    "SGChemInitCHxAbund",
    "SGChemInitCOAbund",
    "SGChemInitCPAbund",
    "SGChemInitDIIAbund",
    "SGChemInitH2Abund",
    "SGChemInitHCOPAbund",
    "SGChemInitHDAbund",
    "SGChemInitHeIIIAbund",
    "SGChemInitHePAbund",
    "SGChemInitHPAbund",
    "SGChemInitMPAbund",
    "SGChemInitOHxAbund",
    "SnapFormat",
    "SnapshotFileBase",
    "SofteningComovingType0",
    "SofteningComovingType1",
    "SofteningComovingType2",
    "SofteningComovingType3",
    "SofteningComovingType4",
    "SofteningComovingType5",
    "SofteningMaxPhysType0",
    "SofteningMaxPhysType1",
    "SofteningMaxPhysType2",
    "SofteningMaxPhysType3",
    "SofteningMaxPhysType4",
    "SofteningMaxPhysType5",
    "SofteningTypeOfPartType0",
    "SofteningTypeOfPartType1",
    "SofteningTypeOfPartType2",
    "SofteningTypeOfPartType3",
    "SofteningTypeOfPartType4",
    "SofteningTypeOfPartType5",
    "StarformationOn",
    "sxLoadFactor",
    "TargetGasMassFactor",
    "TestSrcFile",
    "TestSourcePosX",
    "TestSourcePosY",
    "TestSourcePosZ",
    "TestSourceRate056",
    "TestSourceRate112",
    "TestSourceRate136",
    "TestSourceRate152",
    "TestSourceRate246",
    "TestSrcFile",
    "TestSrcFile",
    "TestSrcFile",
    "SweepConvergenceThreshold",
    "SweepMaxNumIterations",
    "SweepSigmaScatter",
    "SourceFactor",
    "TreecoolFile",
    "DesNumNgbEnrichment",
    "MaxNumNgbDeviationEnrichment",
    "SNII_MinMass_Msun",
    "SNII_MaxMass_Msun",
    "IMF_MinMass_Msun",
    "CritOverDensity",
    "CritPhysDensity",
    "TemperatureThresh",
    "FactorSN",
    "FactorEVP",
    "TempSupernova",
    "TempClouds",
    "MaxSfrTimescale",
    "IMF_MaxMass_Msun",
    "AGB_MassTransferOn",
    "SNIa_Rate_Norm",
    "SNIa_Rate_TAU",
    "SNIa_MassTransferOn",
    "SNII_MassTransferOn",
    "YieldTablePath",
    "CoolingTablePath",
    "MinMetalTemp",
    "TimeBegin",
    "TimeBetSnapshot",
    "TimeBetStatistics",
    "TimeLimitCPU",
    "TimeMax",
    "TimeOfFirstSnapshot",
    "TopNodeFactor",
    "TypeOfOpeningCriterion",
    "TypeOfTimestepCriterion",
    "UnitLength_in_cm",
    "UnitMass_in_g",
    "UnitPhotons_per_s",
    "UnitVelocity_in_cm_per_s",
    "UVFieldStrength",
    "ZAtom",
    "PicXpixels",
    "PicYpixels",
    "PicXaxis",
    "PicYaxis",
    "PicZaxis",
    "PicXmin",
    "PicXmax",
    "PicYmin",
    "PicYmax",
    "PicZmin",
    "PicZmax",
    "TimeBetweenImages",
    "FlushCpuTimeDiff",
    "DesLinkNgb",
    "ErrTolThetaSubfind",
    "MinimumComovingHydroSoftening",
    "AdaptiveHydroSofteningSpacing",
    "BlackHoleCenteringMassMultiplier",
    "TimeBetOnTheFlyFoF",
    "BlackHoleAccretionFactor",
    "BlackHoleFeedbackFactor",
    "BlackHoleEddingtonFactor",
    "SeedBlackHoleMass",
    "MinFoFMassForNewSeed",
    "DesNumNgbBlackHole",
    "BlackHoleMaxAccretionRadius",
    "BlackHoleRadiativeEfficiency",
    "SelfShieldingFile",
    "TreecoolFileAGN",
    "SelfShieldingDensity",
    "ObscurationFactor",
    "ObscurationSlope",
    "RadioFeedbackFactor",
    "RadioFeedbackMinDensityFactor",
    "RadioFeedbackReiorientationFactor",
    "QuasarThreshold",
    "WindEnergyReductionFactor",
    "WindEnergyReductionMetallicity",
    "WindEnergyReductionExponent",
    "NSNS_MassTransferOn",
    "NSNS_Rate_TAU",
    "NSNS_MassPerEvent",
    "NSNS_per_SNIa",
    "PreEnrichTime",
    "PreEnrichAbundanceFile",
    "PhotometricsTablePath",
    "FactorForSofterEQS",
    "TempForSofterEQS",
    "WindDumpFactor",
    "WindEnergyIn1e51erg",
    "WindFreeTravelMaxTimeFactor",
    "WindFreeTravelDensFac",
    "ThermalWindFraction",
    "VariableWindVelFactor",
    "VariableWindSpecMomentum",
    "MinWindVel",
    "MHDSeedDir",
    "MHDSeedValue",
];
