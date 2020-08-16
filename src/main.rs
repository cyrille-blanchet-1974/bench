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
Cpu fonctionne a 1.1Ghz
2 coeurs + HT
Turbo & 3,4Ghz

4 threads => cpus à 2.2Ghz
Thread 3-> 96.401347831s
Thread 0-> 97.867943323s
Thread 2-> 98.167467206s
Thread 1-> 98.31584045s
All Threads -> 98.316695124s

Semble très long (2,2 milliards d'op/secondes)

2 threads => cpus a 2,7ghz
Thread 0-> 48.405346159s
Thread 1-> 48.814498652s
All Threads -> 48.816010449s
2 fois plus rapide.println!

1 thread: monte à 3ghz pour au moins 1 et 2.7 pour les autres
Thread 0-> 40.108364712s
All Threads -> 40.108628903s
2°% plus rapide
*/

pub const THREAD_COUNT: i8 = 1;
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
