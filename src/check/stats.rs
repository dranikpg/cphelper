use console::style;

struct AvgCounter{
    sum: u64,
    n: u64
}

impl AvgCounter{
    fn supply(&mut self, val: u64){
        self.sum += val;
        self.n += 1;
    }
    fn median(&self) -> f64{
        (self.sum as f64) / (self.n as f64)
    }
}

pub struct Stats{
    diffc: u64, 
    bavg: AvgCounter,
    savg: AvgCounter
}

impl Stats{
    pub fn raw() -> Self{
        Stats{
            diffc: 0,
            bavg: AvgCounter{sum: 0, n: 0},
            savg: AvgCounter{sum: 0, n: 0}
        }
    }
    pub fn report(&mut self, failed: bool, tbrute: u64, tsolve: u64){
        self.diffc += if failed {1} else {0};
        self.bavg.supply(tbrute);
        self.savg.supply(tsolve);
    }
}

impl std::fmt::Display for Stats {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.diffc{
            0 => write!(f, "{} test passed!", style("ALL").green()),
            _ => write!(f, "{} tests {}!", style(&format!("{}", self.diffc)).bold(), style("failed").red())
        }.ok();
        write!(f,"\nAvg time brute: {:.2} ms, solve: {:.2} ms", self.bavg.median(), self.savg.median()).ok();
        write!(f,"  -> {:.2} times faster",(self.bavg.median()/self.savg.median()))
    }
}