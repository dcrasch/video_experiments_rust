use rav1e::*;
use rav1e::data::FrameType;
use std::fs::File;
use muxide::api::{MuxerBuilder, VideoCodec, AudioCodec, Metadata, MuxerStats};
use yuvutils_rs::{
    YuvChromaSubsampling, YuvConversionMode, YuvPlanarImageMut, YuvRange, YuvStandardMatrix,
    rgb_to_yuv420,
};

fn main() {
    let img = image::open("input.png")
        .expect("Failed to open image")
        .to_rgb8();
    let enc = EncoderConfig {
        width: 1920,
        height: 1080,
        bit_depth: 8,

        chroma_sampling: rav1e::color::ChromaSampling::Cs420,
        ..Default::default()
    };
    let cfg = Config::new().with_encoder_config(enc.clone());
    let mut ctx: Context<u8> = cfg.new_context().unwrap();
    
    let width = 1920;
    let height = 1080;

    println!("convert image to yuv");
    let rgba_stride = width * 3;
    let mut planar_image =
        YuvPlanarImageMut::<u8>::alloc(width as u32, height as u32, YuvChromaSubsampling::Yuv420);
    let _ = rgb_to_yuv420(
        &mut planar_image,
        &img,
        rgba_stride as u32,
        YuvRange::Full,
        YuvStandardMatrix::Bt601,
        YuvConversionMode::Balanced,
    );

    println!("encode frame");
    for _ in 0..1 {
        let mut frame = ctx.new_frame();
        frame.planes[0].copy_from_raw_u8(&planar_image.y_plane.borrow(), width, 1);
        frame.planes[1].copy_from_raw_u8(&planar_image.u_plane.borrow(), width / 2, 1);
        frame.planes[2].copy_from_raw_u8(&planar_image.v_plane.borrow(), width / 2, 1);
        ctx.send_frame(frame.clone()).unwrap();
    }
    ctx.flush();
    println!("create output");
    
    let output = File::create("output.mp4").unwrap();
    let mut muxer = MuxerBuilder::new(output)
        .video(VideoCodec::Av1, 1920, 1080, 30.0)
        .with_metadata(Metadata::new().with_title("My Recording"))
        .with_fast_start(true)
        .build().unwrap();

    loop {
        match ctx.receive_packet() {
            Ok(packet) => {
	    	let timestamp = packet.input_frameno as f64 * (1.0/30.0); 
                println!("{}", packet.input_frameno);
                println!("{}", packet.frame_type);
		muxer.write_video(timestamp, &packet.data, match packet.frame_type {
		FrameType::KEY => true,
		_ => false
		}
		).unwrap();
            }
            Err(EncoderStatus::LimitReached) => {
                break;
            }
            Err(e) => {
                println!("{}", e.to_string());
            }
        }
    }
    
    let stats: MuxerStats = muxer.finish_with_stats().unwrap();
    println!("Video encoded successfully! 🦦");
    println!("{:?}",stats);
}
