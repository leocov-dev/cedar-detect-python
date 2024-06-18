use image::{GrayImage};
use cedar_detect;
use imageproc::rect::Rect;
use pyo3::{prelude::*, Bound, pyclass, pymethods, PyResult};

#[pyclass(frozen, module = "algorithm")]
pub struct StarDescription {
    pub(crate) inner: cedar_detect::algorithm::StarDescription,
}

impl StarDescription {
    pub fn wrap(inner: cedar_detect::algorithm::StarDescription) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl StarDescription {
    #[getter]
    pub fn centroid_x(&self) -> f32 {
        self.inner.centroid_x
    }
    #[getter]
    pub fn centroid_y(&self) -> f32 {
        self.inner.centroid_y
    }
    #[getter]
    pub fn peak_value(&self) -> u8 {
        self.inner.peak_value
    }
    #[getter]
    pub fn brightness(&self) -> f32 {
        self.inner.brightness
    }
    #[getter]
    pub fn num_saturated(&self) -> u16 {
        self.inner.num_saturated
    }

    fn __repr__(slf: &Bound<Self>) -> PyResult<String> {
        // This is the equivalent of `self.__class__.__name__` in Python.
        let binding = slf.get_type();
        let class_name = &binding.name()?;

        Ok(format!("{}(centroid_x={}, centroid_y={}, peak_value={}, brightness={}, num_saturated={})",
                   class_name,
                   slf.borrow().inner.centroid_x,
                   slf.borrow().inner.centroid_y,
                   slf.borrow().inner.peak_value,
                   slf.borrow().inner.brightness,
                   slf.borrow().inner.num_saturated,
        ))
    }
}

// TODO: investigate options for combined image type
#[pyfunction]
fn estimate_noise_from_image(width: u32, height: u32, data: Vec<u8>) -> f32 {
    let image = GrayImage::from_raw(width, height, data).unwrap();
    cedar_detect::algorithm::estimate_noise_from_image(&image)
}

// TODO: investigate options for combined image, rect type
#[pyfunction]
fn estimate_background_from_image_region(
    width: u32, height: u32, data: Vec<u8>,  // img
    x: i32, y: i32, w: i32, h: i32,          // rect
) -> (f32, f32) {
    let image = GrayImage::from_raw(width, height, data).unwrap();
    cedar_detect::algorithm::estimate_background_from_image_region(
        &image,
        &Rect::at(x, y).of_size(w as u32, h as u32),
    )
}


// TODO: investigate options for combined image type
#[pyfunction]
pub fn get_stars_from_image(
    width: u32, height: u32, data: Vec<u8>,  // img
    noise_estimate: f32,
    sigma: f32,
    max_size: u32,
    binning: u32,
    detect_hot_pixels: bool,
    return_binned_image: bool,
) -> (
    Vec<StarDescription>,
    /*hot_pixel_count*/i32,
    Option<Vec<u8>>,
    [u32; 256],
)
{
    let image = GrayImage::from_raw(width, height, data).unwrap();
    let (sd, hpc, img, hgrm) = cedar_detect::algorithm::get_stars_from_image(&image, noise_estimate, sigma, max_size, binning, detect_hot_pixels, return_binned_image);

    let mut img_result: Option<Vec<u8>> = None;
    if let Some(img) = img {
        img_result = Some(img.into_raw())
    }
    (sd.iter().map(|s| StarDescription::wrap(*s)).collect(), hpc, img_result, hgrm)
}

#[pyclass(frozen, module = "algorithm")]
pub struct RegionOfInterestSummary {
    pub(crate) inner: cedar_detect::algorithm::RegionOfInterestSummary,
}

impl RegionOfInterestSummary {
    pub fn wrap(inner: cedar_detect::algorithm::RegionOfInterestSummary) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl RegionOfInterestSummary {
    #[getter]
    pub fn histogram(&self) -> [u32; 256] {
        self.inner.histogram
    }

    #[getter]
    pub fn peak_x(&self) -> f32 {
        self.inner.peak_x
    }

    #[getter]
    pub fn peak_y(&self) -> f32 {
        self.inner.peak_y
    }

    fn __repr__(slf: &Bound<Self>) -> PyResult<String> {
        // This is the equivalent of `self.__class__.__name__` in Python.
        let binding = slf.get_type();
        let class_name = &binding.name()?;

        let front = &slf.borrow().inner.histogram[0..3];
        let back = &slf.borrow().inner.histogram[253..256];

        Ok(format!("{}(peak_x={}, peak_y={}, histogram=[{}...{}])",
                   class_name,
                   slf.borrow().inner.peak_x,
                   slf.borrow().inner.peak_y,
                   front.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "),
                   back.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "),
        ))
    }
}

// TODO: investigate options for combined image, rect type
#[pyfunction]
pub fn summarize_region_of_interest(
    width: u32, height: u32, data: Vec<u8>,  // img
    x: i32, y: i32, w: i32, h: i32,          // rect
    noise_estimate: f32,
    sigma: f32,
) -> RegionOfInterestSummary {
    let image = GrayImage::from_raw(width, height, data).unwrap();
    let roi = &Rect::at(x, y).of_size(w as u32, h as u32);
    let summary = cedar_detect::algorithm::summarize_region_of_interest(&image, roi, noise_estimate, sigma);
    RegionOfInterestSummary::wrap(summary)
}


// Add this module to the main module
pub fn add_module(py: Python, parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let algorithm_mod = PyModule::new_bound(py, "algorithm")?;

    algorithm_mod.add_class::<StarDescription>()?;
    algorithm_mod.add_function(wrap_pyfunction!(estimate_noise_from_image, &algorithm_mod)?)?;
    algorithm_mod.add_function(wrap_pyfunction!(estimate_background_from_image_region, &algorithm_mod)?)?;
    algorithm_mod.add_function(wrap_pyfunction!(get_stars_from_image, &algorithm_mod)?)?;
    algorithm_mod.add_class::<RegionOfInterestSummary>()?;
    algorithm_mod.add_function(wrap_pyfunction!(summarize_region_of_interest, &algorithm_mod)?)?;

    parent.add_submodule(&algorithm_mod)?;

    Ok(())
}
