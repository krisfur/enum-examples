use std::fmt;

#[derive(Clone, Copy)]
#[allow(dead_code)] // we only build `Running` below; the rest are for show
enum Status {
    Pending,
    Running,
    Succeeded,
    Failed,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Status::Pending => "pending",
            Status::Running => "running",
            Status::Succeeded => "succeeded",
            Status::Failed => "failed",
        };
        f.write_str(name)
    }
}

// This is the same idea as the C union + tag, made safe and built in.
enum Media {
    Text {
        length: u32,
    },
    Image {
        width: u32,
        height: u32,
    },
    Video {
        width: u32,
        height: u32,
        duration: f64,
    },
}

// `match` destructures the variant and must be exhaustive
fn describe(m: &Media) -> String {
    match m {
        Media::Text { length } => format!("text: {length} chars"),
        Media::Image { width, height } => format!("image: {width}x{height}"),
        Media::Video {
            width,
            height,
            duration,
        } => {
            format!("video: {width}x{height}, {duration}s")
        }
    }
}

fn main() {
    let s = Status::Running;
    println!("status: {s} ({})", s as u32);

    let items = [
        Media::Text { length: 280 },
        Media::Image {
            width: 1920,
            height: 1080,
        },
        Media::Video {
            width: 1920,
            height: 1080,
            duration: 12.5,
        },
    ];
    for m in &items {
        println!("{}", describe(m));
    }
}
