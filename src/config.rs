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
#SBATCH --ntasks-per-node={coresPerNode}
#SBATCH --time={wallTime}
#SBATCH --mem=50gb
#SBATCH --output={logFile}
#SBATCH --export=HDF5_DISABLE_VERSION_CHECK=2
{jobLines}
module load compiler/intel/16.0
module load mpi/impi/5.1.3-intel-16.0
module load numlib/gsl/2.2.1-intel-16.0
module load numlib/fftw/3.3.5-impi-5.1.3-intel-16.0
module load lib/hdf5/1.8-intel-16.0
module load devel/python_intel/3.6
startexe=\"mpirun {runCommand}\"
exec $startexe";

#[cfg(not(feature = "bwfor"))]
pub static JOB_FILE_TEMPLATE: &str = "#!/bin/bash
{jobLines}
mpirun -n {numCores} {runCommand} > {logFile}";

pub static CONFIG_FILE_PARAMS: &'static [&'static str] = &[
    "SX_SWEEP",
    "CHEM_IMAGE",
    "CHEMISTRYNETWORK",
    "CHUNKING",
    "DO_NOT_RANDOMIZE_DOMAINCENTER",
    "DOUBLEPRECISION",
    "DOUBLEPRECISION_FFTW",
    "EVALPOTENTIAL",
    "FIX_PATHSCALE_MPI_STATUS_IGNORE_BUG",
    "HAVE_HDF5",
    "HOST_MEMORY_REPORTING",
    "IMPOSE_PINNING",
    "INPUT_IN_DOUBLEPRECISION",
    "JEANS_REFINEMENT",
    "MAX_VARIATION_TOLERANCE",
    "NO_ISEND_IRECV_IN_DOMAIN",
    "NO_MPI_IN_PLACE",
    "NTYPES",
    "OUTPUT_IN_DOUBLEPRECISION",
    "OUTPUT_TASK",
    "PERIODIC",
    "REFINEMENT_MERGE_CELLS",
    "REFINEMENT_SPLIT_CELLS",
    "REGULARIZE_MESH_CM_DRIFT",
    "REGULARIZE_MESH_CM_DRIFT_USE_SOUNDSPEED",
    "REGULARIZE_MESH_FACE_ANGLE",
    "SELFGRAVITY",
    "SGCHEM",
    "SGCHEM_CONSTANT_ALPHAB",
    "SGCHEM_DISABLE_COMPTON_COOLING",
    "SGCHEM_NO_MOLECULES",
    "SIMPLEX",
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
    "VORONOI",
    "VORONOI_DYNAMIC_UPDATE",
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
