struct Person<T> {
    value: T,
}

fn GenericPerson() {
    let p1: Person<i32> = Person { value: 100 };
    println!("p1.value is {}", p1.value);

    let p2: Person<f64> = Person { value: 66.00 };
    println!("p2.value is {}", p2.value)
}

trait person_trait {
    fn getValue(&self);
}

impl person_trait for Person<i32> {
    fn getValue(&self) {
        println!("person getvalue is {}",self.value);
    }
}

fn main() {
    GenericPerson();
    let p:Person<i32>=Person{value:666};
    p.getValue();
}