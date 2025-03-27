use rav1e::*;
use std::fs::File;
use webm::mux::{SegmentBuilder, VideoCodecId, Writer};
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
    println!("collect packets");
    // Collect encoded packets
    let mut av1_packets = Vec::new();
    loop {
        match ctx.receive_packet() {
            Ok(packet) => {
                println!("{}", packet.input_frameno);
                println!("{}", packet.frame_type);
                av1_packets.push(packet);
            }
            Err(EncoderStatus::LimitReached) => {
                break;
            }
            Err(e) => {
                println!("{}", e.to_string());
            }
        }
    }

    println!("start creating webm");
    // Open the output WebM file
    let output = File::create("output.webm").unwrap();
    let writer: Writer<File> = Writer::new(output);

    // ebml https://github.com/ietf-wg-cellar/matroska-specification/blob/master/codec/av1.md  // Initialize the WebM segment
    let builder = SegmentBuilder::new(writer).unwrap();
    let (builder, video_track) = builder
        // Add a video track for AV1
        .set_writing_app("RUST")
        .unwrap()
        .add_video_track(1920, 1080, VideoCodecId::AV1, Some(1))
        .unwrap();
    let builder = builder.set_codec_private(video_track, &[0]).unwrap();

    let mut segment = builder.build();
    // Write the encoded AV1 packets to the WebM file
    //for pkt in av1_packets {
    let pkt = &av1_packets[0];
    //dbg!(&pkt);
    let timestamp_ns = 0;
    let is_keyframe = true;
    segment
        .add_frame(video_track, &pkt.data, timestamp_ns, is_keyframe)
        .unwrap();
    //}
    println!("finalize");
    // Finalize the segment
    _ = segment
        .finalize(None)
        .inspect_err(|_| eprintln!("could not finalize"));

    println!("Video encoded successfully! 🦦");
}
