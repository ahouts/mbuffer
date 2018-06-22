
use std::io::{Read, Write};
use std::sync::mpsc::sync_channel;
use std::thread;

struct Segment {
    data: Vec<u8>,
    bytes: usize,
}

pub struct Buffer{
    r: Option<thread::JoinHandle<()>>,
    w: Option<thread::JoinHandle<()>>,
}

impl Buffer{
    pub fn new(segment_length: usize, buffer_size: usize, mut r: Box<Read + Send>, mut w: Box<Write + Send>) -> Self {
        let num_segments = (buffer_size as f32 / segment_length as f32).ceil() as usize;
        let (sender, receiver) = sync_channel::<Segment>(num_segments);

        let read_thread = thread::spawn(move|| {
            let r = r.as_mut();
            loop {
                let mut seg = Segment{
                    data: vec![0; segment_length],
                    bytes: 0,
                };
                let num_bytes = match r.read(seg.data.as_mut_slice()) {
                    Ok(0) => return,
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("error while reading from input: {}", e);
                        return;
                    }
                };
                seg.bytes = num_bytes;
                if let Err(e) = sender.send(seg) {
                    eprintln!("error while moving bytes into buffer: {}", e);
                    return;
                };
            }
        });

        let write_thread = thread::spawn(move|| {
            let w = w.as_mut();
            loop {
                let seg = match receiver.recv() {
                    Ok(s) => s,
                    Err(_) => {
                        // channel closed, we are done
                        return;
                    },
                };
                let data_ref = seg.data.as_slice();
                let data_ref = &data_ref[0..seg.bytes];
                if let Err(e) = w.write(data_ref) {
                    eprintln!("error while writing bytes: {}", e);
                    return;
                };
            }
        });

        Buffer{
            r: Some(read_thread),
            w: Some(write_thread),
        }
    }

    pub fn join(&mut self) -> Result<(), String> {
        if let Some(x) = self.r.take() {
            if let Err(e) = x.join() {
                return Err(format!("{:?}",  e.as_ref()));
            }
        };
        if let Some(x) = self.w.take() {
            if let Err(e) = x.join() {
                return Err(format!("{:?}",  e.as_ref()));
            }
        };
        Ok(())
    }
}
