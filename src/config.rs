use crate::job_params::SystemConfiguration;

pub static DEFAULT_BOB_CONFIG_NAME: &str = "sims.bob";
pub static DEFAULT_PARAM_FILE_NAME: &str = "param.txt";
pub static DEFAULT_CONFIG_FILE_NAME: &str = "Config.sh";
pub static DEFAULT_JOB_FILE_NAME: &str = "job";

#[cfg(feature = "bwfor")]
pub static DEFAULT_AREPO_FOLDER: &str = "/beegfs/home/hd/hd_hd/hd_hp240/projects/phd/arepo";
#[cfg(not(feature = "bwfor"))]
pub static DEFAULT_AREPO_FOLDER: &str = "/home/toni/projects/phd/arepo";

pub static DEFAULT_AREPO_EXECUTABLE_NAME: &str = "Arepo";
pub static DEFAULT_AREPO_SOURCE_FOLDER: &str = "src";
pub static DEFAULT_AREPO_CONFIG_BUILD_FILE: &str = "build/arepoconfig.h";
pub static DEFAULT_AREPO_CONFIG_SOURCE_FILE: &str = "src/arepoconfig.h";

#[cfg(feature = "bwfor")]
pub static JOB_FILE_TEMPLATE: &str = "#!/bin/bash
#SBATCH --partition={partition}
#SBATCH --nodes={numNodes}
#SBATCH --ntasks-per-node={numCoresPerNode}
#SBATCH --time={wallTime}
#SBATCH --mem=50gb
#SBATCH --output={logFile}
#SBATCH --export=HDF5_DISABLE_VERSION_CHECK=2
module load compiler/intel/16.0
module load mpi/impi/5.1.3-intel-16.0
module load numlib/gsl/2.2.1-intel-16.0
module load numlib/fftw/3.3.5-impi-5.1.3-intel-16.0
module load lib/hdf5/1.8-intel-16.0
module load devel/python_intel/3.6
startexe=\"mpirun {runCommand}\"
exec $startexe";

#[cfg(feature = "bwfor")]
pub static RUN_COMMAND: &str = "sbatch";

#[cfg(feature = "bwfor")]
pub static SYSTEM_CONFIG: &SystemConfiguration = &SystemConfiguration {
    max_num_cores: 1024,
    max_num_cores_per_node: 16,
};

#[cfg(not(feature = "bwfor"))]
pub static JOB_FILE_TEMPLATE: &str = "#!/bin/bash
mpirun -n {numCores} {runCommand} > {logFile}";

#[cfg(not(feature = "bwfor"))]
pub static RUN_COMMAND: &str = "bash";

#[cfg(not(feature = "bwfor"))]
pub static SYSTEM_CONFIG: &SystemConfiguration = &SystemConfiguration {
    max_num_cores: 1024,
    max_num_cores_per_node: 16,
};

pub static DEFAULT_LOG_FILE: &str = "stdout.log";
pub static DEFAULT_JOB_NAME: &str = "arepoTest";
pub static DEFAULT_WALL_TIME: &str = "3:00:00";
pub static DEFAULT_PARTITION: &str = "single";
pub static DEFAULT_NUM_CORES: &i64 = &1;
pub static DEFAULT_RUN_COMMAND: &str = &"./Arepo param.txt 0";

pub static NX_SLICE: usize = 1;
pub static NY_SLICE: usize = 1;
pub static PIC_FILE_ENDING: &str = "png";

pub static CONFIG_FILE_PARAMS: &'static [&'static str] = &[
    "ADAPTIVE_HYDRO_SOFTENING",
    "CHEMISTRYNETWORK",
    "CHEM_IMAGE",
    "CHUNKING",
    "DEREFINE_GENTLY",
    "DOUBLEPRECISION",
    "DOUBLEPRECISION_FFTW",
    "DO_NOT_RANDOMIZE_DOMAINCENTER",
    "DUMP_SINK_PARTICLE_INFO",
    "ENTROPY_MACH_THRESHOLD",
    "EVALPOTENTIAL",
    "FIX_PATHSCALE_MPI_STATUS_IGNORE_BUG",
    "GENERATE_GAS_IN_ICS",
    "HAVE_HDF5",
    "HOST_MEMORY_REPORTING",
    "IMPOSE_PINNING",
    "INJECT_TRACER_INTO_SN",
    "INPUT_IN_DOUBLEPRECISION",
    "INSTANT_EXPLOSIONS",
    "JEANS_REFINEMENT",
    "LONGIDS",
    "MAX_VARIATION_TOLERANCE",
    "NO_ISEND_IRECV_IN_DOMAIN",
    "NO_MPI_IN_PLACE",
    "NO_TARGET_MASS_CONDITION",
    "NTYPES",
    "OUTPUT_IN_DOUBLEPRECISION",
    "OUTPUT_MCTRNUM",
    "OUTPUT_TASK",
    "PERIODIC",
    "POPIII_SNE",
    "REFINEMENT_MERGE_CELLS",
    "REFINEMENT_SPLIT_CELLS",
    "REFINEMENT_VOLUME_LIMIT",
    "REGULARIZE_MESH_CM_DRIFT",
    "REGULARIZE_MESH_CM_DRIFT_USE_SOUNDSPEED",
    "REGULARIZE_MESH_FACE_ANGLE",
    "SELFGRAVITY",
    "SGCHEM",
    "SGCHEM_CONSTANT_ALPHAB",
    "SGCHEM_DISABLE_COMPTON_COOLING",
    "SGCHEM_NO_MOLECULES",
    "SGCHEM_SUPPRESS_DVODE_WARNING",
    "SIMPLEX",
    "SINK_FEEDBACK_SINGLE_STAR",
    "SINK_PARTICLES",
    "SINK_PARTICLES_FEEDBACK",
    "SINK_PARTICLES_FEEDBACK_RETURN_MASS",
    "SINK_PARTICLES_SKIM_CELL_MASS",
    "SNE_FEEDBACK",
    "SPLIT_PARTICLE_TYPE=2",
    "SUBBOX_SNAPSHOTS",
    "SX_CHEMISTRY",
    "SX_DISPLAY_LOAD",
    "SX_DISPLAY_STATS",
    "SX_DISPLAY_TIMERS",
    "SX_HYDROGEN_ONLY",
    "SX_LOAD_BALANCE",
    "SX_NDIR",
    "SX_NUM_ROT",
    "SX_OUTPUT_FLUX",
    "SX_OUTPUT_IMAGE",
    "SX_OUTPUT_IMAGE_ALL",
    "SX_SOURCES",
    "SX_SWEEP",
    "TREE_BASED_TIMESTEPS",
    "USE_ENTROPY_FOR_COLD_FLOWS",
    "VORONOI",
    "VORONOI_DYNAMIC_UPDATE",
    "VORONOI_IMAGES_FOREACHSNAPSHOT",
    "VORONOI_NEW_IMAGE",
];

pub static PARAM_FILE_PARAMS: &'static [&'static str] = &[
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
];
