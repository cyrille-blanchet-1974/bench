use std::thread::spawn;
//use std::time::Duration;
use std::time::SystemTime;


/*
 1 millions de boucles
 1 add + 1 cmp pour boucle
 1 add pour op
 1 sub pour step
 1 cmp
=> par itération 2 add 2 cmp 1 cmp a 1 op/cycles
=> 5 millions de cycles 


chomebook
m3y8100 frq normale 1.1Ghz turbo 3.4Ghz
2 coeurs + HT
----------------------------------------
XPS17
core i7 2670QM  2.2Ghz turbo à 3.1
4 coeurs + HT
********************************************************************
1 thread (turbo max normalement)
----------------------------------------
CB => monte un peu a 3Ghz et 2.7Ghz la pluspart du temps
Thread 0-> 40.108364712s
All Threads -> 40.108628903s
----------------------------------------
XPS17 => 3.06Ghz tout du long
Thread 0-> 55.1589519s
All Threads -> 55.1619514s
----------------------------------------
CB plus rapide (72.7% du temps)
********************************************************************
2 threads 
----------------------------------------
CB => cpus a 2,7ghz
Thread 0-> 48.405346159s
Thread 1-> 48.814498652s
All Threads -> 48.816010449s
----------------------------------------
xps17 => 2.8Ghz
Thread 0-> 58.981869s
Thread 1-> 59.3266559s
All Threads -> 59.3316533s
----------------------------------------
CB plus rapide (82.2% du temps)
********************************************************************
4 threads 
----------------------------------------
CB =>  cpus à 2.2Ghz
Thread 3-> 96.401347831s
Thread 0-> 97.867943323s
Thread 2-> 98.167467206s
Thread 1-> 98.31584045s
All Threads -> 98.316695124s
----------------------------------------
XPS17 => 2.19Ghz
Thread 2-> 72.3784986s
Thread 1-> 72.7312846s
Thread 3-> 73.2489611s
Thread 0-> 76.541591s
All Threads -> 76.5495863s
----------------------------------------
CB plus lent (128.44% du temps)   de plus CB = 201.41% du temps de 2 threads 
plus de 2 fois plus lent pour le double de traitement
********************************************************************
8 threads 
----------------------------------------
CB => non lancé
----------------------------------------
xps17 => 2.19Ghz
Thread 2-> 129.4158752s
Thread 5-> 129.5938489s
Thread 6-> 129.6553475s
Thread 1-> 129.7082177s
Thread 4-> 131.8195778s
Thread 0-> 132.348947s
Thread 3-> 132.4709454s
Thread 7-> 132.5909433s
All Threads -> 132.6019321s
----------------------------------------
XPS17 = 173.24% du temps de 4 threads
73% plus lent pour le double de traitement
********************************************************************
*/

pub const THREAD_COUNT: i8 = 8;
pub const LOOP_COUNT: i64 = 1_000_000_000;//_000;
pub const STEPS: i64 = 25;

fn actions(threadnb : i8) {
    println!("Thread {}", threadnb);
    let start = SystemTime::now();
    let mut j=0;
    let mut space=String::new();
    space.push(' ');
    space.push(' ');
    for _i in 0..threadnb{
        space.push(' ');
        space.push(' ');
    }
    let mut step=LOOP_COUNT/STEPS;
    for i in 1..LOOP_COUNT {
        j = j+1;
        step = step-1;
        if step == 0 {
            step=LOOP_COUNT/STEPS;
            println!("{}{}{}", threadnb,space,i);
        }
    }
    let end = SystemTime::now();
    let tps = end
        .duration_since(start)
        .expect("ERROR computing duration!");
    println!("Thread {}-> {:?}", threadnb,tps);
}

fn main() {
    println!("Hello, world!");
    let mut threads = Vec::new();
    let start = SystemTime::now();
    for i in 0..THREAD_COUNT{
        println!("Start Thread N°{}",i);
        threads.push(spawn(move || {
            actions(i);
        }));
    }
    for t in threads{
        if t.join().is_err() {
            println!("1 Thread finished with error");
        }else
        {
            println!("1 Thread finished properly");
        }
    }
    let end = SystemTime::now();
    let tps = end
        .duration_since(start)
        .expect("ERROR computing duration!");
    println!("All Threads -> {:?}", tps);
}
