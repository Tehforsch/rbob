# MESHRELAX
NTYPES=6
PERIODIC
IMPOSE_PINNING
VORONOI
REGULARIZE_MESH_CM_DRIFT
REGULARIZE_MESH_CM_DRIFT_USE_SOUNDSPEED
REGULARIZE_MESH_FACE_ANGLE
TREE_BASED_TIMESTEPS
REFINEMENT_SPLIT_CELLS
REFINEMENT_MERGE_CELLS
SELFGRAVITY
EVALPOTENTIAL
DO_NOT_RANDOMIZE_DOMAINCENTER
CHUNKING
DOUBLEPRECISION=1
DOUBLEPRECISION_FFTW
OUTPUT_IN_DOUBLEPRECISION
INPUT_IN_DOUBLEPRECISION
VORONOI_DYNAMIC_UPDATE
NO_MPI_IN_PLACE
NO_ISEND_IRECV_IN_DOMAIN
FIX_PATHSCALE_MPI_STATUS_IGNORE_BUG
OUTPUT_TASK
HAVE_HDF5
HOST_MEMORY_REPORTING
SGCHEM
SGCHEM_NO_MOLECULES
CHEMISTRYNETWORK=1
JEANS_REFINEMENT=8
CHEM_IMAGE
SGCHEM_CONSTANT_ALPHAB=2.59e-13
SGCHEM_DISABLE_COMPTON_COOLING
SIMPLEX
SX_CHEMISTRY=3
SX_SOURCES=10
SX_NUM_ROT=1
SX_HYDROGEN_ONLY
SX_DISPLAY_STATS
SX_DISPLAY_TIMERS
SX_OUTPUT_IMAGE
SX_OUTPUT_IMAGE_ALL
SX_OUTPUT_FLUX
SX_LOAD_BALANCE
SX_DISPLAY_LOAD
SX_NDIR=84
# SX_SWEEP
# SX_SWEEP_MOST_STRAIGHTFORWARD
MAX_VARIATION_TOLERANCE=100