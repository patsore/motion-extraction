use image::{DynamicImage, EncodableLayout, ImageBuffer, Rgb};
use rayon::prelude::*;

fn main() {
    const NUM_IMG: u16 = 653;

    const OFFSET: u16 = 20;

    (1..NUM_IMG + 1).par_bridge().for_each(|i| {
        let mut image_source = image::open(&format!("frames/out-{:0>3}.png", i)).unwrap();
        let image_source = image_source.into_rgb8();

        let mut image_comp = image::open(&format!("frames/out-{:0>3}.png", i + OFFSET)).unwrap();
        image_comp.invert();
        let mut image_comp = image_comp.into_rgb8();

        let mut out_bytes = vec![0_u8; image_source.as_bytes().len()];

        image_source.as_bytes().par_iter()
            .zip(image_comp.as_bytes().par_iter())
            .zip(out_bytes.par_iter_mut())
            .for_each(|((src, comp), mut out)| {
                let avg = (src / 2 + comp / 2);
                if avg.abs_diff(128) > 5{
                    *out = (src / 2 + (255 - comp) / 2);
                }else{
                    *out = avg
                }
            });

        // image_source.as_bytes().chunks_exact(32)
        //     .zip(image_comp.as_bytes().chunks_exact(32))
        //     .zip(out_bytes.chunks_exact_mut(32))
        //     .for_each(|((src_chunk, comp_chunk), out_chunk)| {
        //
        //         use std::arch::x86_64::*;
        //         let src = unsafe { _mm256_loadu_si256(src_chunk.as_ptr() as *const __m256i) };
        //         let comp = unsafe { _mm256_loadu_si256(comp_chunk.as_ptr() as *const __m256i) };
        //         let result = unsafe { _mm256_avg_epu8(src, comp) };
        //         unsafe { _mm256_storeu_si256(out_chunk.as_mut_ptr() as *mut __m256i, result) };
        //
        //     });

        let out_image = ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(image_source.width(), image_source.height(), out_bytes).unwrap();

        out_image.save(&format!("frames-out/out-{:0>3}.png", i)).unwrap()
    });
}