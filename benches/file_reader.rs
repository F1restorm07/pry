use pry::file_reader::index_file;


brunch::benches!(
    brunch::Bench::new("pry::file_reader::index_file(gutenburg)")
        .with_samples(1000)
        .run(|| index_file("gutenburg/12374.txt")),
    brunch::Bench::new("pry::file_reader::index_file(readme)")
        .with_samples(1000)
        .run(|| index_file("readme.md")),
                );
