#[macro_export]
macro_rules! getln {

    ( $x:expr ) => {
        {
            std::io::stdin()
                .read_line(&mut $x)
                .expect("Failed to read line");
        }
    };
    
    ( ) => {
        {
            let mut write_var = String::new();
            std::io::stdin()
                .read_line(&mut write_var)
                .expect("Failed to read line");
            write_var
        }
    };
}

#[macro_export]
macro_rules! readch {

    ( $in_str:expr , $file:expr, $display:expr ) => {
        {
            if let Ok() = $file.read(in_str) {
                
            }
            $in_str
        }
    };
}

#[macro_export]
macro_rules! getln_no_echo {

    ( $x:expr ) => {
        {
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
            $x
            // Re-enable terminal echo before exiting
            if cfg!(unix) {
                let _ = Command::new("stty")
                    .arg("echo")
                    .spawn()
                    .expect("Failed to execute command");
            }
        }
    };
    
    ( ) => {
        {
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
        }
    };
}