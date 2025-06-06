fn main() {
    enum OS {
        Linux,
        Macos,
        Windows,
    }

    let linux = OS::Linux;
    let macos = OS::Macos;
    let windows = OS::Windows;

    fn boot(os: OS) {
        match os {
            OS::Linux => println!("grub"),
            OS::Macos => println!("opencore"),
            OS::Windows => println!("windows boot manager"),
        }
    }

    boot(linux);
    boot(macos);
    boot(windows);

    enum Woman {
        Wife(String),
        Waifu { name: String, age: u8 },
        Girl,
    }

    let tya = Woman::Wife(String::from("Tya"));

    let asuna = Woman::Waifu {
        name: String::from("Asuna"),
        age: 18,
    };

    let random_girl = Woman::Girl;

    impl Woman {
        fn name(&self) -> String {
            match self {
                Woman::Wife(name) => name.clone(),
                Woman::Waifu { name, .. } => name.clone(),
                Woman::Girl => "Some Random Girl".to_string(),
            }
        }

        fn age(&self) -> u8 {
            match self {
                Woman::Waifu { age, .. } => *age,
                _ => 0,
            }
        }
    }

    println!("{} is {} years old", asuna.name(), asuna.age());
    println!("My wife's name is {}! ", tya.name());
    println!("Some {} passed by me yesterday", random_girl.name());
}
