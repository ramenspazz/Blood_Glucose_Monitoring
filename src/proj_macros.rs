#[macro_export]
macro_rules! readln {
    ( $x:ident ) => {{
        std::io::stdin()
            .read_line(&mut $x)
            .expect("Failed to read line");
    }};

    ( ) => {{
        let mut write_var = String::new();
        std::io::stdin()
            .read_line(&mut write_var)
            .expect("Failed to read line");
        write_var
    }};
}

#[macro_export]
macro_rules! readln_no_echo {
    ( $x:ident ) => {{
        // Incase string is uninitialized, do so. If not then meh, it didn't really hurt anything
        $x = std::default::Default::default();
        // Disable terminal echo
        if cfg!(unix) {
            let _ = std::process::Command::new("stty")
                .arg("-echo")
                .spawn()
                .expect("Failed to execute command");
        }
        std::io::stdin()
            .read_line(&mut $x)
            .expect("Failed to read line");

        // Re-enable terminal echo before exiting
        if cfg!(unix) {
            let _ = std::process::Command::new("stty")
                .arg("echo")
                .spawn()
                .expect("Failed to execute command");
        }
    }};

    ( ) => {{
        // Disable terminal echo
        if cfg!(unix) {
            let _ = std::process::Command::new("stty")
                .arg("-echo")
                .spawn()
                .expect("Failed to execute command");
        }
        let mut write_var = String::new();
        std::io::stdin()
            .read_line(&mut write_var)
            .expect("Failed to read line");
        // Re-enable terminal echo before exiting
        if cfg!(unix) {
            let _ = std::process::Command::new("stty")
                .arg("echo")
                .spawn()
                .expect("Failed to execute command");
        }
        write_var
    }};
}
