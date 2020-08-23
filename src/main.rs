use std::thread::spawn;
//use std::time::Duration;
use std::time::SystemTime;


/*
 1000 milliard de boucles
 1 add + 1 cmp pour boucle
 1 add pour op
 1 sub pour step
 1 cmp
=> par itération 2 add 2 cmp 1 cmp a 1 op/cycles
=> 5000 milliard de cycles 
=> 5 000 secondes à 1Ghz ?

Machines de tests:
ASUS chomebook => m3 8100Y 
  https://ark.intel.com/content/www/fr/fr/ark/products/185282/intel-core-m3-8100y-processor-4m-cache-up-to-3-40-ghz.html 
  2C/4T 1.1Ghz Base 3.4Ghz turbo (1 coeur ?) 14nm Q3 2018 4Mo de cache 4 watt TDP UHD 615 900Mhz
DELL XPS17 => i7 2670QGM 
  https://ark.intel.com/content/www/fr/fr/ark/products/53469/intel-core-i7-2670qm-processor-6m-cache-up-to-3-10-ghz.html 
  4C/8T 2.2Ghz Base 3.1ghz turbo (1 coeur ?) 32nm Q4 2011 6Mo de cache 45 watt TDP  HD 3000 1100Mhz
HP zbook => i7-6820HK
  https://ark.intel.com/content/www/fr/fr/ark/products/88970/intel-core-i7-6820hq-processor-8m-cache-up-to-3-60-ghz.html 
  4C/8T 2.7Ghz Base 3.6Ghz turbo (1 coeur ?) 14nm Q3 2015 8Mo de cache 45 watt TDP HD 530 1050Mhz
----------------------------------------
théorie
1 Thread pour 5 000 milliards d'op
2 Thread pour 2 500 milliards d'op
4 Thread pour 1 250 milliards d'op -> 1250s@1Ghz -> 625s @ 2Ghz (CB)
8 Thread pour 625 milliards d'op -> 625s @ 1Ghz -> 284s @2.2Ghz (XPS)
8 Thread pour 625 milliards d'op -> 625s @ 1Ghz -> 195s @3.2Ghz (HP)
----------------------------------------
chromebook/m3
1 Thread -> 404.575073961s (2.7Ghz tout du long)
2 Thread ->
4 Thread -> 241.076748227s (2.7Ghz-> 2.0)  => 2.56 ins/cycle
8 Thread -> 261.777100944s (2.7Ghz-> 2.0)   8T plus lent car cpu a 4T seulement
----------------------------------------
XPX/i7
1 Thread -> 
2 Thread -> 342.6585474s (2.98/2.97->2.96->2.95Ghz)
4 Thread -> 207.5616935s (2.8->2->3 sur la fin)
8 Thread -> 218.8510899ss (2.8Ghz-> 2.2Ghz) => 1.3 IPC
----------------------------------------
HP/i7
1 Thread -> 
2 Thread -> 373.3341977s (3.3Ghz)
4 Thread -> 216.979901s (3.2GHz)
8 Thread -> 168.1269106s (3.2Ghz) => 1.15 IPC
----------------------------------------
*/

pub const THREAD_COUNT: i64 = 1;
pub const LOOP_COUNT: i64 = 1_000_000_000_000;
pub const STEPS: i64 = 25;

fn actions(threadnb : i64) {
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
    for i in 1..(LOOP_COUNT/THREAD_COUNT) {
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
