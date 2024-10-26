use std::{usize, vec};
use rand::{seq::SliceRandom, thread_rng, Rng};

#[derive(Debug)]
struct Deste {
 kartlar : Vec<String>,
}

impl Deste{

    fn new()-> Self {
        let türler = ["sinek","kupa","macha","karo"];
    let sayılar = ["birli","ikili","üçlü","dörtlü","beşli","altılı","yedili","sekizli","dokuzlu","onlu","vale","kız","kral","AS"];
    let mut kartlar= vec![]; 
    for tür in türler{
        for sayı in sayılar  {
            let kart=format!("{} {}",tür,sayı);
            kartlar.push(kart);
            
        }
    }
    let deste= Deste { kartlar};
    return deste;
        
    }


    fn shuffle(&mut self){
        let mut rng = thread_rng();
        self.kartlar.shuffle(&mut rng);
    }
    fn dağıtma (&mut self,kart_numaraları: usize ) -> Vec<String>{

        self.kartlar.split_off(self.kartlar.len()-kart_numaraları)
    }


}



fn main() {
    let mut deste = Deste::new();
    deste.shuffle();
   let kartlar =deste.dağıtma(5) ;
   let el1 = deste.dağıtma(5);
   let el2 = deste.dağıtma(5);
   let el3 = deste.dağıtma(5);
   let el4 = deste.dağıtma(5);

    println!(" Eliniz : {:#?}",el1);
    println!(" Eliniz : {:#?}",el2);
    println!(" Eliniz : {:#?}",el3);
    println!(" Eliniz : {:#?}",el4);



}
