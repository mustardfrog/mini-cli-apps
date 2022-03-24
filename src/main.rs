mod programs{ 
    pub mod grepit; 
    pub mod echo; 
    pub mod pussycat; 
} 

#[allow(unused_must_use)]
fn main() {
    programs::grepit::grepit();
    //programs::echo::echo();
    //programs::pussycat::cat();
}
