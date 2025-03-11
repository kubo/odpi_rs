use crate::types::VectorFormat;
use crate::utils::*;
use crate::*;
use odpic_sys::*;
use std::borrow::Cow;
use std::ffi::c_void;
use std::ptr;
use std::slice;

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum VectorInfo<'a> {
    Binary(Cow<'a, [u8]>),
    Int8(Cow<'a, [i8]>),
    Float32(Cow<'a, [f32]>),
    Float64(Cow<'a, [f64]>),
    SparseBinary {
        ndims: u32,
        values: Cow<'a, [u8]>,
        indices: Cow<'a, [u32]>,
    },
    SparseInt8 {
        ndims: u32,
        values: Cow<'a, [i8]>,
        indices: Cow<'a, [u32]>,
    },
    SparseFloat32 {
        ndims: u32,
        values: Cow<'a, [f32]>,
        indices: Cow<'a, [u32]>,
    },
    SparseFloat64 {
        ndims: u32,
        values: Cow<'a, [f64]>,
        indices: Cow<'a, [u32]>,
    },
}

impl VectorInfo<'_> {
    pub fn vector_format(&self) -> VectorFormat {
        match self {
            VectorInfo::Binary(_) => VectorFormat::Binary,
            VectorInfo::Int8(_) => VectorFormat::Int8,
            VectorInfo::Float32(_) => VectorFormat::Float32,
            VectorInfo::Float64(_) => VectorFormat::Float64,
            VectorInfo::SparseBinary { .. } => VectorFormat::Binary,
            VectorInfo::SparseInt8 { .. } => VectorFormat::Int8,
            VectorInfo::SparseFloat32 { .. } => VectorFormat::Float32,
            VectorInfo::SparseFloat64 { .. } => VectorFormat::Float64,
        }
    }

    pub fn num_dimentions(&self) -> usize {
        match self {
            VectorInfo::Binary(v) => 8 * v.len(),
            VectorInfo::Int8(v) => v.len(),
            VectorInfo::Float32(v) => v.len(),
            VectorInfo::Float64(v) => v.len(),
            VectorInfo::SparseBinary { ndims, .. } => *ndims as usize,
            VectorInfo::SparseInt8 { ndims, .. } => *ndims as usize,
            VectorInfo::SparseFloat32 { ndims, .. } => *ndims as usize,
            VectorInfo::SparseFloat64 { ndims, .. } => *ndims as usize,
        }
    }
}

impl ToDpi<dpiVectorInfo> for VectorInfo<'_> {
    fn to_dpi(&self) -> dpiVectorInfo {
        match self {
            VectorInfo::Binary(v) => dpiVectorInfo {
                format: DPI_VECTOR_FORMAT_BINARY,
                numDimensions: 8 * v.len() as u32,
                dimensionSize: 1,
                dimensions: dpiVectorDimensionBuffer {
                    asPtr: v.as_ptr() as *mut c_void,
                },
                numSparseValues: 0,
                sparseIndices: ptr::null_mut(),
            },
            VectorInfo::Int8(v) => dpiVectorInfo {
                format: DPI_VECTOR_FORMAT_INT8,
                numDimensions: v.len() as u32,
                dimensionSize: 1,
                dimensions: dpiVectorDimensionBuffer {
                    asPtr: v.as_ptr() as *mut c_void,
                },
                numSparseValues: 0,
                sparseIndices: ptr::null_mut(),
            },
            VectorInfo::Float32(v) => dpiVectorInfo {
                format: DPI_VECTOR_FORMAT_FLOAT32,
                numDimensions: v.len() as u32,
                dimensionSize: 4,
                dimensions: dpiVectorDimensionBuffer {
                    asPtr: v.as_ptr() as *mut c_void,
                },
                numSparseValues: 0,
                sparseIndices: ptr::null_mut(),
            },
            VectorInfo::Float64(v) => dpiVectorInfo {
                format: DPI_VECTOR_FORMAT_FLOAT64,
                numDimensions: v.len() as u32,
                dimensionSize: 8,
                dimensions: dpiVectorDimensionBuffer {
                    asPtr: v.as_ptr() as *mut c_void,
                },
                numSparseValues: 0,
                sparseIndices: ptr::null_mut(),
            },
            VectorInfo::SparseBinary {
                ndims,
                values,
                indices,
            } => dpiVectorInfo {
                format: DPI_VECTOR_FORMAT_BINARY,
                numDimensions: *ndims,
                dimensionSize: 1,
                dimensions: dpiVectorDimensionBuffer {
                    asPtr: values.as_ptr() as *mut c_void,
                },
                numSparseValues: indices.len() as u32,
                sparseIndices: indices.as_ptr() as *mut u32,
            },
            VectorInfo::SparseInt8 {
                ndims,
                values,
                indices,
            } => dpiVectorInfo {
                format: DPI_VECTOR_FORMAT_INT8,
                numDimensions: *ndims,
                dimensionSize: 1,
                dimensions: dpiVectorDimensionBuffer {
                    asPtr: values.as_ptr() as *mut c_void,
                },
                numSparseValues: indices.len() as u32,
                sparseIndices: indices.as_ptr() as *mut u32,
            },
            VectorInfo::SparseFloat32 {
                ndims,
                values,
                indices,
            } => dpiVectorInfo {
                format: DPI_VECTOR_FORMAT_FLOAT32,
                numDimensions: *ndims,
                dimensionSize: 4,
                dimensions: dpiVectorDimensionBuffer {
                    asPtr: values.as_ptr() as *mut c_void,
                },
                numSparseValues: indices.len() as u32,
                sparseIndices: indices.as_ptr() as *mut u32,
            },
            VectorInfo::SparseFloat64 {
                ndims,
                values,
                indices,
            } => dpiVectorInfo {
                format: DPI_VECTOR_FORMAT_FLOAT64,
                numDimensions: *ndims,
                dimensionSize: 8,
                dimensions: dpiVectorDimensionBuffer {
                    asPtr: values.as_ptr() as *mut c_void,
                },
                numSparseValues: indices.len() as u32,
                sparseIndices: indices.as_ptr() as *mut u32,
            },
        }
    }
}

impl TryToRust<VectorInfo<'static>> for dpiVectorInfo {
    // Note: The lifetime parameter 'static is incorrect.
    // Its actual lifetime is the source of dpiVectorInfo.
    fn try_to_rust(&self) -> Result<VectorInfo<'static>> {
        let format: VectorFormat = self.format.try_to_rust()?;
        Ok(if self.numSparseValues == 0 {
            // dense vector
            match format {
                VectorFormat::Binary => VectorInfo::Binary(
                    unsafe {
                        slice::from_raw_parts(
                            self.dimensions.asPtr as *const u8,
                            (self.numDimensions / 8).try_into()?,
                        )
                    }
                    .into(),
                ),
                VectorFormat::Float32 => VectorInfo::Float32(
                    unsafe {
                        slice::from_raw_parts(
                            self.dimensions.asFloat,
                            self.numDimensions.try_into()?,
                        )
                    }
                    .into(),
                ),
                VectorFormat::Float64 => VectorInfo::Float64(
                    unsafe {
                        slice::from_raw_parts(
                            self.dimensions.asDouble,
                            self.numDimensions.try_into()?,
                        )
                    }
                    .into(),
                ),
                VectorFormat::Int8 => VectorInfo::Int8(
                    unsafe {
                        slice::from_raw_parts(
                            self.dimensions.asInt8,
                            self.numDimensions.try_into()?,
                        )
                    }
                    .into(),
                ),
            }
        } else {
            // sparse vector
            let ndims = self.numDimensions.try_into()?;
            let indices = unsafe {
                slice::from_raw_parts(self.sparseIndices, self.numSparseValues.try_into()?)
            }
            .into();
            match format {
                VectorFormat::Binary => VectorInfo::SparseBinary {
                    ndims,
                    values: unsafe {
                        slice::from_raw_parts(
                            self.dimensions.asPtr as *const u8,
                            self.numSparseValues.try_into()?,
                        )
                    }
                    .into(),
                    indices,
                },
                VectorFormat::Float32 => VectorInfo::SparseFloat32 {
                    ndims,
                    values: unsafe {
                        slice::from_raw_parts(
                            self.dimensions.asFloat,
                            self.numDimensions.try_into()?,
                        )
                    }
                    .into(),
                    indices,
                },
                VectorFormat::Float64 => VectorInfo::SparseFloat64 {
                    ndims,
                    values: unsafe {
                        slice::from_raw_parts(
                            self.dimensions.asDouble,
                            self.numDimensions.try_into()?,
                        )
                    }
                    .into(),
                    indices,
                },
                VectorFormat::Int8 => VectorInfo::SparseInt8 {
                    ndims,
                    values: unsafe {
                        slice::from_raw_parts(
                            self.dimensions.asInt8,
                            self.numDimensions.try_into()?,
                        )
                    }
                    .into(),
                    indices,
                },
            }
        })
    }
}
