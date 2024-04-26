pub trait Summary {
    fn summarize(&self) -> String;
  }
  pub struct Tweet {
    pub red: String,
    pub yellow: String,
    pub green: String,
  } 
   impl Summary for Tweet {
    fn summarize(&self) -> String{
        format!("Red light {} second,yellow light {} second,green light {} second", self.red, self.yellow, self.green)
    }
  }
  pub fn notify<T:Summary>(item:&T){
    println!("{}",item.summarize());
  }
  fn main() {
    let time = Tweet{
      red: String::from("40"),
      yellow: String::from("3"),
      green: String::from("37"),
    };
    notify(&time);
  }