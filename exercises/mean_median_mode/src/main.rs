use std::collections::HashMap;

fn main(){
    println!("\n --- The Mean, Median and Mode of List --- \n");
    
    let my_list = ListOfIntegers::new(vec![1,2,3,4,5,2,10,10,9,8,7,4,2]);
    println!("\nVector: {:?}\n", my_list.list);

    println!("Mean: {}\n", my_list.mean());
    println!("Median: {}\n", my_list.median());
    println!("Mode: {}\n", my_list.mode());


}

//Struct ListOfIntegers:
#[derive(Debug)]
struct ListOfIntegers {
    list: Vec<i32>,
}

impl ListOfIntegers{
    //associated function that returns a ListOfIntegers instance with list passed
    fn new(list: Vec<i32>) -> ListOfIntegers{
        ListOfIntegers {
            list,
        }
    }

    fn mean(&self) -> f64 {
        let sum: i32 = Iterator::sum(self.list.iter());
        f64::from(sum) / (self.list.len() as f64)
    }

    fn median(&self) -> f64 {
        let len = self.list.len();
        let mid = len / 2;
        
        if len % 2 == 0 {
            let sum: i32 = Iterator::sum(self.list[(mid-1)..(mid+1)].iter());
            f64::from(sum) / 2.0
        }else{
            f64::from(self.list[mid])
        } 
    }

    fn mode(&self) -> i32{
        let mut map = HashMap::new();
        let mut max: (i32, i32) = (0,0);

        for num in self.list.iter() {
            let count = map.entry(num).or_insert(0);
            *count +=1;
        }
       
        for (&&key, &val) in map.iter() {
            if val > max.1 {
                max = (key, val);
            }
        }

        max.0
    }

}

