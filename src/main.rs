use std::io;
use std::thread::spawn;
use std::time::SystemTime;

/*
 250 milliard de boucles
 1 add + 1 cmp pour boucle
 1 add pour op
 1 sub pour step
 1 cmp
=> par itération 2 add 2 cmp 1 sub a 1 op/cycles
=> 2 500 milliard de cycles
=> 2 500 secondes à 1Ghz ?

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
nv pc HP => i7 8665U
  https://ark.intel.com/content/www/fr/fr/ark/products/193563/intel-core-i7-8665u-processor-8m-cache-up-to-4-80-ghz.html
  4c/8T 1.9Ghz base 4.8Ghz turbo (1coeur?) 14nm Q2 2019 8Mo cache 25 watt TDP UHD620 1.15Ghz 
DELL G15 => i7 11800H
   https://ark.intel.com/content/www/fr/fr/ark/products/213803/intel-core-i7-11800h-processor-24m-cache-up-to-4-60-ghz.html
   8c/16t 2,3Ghz base 4,6Ghz turbo 10nm Q2 2021 24Mo cache 45 watt TDP UHD 1,45Ghz
  ----------------------------------------
théorie
1 Thread pour 250 milliards d'op
2 Thread pour 125 milliards d'op
4 Thread pour 62.5 milliards d'op -> 62.5s@1Ghz -> 31.25s @ 2Ghz (CB)
8 Thread pour 31.25 milliards d'op -> 31.25s @ 1Ghz -> 12.6s @2.2Ghz (XPS) 11.16s@2.8Ghz 9.76s @3.2Ghz
-------------------------------------------------------------------------------
XPS17-L702/i7 2670QM
1 Thread -> 164.8552775s (3.05Ghz)
2 Thread -> 85.6135831s (2.95Ghz)
4 Thread -> 48.9635876s (2.8Ghz tout du long)
8 Thread -> 45.1763295s (2.8Ghz tout du long)
-------------------------------------------------------------------------------
chromebook/m3 8100Y
1 Thread -> 87.532552053s 3Ghz puis baisse jusquà 2,7Ghz        1.88*xps
2 Thread -> 52.41956528s 2.7Ghz en fin                          1.63*xps
4 Thread -> 49.257671545s (2.7Ghz)                              0.99*xps
8 Thread -> 49.681403841s (2.7ghz)                              0.90*xps
-------------------------------------------------------------------------------
HP zbook/i7-6820HK
1 Thread -> 71.026 3.6G 71.11                                   2.32*xps
2 Thread -> 37.997 3.4G 37.92                                   2.25*xps
4 Thread -> 22.460 3.2G 23.50                                   2.18*xps
8 Thread -> 22.586 3.2G 23.58                                   1.86*xps
-------------------------------------------------------------------------------
HP 840G6/i7 8665U
1 thread : 65.7163404s 4.2Ghz puis 3.8                          2.5*xps
2 threads: 40.644553s  3.7-3.6Ghz                               2.10*xps 
4 threads: 27.4814645s 4.2 puis 3Ghz                            1.78*xps     
8 threads: 24.3146738s 4.2 puis 3Ghz                            1.85*xps
--------------------------------------------------------------------------------
DELL G15/I7 11800h
 1 thread : 54.7855278s 4,45 à 4.47Ghz puis 3.8                  3.00*xps
 2 threads: 27.9650044s 4,42 à 4,44 Ghz                          3.06*xps 
 4 threads: 14.1322422s 4.44 à 4.46 Ghz                          3,46*xps     
 8 threads: 7.9422156s  4.20 à 4,21 Ghz                           5,68*xps
16 threads: 7,52733244s 4.19 Ghz                                  6,00*xps
--------------------------------------------------------------------------------
CPU           1T        2T       4T       8T    16T
i7 2870QM    164.85    85.61    48.96   45.17    ?
m3 8100Y      87.53    52.41    49.25   49.68    ?
i7 6820HK     71.02    37.99    22.46   22.58    ?
i7 8665U      65.71    40.64    27.48   24.31    ?
i7 11800H     54.78    27.96    14.13   7.94     7.52
---------------------------------------------------------------------------------
*/

//pub const THREAD_COUNT: i64 = 1;
pub const LOOP_COUNT: i64 = 250_000_000_000;
pub const STEPS: i64 = 10;

fn actions(thread: i64, nb_threads: i64) {
    println!("Thread {} start", thread);
    let start = SystemTime::now();
    let mut _j = 0;
    let mut loop_by_thread = LOOP_COUNT / nb_threads;
    let loop_by_step = loop_by_thread / STEPS;
    let mut step_count = loop_by_step;
    let mut step_id = 1;
    loop_by_thread += 1;
    for _i in 1..loop_by_thread + 1 {
        _j += 1;
        step_count -= 1;
        if step_count == 0 {
            println!("Thread {} Step {}/{}", thread, step_id, STEPS);
            step_id += 1;
            step_count = loop_by_step;
        }
    }
    let end = SystemTime::now();
    let tps = end
        .duration_since(start)
        .expect("ERROR computing duration!");
    println!("Thread {}-> {:?}", thread, tps);
}

//ask the user and read his answer
fn read_i64(mess: String) -> Option<i64> {
    println!("{}", mess);
    let mut res = String::new();
    io::stdin()
        .read_line(&mut res)
        .expect("Failed to read line");
    let res = res.trim();

    let r: i64 = match res.parse() {
        Err(e) => {
            println!("erreur {}", e);
            return None;
        }
        Ok(v) => v,
    };
    Some(r)
}

fn traitement(nb_threads: i64) {
    let mut threads = Vec::new();
    let start = SystemTime::now();
    for i in 0..nb_threads {
        threads.push(spawn(move || {
            actions(i, nb_threads);
        }));
    }
    for t in threads {
        if t.join().is_err() {
            println!("1 Thread finished with error");
        }
    }
    let end = SystemTime::now();
    let tps = end
        .duration_since(start)
        .expect("ERROR computing duration!");
    println!("Total -> {:?}", tps);
}

fn main() {
    println!("Little Benchmark");

    println!("Nombre de threads pour traiter les {} boucles (les boucles seront partagées entre les threads:",LOOP_COUNT);
    match read_i64("Your choice?".to_string()) {
        None => {}
        Some(x) => {
            traitement(x);
        }
    }
}
