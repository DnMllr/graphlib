use std::io::{stdout, BufWriter, Write};

use rand::{thread_rng, Rng};

/// The higher this number the more weighted towards close connections the graph generator will make
const CLOSENESS_PREFERENCE: f64 = 10.0;

pub fn print_graph(count: usize, probability: f64, parallel: bool) -> color_eyre::Result<()> {
    let result = if parallel {
        par_generate_lines(count, probability)
    } else {
        generate_lines(count, probability)
    };

    swallow_broken_pipe_errors(result)
}

fn generate_lines(count: usize, probability: f64) -> color_eyre::Result<()> {
    print_edges(edges(count, probability))
}

fn par_generate_lines(count: usize, probability: f64) -> color_eyre::Result<()> {
    let (tx, rx) = std::sync::mpsc::sync_channel(10000);
    let t = std::thread::spawn(move || {
        for edge in edges(count, probability) {
            tx.send(edge).expect("failed to send over channel");
        }
    });

    print_edges(rx.iter())?;

    t.join().unwrap();

    Ok(())
}

fn swallow_broken_pipe_errors(result: color_eyre::Result<()>) -> color_eyre::Result<()> {
    match result {
        Err(err) => match err.downcast_ref::<std::io::Error>() {
            Some(io_err) => match io_err.kind() {
                std::io::ErrorKind::BrokenPipe => Ok(()),
                _ => Err(err),
            },
            None => Err(err),
        },
        ok => ok,
    }
}

#[derive(Debug)]
struct Edge {
    pub from: usize,
    pub to: usize,
}

fn permutations<'a, T, I>(iter: I) -> impl Iterator<Item = (T, T)>
where
    T: Copy + PartialEq + 'a,
    I: Iterator<Item = T> + Clone + 'a,
{
    iter.clone()
        .flat_map(move |i| iter.clone().map(move |j| (i, j)).filter(|(a, b)| *a != *b))
}

fn edges(count: usize, probability: f64) -> impl Iterator<Item = Edge> {
    permutations(0..count)
        .zip(std::iter::from_fn(|| Some(thread_rng().gen::<f64>())))
        .filter(move |((f, t), r)| *r < calculate_probability(count, *f, *t, probability))
        .map(|((from, to), _)| Edge { from, to })
}

fn calculate_probability(count: usize, from: usize, to: usize, max_probability: f64) -> f64 {
    let diff = (from.abs_diff(to) as f64) / (count as f64);
    std::f64::consts::E.powf(-CLOSENESS_PREFERENCE * diff) * max_probability
}

fn print_edges<T: Iterator<Item = Edge>>(edges: T) -> color_eyre::Result<()> {
    let out = stdout();
    let mut writer = BufWriter::new(out.lock());
    let mut needs_space = false;
    let mut index = 0;
    let mut number_buf = itoa::Buffer::new();
    for edge in edges {
        while index < edge.from {
            index += 1;
            writer.write_all(b"\n")?;
            needs_space = false;
        }

        if needs_space {
            writer.write_all(b" ")?;
        }

        writer.write_all(number_buf.format(edge.to).as_bytes())?;

        needs_space = true;
    }

    writer.write_all(b"\n\n")?;
    writer.flush()?;

    Ok(())
}
