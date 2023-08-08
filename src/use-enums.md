# How to use Enums


```rust,editable
fn main() {
    pub struct Animal {
        pub name: String,
        legs: u32,
        pub(crate) wings: u32,
    }
    pub struct Dog {
        pub name: String,
        legs: u32,
    }
    pub enum AnimalTypes {
        Chicken(Animal, u32),
        Dog(Dog),
        Other(String),
        BadTypesystemUse((String, String, u32, String, String)),
    }

    let list = vec![
        AnimalTypes::Chicken(
            Animal {
                name: "Chicken".to_string(),
                legs: 2,
                wings: 2,
            },
            5,
        ),
        AnimalTypes::Chicken(
            Animal {
                name: "Chicken".to_string(),
                legs: 2,
                wings: 2,
            },
            3,
        ),
        AnimalTypes::Dog(Dog {
            name: "Dog".to_string(),
            legs: 4,
        }),
        AnimalTypes::Other("Other".to_string()),
    ];
    let res = list.iter().fold(0u32,
        |acc, x| match x {
            AnimalTypes::Chicken(.., count) => *count + acc,
            AnimalTypes::Dog(_) => acc,
            AnimalTypes::Other(_) => acc,
            AnimalTypes::BadTypesystemUse(_) => {
                panic!("Bad typesystem use")
            }
        }
    );

    println!("res: {}", res);

    let legs = list.iter().fold(0u32,
        |acc, x| match x {
            AnimalTypes::Chicken(Animal{ legs, ..}, ..) => *legs + acc,
            AnimalTypes::Dog(Dog { legs, ..}) => *legs + acc,
            AnimalTypes::Other(_) => acc,
            AnimalTypes::BadTypesystemUse(_) => {
                panic!("Bad typesystem use")
            }
        }
    );

    println!("legs: {}", legs);
}


```