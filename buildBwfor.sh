# module unload lib/hdf5/1.8-intel-16.0 
# module unload devel/python_intel/3.6
# module unload mpi/impi/5.1.3-intel-16.0
# module unload numlib/gsl/2.2.1-intel-16.0
# module unload numlib/fftw/3.3.5-impi-5.1.3-intel-16.0
# module unload compiler/intel/16.0
# module load devel/cmake/3.17.3
# module load lib/hdf5/1.10-gnu-7.1
export HDF5_VERSION="1.8.18"
export HDF5_DIR=/opt/bwhpc/common/lib/hdf5/1.8.18-intel-16.0
cargo build --release --features bwfor
