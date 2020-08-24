extern crate crossbeam;
use crossbeam::crossbeam_channel::bounded;
use std::sync::mpsc;
use std::thread;

fn main() {
    // メインの処理用
    let (sender, receiver) = bounded(4);
    // 確認用
    let (conf_sender, conf_receiver) = mpsc::channel();

    let producers = (0..2)
        .map(|_| {
            let s = sender.clone();
            thread::spawn(move || {
                for i in 0..20 {
                    s.send(i).unwrap();
                }
            })
        })
        .collect::<Vec<_>>();

    let consumers = (0..2)
        .into_iter()
        .map(|_| {
            let r = receiver.clone();
            let conf_s = conf_sender.clone();
            thread::spawn(move || {
                let mut sum = 0;
                for _ in 0..20 {
                    sum += r.recv().unwrap();
                }
                println!("sum={}", sum);
                conf_s.send(sum).unwrap();
            })
        })
        .collect::<Vec<_>>();

    producers.into_iter().for_each(|p| p.join().unwrap());
    consumers.into_iter().for_each(|c| c.join().unwrap());

    let sum: usize = (0..2)
        .map(|_| conf_receiver.recv().unwrap())
        .sum();

    println!("total sum: {}", sum);
}
