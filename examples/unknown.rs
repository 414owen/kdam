use kdam::Bar;

fn main() {
    let mut pb = Bar::default();

    for _ in 0..10000000 {
        pb.update(1);
    }
    pb.refresh();
}