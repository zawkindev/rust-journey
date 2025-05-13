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
}
