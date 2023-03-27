mod utils;

use std::f32::consts::PI;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct BilateralFilter {
    kernel_size: u8,
    half_kernel_size: u8,

    intensity_lut: [f32; 256],
    spatial_lut: [f32; 255],

    image_width: u32,
    image_height: u32,
    input_data: Vec<u8>,

    output_data: Vec<u8>,
}

#[wasm_bindgen]
impl BilateralFilter {

    pub fn input_data_ptr(&self) -> *const u8 {
        return self.input_data.as_ptr();
    }

    pub fn output_data_ptr(&self) -> *const u8 {
        return self.output_data.as_ptr();
    }

    pub fn new(image_width: u32, image_height: u32) -> BilateralFilter {

        utils::set_panic_hook();

        let input_data: Vec<u8> = (0..image_width*image_height).map(|_i| {0}).collect();
        let output_data: Vec<u8> = (0..image_width*image_height).map(|_i| {0}).collect();

        return BilateralFilter {
            kernel_size: 3,
            half_kernel_size: 1,
            intensity_lut: [0.0; 256],
            spatial_lut: [0.0; 255],
            image_width,
            image_height,
            input_data,
            output_data
        }
    }

    pub fn set_sigma(&mut self, spatial: u8, intensity: u8) {
        self.set_kernel_size(self.calculate_kernel_size(spatial));

        self.calculate_intensity_lut(intensity);
        self.calculate_gauss_spatial_lut(spatial);
    }

    pub fn run(&mut self) {

        let half_kernel_size = self.half_kernel_size as u32;

        let end_height = self.image_height - half_kernel_size;
        let end_width = self.image_width - half_kernel_size;

        let half_kernel_size_stride = half_kernel_size * self.image_width;

        let mut height = half_kernel_size * self.image_width;
        let mut central_pixel_index;
        let mut top_left_kernel_index;

        for _i in half_kernel_size..end_height {

            top_left_kernel_index = height - half_kernel_size_stride;

            for j in half_kernel_size..end_width {
                central_pixel_index = (height + j) as usize;
                
                self.output_data[central_pixel_index] = self.kernel(top_left_kernel_index + (j - half_kernel_size), self.input_data[central_pixel_index]);
            }

            height += self.image_width;
        }

    }

    fn kernel(&self, mut start_position: u32, central_pixel: u8) -> u8 {
        let mut sum_weight = 0.0;
        let mut normalize_weight = 0.0;
        let mut weight;
        let mut nearby_pixel;
        let mut counter = 0;

        for _i in 0..self.kernel_size {

            for j in 0..self.kernel_size as u32 {
                nearby_pixel = self.input_data[(start_position + j) as usize];

                weight = self.spatial_lut[counter] * self.intensity_lut[nearby_pixel.abs_diff(central_pixel) as usize];
                sum_weight += weight * nearby_pixel as f32;
                normalize_weight += weight;

                counter+=1;
            }

            start_position += self.image_width;
        }

        return (sum_weight / normalize_weight).round() as u8;

    }

    fn set_kernel_size(&mut self, value: u8) {
        self.kernel_size = value;
        self.half_kernel_size = ((value / 2) as f32).floor() as u8;
    }
    
    fn calculate_kernel_size(&self, spatial: u8) -> u8 {
        let kernel_size: u8 = (1.95 * spatial as f32).floor() as u8;
        
        let is_even: bool = kernel_size % 2 == 0;
    
        let odd_kernel_size: u8;
        
        if is_even {
            odd_kernel_size = kernel_size - 1;
        } else {
            odd_kernel_size = kernel_size;
        }
        
        return odd_kernel_size.max(3);
    }

    fn calculate_intensity_lut(&mut self, sigma: u8) {

        let intensity: f32 = 1.0 / ((2.0 * PI * sigma.pow(2) as f32));
        let intensity_square: i32 = 2 * sigma.pow(2) as i32;
    
        // * Calculate intensities functions for all of pixel intensity range, E.g. 0-255
        for i in 0..255 {
            self.intensity_lut[i] = intensity * f32::exp((-(i.pow(2) as i32 / intensity_square)) as f32);
        }
    }
    
    fn calculate_gauss_spatial_lut(&mut self, sigma: u8) {
        let spatial = 1.0 / ((2.0 * PI) * (sigma * sigma) as f32);
        let spatial_square = (2 * (sigma * sigma)) as i32;
    
        
        let mut counter = 0;
        /*
        * Fill look up table using geometrical 
        * distance between central pixel and nearby one within one kernel
        */
        for i in -(self.half_kernel_size as i8)..self.half_kernel_size as i8 {
            for j in -(self.half_kernel_size as i8)..self.half_kernel_size as i8 {
                self.spatial_lut[counter] = spatial * f32::exp(-(f32::powi(f32::hypot(i as f32, j as f32), 2)) / spatial_square as f32);
                counter += 1;
            }
        }
    }
}


#[cfg(test)]
mod test {
    use crate::BilateralFilter;

    const IMAGE_WIDTH: u32 = 6;
    const IMAGE_HEIGHT: u32 = 6;

    const DEFAULT_KERNEL_SIZE: u8 = 3;

    const SIGMA_SPATIAL: u8 = 3;
    const SIGMA_INTENSITY: u8 = 3;

    const DATASET: [u8; 36] = [
        205, 185, 193, 105, 135, 93,
        205, 189, 193, 115, 116, 13,
        215, 142, 124, 125, 181, 73,
        108, 185, 161, 135, 135, 83,
        65, 185, 53, 119, 135, 93,
        89, 185, 193, 105, 135, 93,
    ];

    fn get_new_filter() -> BilateralFilter {
        BilateralFilter::new(IMAGE_WIDTH, IMAGE_HEIGHT)
    }

    #[test]
    fn calculate_kernel_size_test() {
        let bf = get_new_filter();

        let kernel_size = bf.calculate_kernel_size(SIGMA_SPATIAL);

        assert_eq!(kernel_size, 5, "Kernel size = {}", kernel_size);
    }

    #[test]
    fn set_kernel_size_test() {
        let mut bf = get_new_filter();

        bf.set_kernel_size(DEFAULT_KERNEL_SIZE);

        assert_eq!(bf.kernel_size, 3, "Kernel size = {}", bf.kernel_size);
        assert_eq!(bf.half_kernel_size, 1, "Half kernel size = {}", bf.half_kernel_size);
    }

    #[test]
    fn calculate_intensity_lut_test() {
        let mut bf = get_new_filter();

        bf.calculate_intensity_lut(SIGMA_INTENSITY);

        assert_eq!(bf.intensity_lut[0], 0.017683882, "Intensity LUT's first value = {}", bf.intensity_lut[0]);
    }

    #[test]
    fn calculate_spatial_lut_test() {
        let mut bf = get_new_filter();

        bf.set_sigma(SIGMA_SPATIAL, SIGMA_INTENSITY);

        bf.calculate_gauss_spatial_lut(SIGMA_SPATIAL);

        assert_eq!(bf.spatial_lut[0], 0.011338559, "Spatial LUT's first value = {}", bf.spatial_lut[0]);
    }

    #[test]
    fn kernel_test() {
        let mut bf = get_new_filter();
        bf.set_sigma(SIGMA_SPATIAL, SIGMA_INTENSITY);
        bf.input_data = DATASET.to_vec();

        bf.calculate_intensity_lut(SIGMA_INTENSITY);
        bf.calculate_gauss_spatial_lut(SIGMA_SPATIAL);
 
        let pixel = bf.kernel(0, DATASET[(1*6) + 1]);

        assert_eq!(pixel, 190, "Pixel value: {}", pixel);
    }
}
