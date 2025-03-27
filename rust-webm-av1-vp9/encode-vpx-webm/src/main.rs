use std::fs::File;
use std::io::Write;
use yuvutils_rs::{rgb_to_yuv420, YuvChromaSubsampling, YuvConversionMode, YuvPlanarImageMut, YuvRange, YuvStandardMatrix};
use webm::mux::{SegmentBuilder, VideoCodecId, Writer};
fn main() {
    // load image and convert to yuv420
    let img = image::open("input.png").expect("Failed to open image").to_rgb8();   
    let width = 1920;
    let height = 1080;   
    let rgba_stride = width*3;
    let mut planar_image =YuvPlanarImageMut::<u8>::alloc(width as u32, height as u32, YuvChromaSubsampling::Yuv420);
    let _ = rgb_to_yuv420(
	&mut planar_image,
	&img,
	rgba_stride as u32,
	YuvRange::Full,
	YuvStandardMatrix::Bt601,
	YuvConversionMode::Balanced
    );


    let mut data = Vec::new();
    data.extend(planar_image.y_plane.borrow());
    data.extend(planar_image.u_plane.borrow());
    data.extend(planar_image.v_plane.borrow());

    let mut vpx = vpx_encode::Encoder::new(vpx_encode::Config {
        width: width,
        height: height,
        timebase: [1, 1000],
        bitrate: 5000,
        codec: vpx_encode::VideoCodecId::VP9,
    }).unwrap();

    println!("start creating webm");
    // Open the output WebM file
    let output = File::create("output.webm").unwrap();
    let writer: Writer<File> = Writer::new(output);

    // Initialize the WebM segment
    let builder = SegmentBuilder::new(writer).unwrap();
    let (builder, video_track) = builder
        .set_writing_app("RUST").unwrap()
        .add_video_track(1920,1080,
             VideoCodecId::VP9, 
			 Some(1)).unwrap();
    let timestamp_ns = 0;
    let timestamp_ms : i64 = 0;
    let is_keyframe = true;
    
    let mut output_obu = File::create("output.obu").unwrap();
    let mut segment = builder.build();
    let _packets = vpx.encode(timestamp_ms, &data).unwrap();
    let mut frames = vpx.finish().unwrap();
    let frame = frames.next().unwrap().unwrap();
    output_obu.write_all(&frame.data).unwrap();
    segment.add_frame(video_track, frame.data, timestamp_ns, is_keyframe).unwrap();
    _ = segment.finalize(None).inspect_err(|_| eprintln!("could not finalize"));

    println!("Video encoded successfully! 🦦");
}
