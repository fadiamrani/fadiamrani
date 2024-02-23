use std::collections::HashMap;
use repl_rs::{Convert, Value};
use lazy_static::lazy_static;
use pingz::basic;
use regex::Regex;

pub fn add<T>(args: HashMap<String, Value>, _context: &mut T) -> repl_rs::Result<Option<String>> {
    let first: i32 = args["first"].convert()?;
    let second: i32 = args["second"].convert()?;

    Ok(Some((first + second).to_string()))
}

// Write "Hello"
pub fn hello<T>(args: HashMap<String, Value>, _context: &mut T) -> repl_rs::Result<Option<String>> {
    Ok(Some(format!("Hello, {}", args["who"])))
}



pub fn pp<T>(args: HashMap<String, Value>, _context: &mut T) -> () {
   println!("eeeeeeeeeeee")
}

lazy_static! {
    pub static ref REE: Regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    pub static ref FEE: Regex = Regex::new(r"(.*)\s+(.*)\s+(.*)\s+(.*)\s+(.*)$").unwrap();
    pub static ref LINE: Regex = Regex::new(r"\n").unwrap();
    pub static ref MY_STRING: String = "\
Filesystem                 Size  Used Avail Use% Mounted on
devtmpfs                    63G     0   63G   0% /dev
tmpfs                       63G     0   63G   0% /dev/shm
tmpfs                       63G  9.4M   63G   1% /run
tmpfs                       63G     0   63G   0% /sys/fs/cgroup
/dev/mapper/vg00-lv00     1014M   72M  943M   8% /
/dev/mapper/vg00-lvusr     5.0G  2.8G  2.3G  55% /usr
/dev/mapper/vg00-lvopt     5.0G  374M  4.7G   8% /opt
/dev/mapper/vg00-lvtmp     4.0G   62M  4.0G   2% /tmp
/dev/sda2                  2.0G  235M  1.8G  12% /boot
/dev/mapper/vg00-lvvar     5.0G  337M  4.7G   7% /var
/dev/mapper/vg00-lvvarlog  5.0G   88M  5.0G   2% /var/log
/dev/sda1                 1022M  5.8M 1017M   1% /boot/efi
/dev/mapper/vg01-lvdstpol  5.0G   76M  5.0G   2% /dstpol
/dev/mapper/vg01-lvhome   1004M   40M  965M   4% /home
/dev/mapper/vg01-lvswl     2.0G   47M  2.0G   3% /dstpol/swl
/dev/mapper/vg01-lvbackup 1004M   40M  965M   4% /dstpol/backup
/dev/mapper/vg01-lvsys     2.0G   47M  2.0G   3% /dstpol/sys
/dev/mapper/vg01-lvkvm     402G  303G  100G  76% /dstpol/kvm
tmpfs                       13G     0   13G   0% /run/user/0
    ".to_string();
}


pub fn df<T>(args: HashMap<String, Value>, _context: &mut T) {
    let lines: Vec<&str>  = MY_STRING.split("\n").collect();

    for line in lines {
        let words:Vec<&str> = line.split(" ").collect();

        print!(" llll  {}", words[6]);

        print!("{}", "\n");
    }


   // let split_text: Vec<&str> = LINE.split(&MY_STRING).collect();
}