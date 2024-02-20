#![allow(unused)]
use tracing::{debug, error, info, info_span, instrument, span, trace, warn, Level};
use tracing_subscriber::filter::EnvFilter;

#[derive(Debug)]
struct Foo {
    a: bool,
    b: u32,
}

fn main() {
    init_logging();

    let span = span!(Level::INFO, "main");
    let _guard = span.enter();

    for thread in std::env::args()
        .map(|arg| std::thread::spawn(move || on_thread(arg)))
        .collect::<Vec<_>>()
    {
        thread.join().unwrap()
    }
}

#[instrument]
fn on_thread(file: String) {
    let span = span!(Level::INFO, "file", fname = %file);
    let _guard = span.enter();

    warn!("Opening the file {file}");
    trace!("Reading file contents");
    info!(bytes.number = 0, "Parsing");

    let f = Foo { a: true, b: 47 };
    warn!(parsed = ?f, "Done with file");
}

fn init_logging() {
    tracing_subscriber::fmt()
        .event_format(
            tracing_subscriber::fmt::format()
                .with_source_location(true)
                .without_time()
                .compact(),
        )
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    info!("Logging initialized!");
}

/*let mut join_handles = Vec::new();
for file in std::env::args() {
    join_handles.push(std::thread::spawn(move || {
        on_thread(file);
    }));
}
for handle in join_handles {
    handle.join().unwrap();
}*/

/*std::env::args()
.fold(Vec::new(), |mut join_handles, file| {
    join_handles.push(std::thread::spawn(move || {
        on_thread(file);
    }));
    join_handles
})
.into_iter()
.for_each(|handle| handle.join().unwrap());*/

/*for handle in std::env::args().fold(Vec::new(), |mut handles, file| {
    handles.push(std::thread::spawn(move || {
        on_thread(file);
    }));
    handles
}) {
    handle.join().unwrap()
}*/
