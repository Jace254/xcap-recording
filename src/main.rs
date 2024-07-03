mod convert;

use std::fs::{OpenOptions, File};
use std::path::PathBuf;
use std::time::{Instant, Duration};
use std::sync::atomic::{AtomicBool, Ordering};
use std::{io, thread};
use std::sync::Arc;
use xcap::Monitor;
use webm::mux;
use webm::mux::Track;
use vpx_encode;

fn main() -> io::Result<()> {
    //  Get the screen
    let monitors = Monitor::all().unwrap();

    let monitor = if monitors.is_empty() {
        println!("No monitor displays found");
        return Ok(());
    } else {
        monitors.into_iter().nth(0).unwrap()
    };

    let width = monitor.width();
    let height = monitor.height();

    let path: PathBuf = "video.webm".into();

    // create an output file
    let out = match {
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
    } {
        Ok(file) => file,
        Err(ref e) if e.kind() == io::ErrorKind::AlreadyExists => {
            File::create(path)?
        },
        Err(e) => return Err(e.into())
    };
    // create the multiplexer
    let mut webm = mux::Segment::new(mux::Writer::new(out)).expect("Could not init multiplexer");
    let (vpx_codec, mux_codec) = (vpx_encode::VideoCodecId::VP8, mux::VideoCodecId::VP8);

    let mut vt = webm.add_video_track(width, height, None, mux_codec);

    let mut vpx = vpx_encode::Encoder::new(vpx_encode::Config {
        width,
        height,
        timebase: [1, 1000],
        bitrate: 6000,
        codec: vpx_codec
    }).unwrap();
    // capture screen for 60 secs

    let start = Instant::now();
    let stop = Arc::new(AtomicBool::new(false));

    thread::spawn({
        let stop = stop.clone();
        move || {
            println!("Recording! Press ⏎ to stop.");
            let _ = std::io::stdin().read_line(&mut String::new());
            stop.store(true, Ordering::Release);
        }
    });

    let seconds_per_frame = Duration::from_nanos(1_000_000_000 / 30); // 0.03333333333333333 seconds
    let duration: Option<u64> = Some(60);
    let duration = duration.map(Duration::from_secs);
    let mut yuv = Vec::new();

    while !stop.load(Ordering::Acquire) {
        let now = Instant::now();
        let time = now - start;

        if Some(true) ==  duration.map(|d| time > d) {
            break;
        }

        match monitor.capture_image() {
            Ok(frame) => {
                let ms = time.as_secs() * 1000 + time.subsec_millis() as u64;

                convert::argb_to_i420(&frame, &mut yuv);

                for frame in vpx.encode(ms as i64, &yuv).unwrap() {
                    vt.add_frame(frame.data, frame.pts as u64 * 1_000_000, frame.key);
                }
            },
            Err(e) => {
                println!("{}", e);
                break;
            }
        }

        let dt = now.elapsed();
        if dt < seconds_per_frame {
            thread::sleep(seconds_per_frame - dt);
        }
    }

    // End things.
    let mut frames = vpx.finish().unwrap();
    while let Some(frame) = frames.next().unwrap() {
        vt.add_frame(frame.data, frame.pts as u64 * 1_000_000, frame.key);
    }

    let _ = webm.finalize(None);
    

    Ok(())
}