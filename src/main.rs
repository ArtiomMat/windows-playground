use std::env;

use windows::Win32::Foundation::*;
use windows::Win32::Security::*;
use windows::Win32::System::Threading::*;
use windows::core::*;

use los32::*;

mod los32;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let pid: Option<u32> = if args.len() > 1 {
        args[1].trim().parse().ok()
    } else {
        None
    };

    let process = if let Some(pid) = pid {
        ProcessHandle::open(PROCESS_QUERY_INFORMATION, false, pid)?
    } else {
        println!("PID provided is None, will use current process.");
        ProcessHandle::get_current()
    };
    
    let token = process.open_token(TOKEN_ALL_ACCESS)?;
    let owner = token.get_owner()?;
    
    let info_result = process.get_security_info();
    println!("Result of get_security_info()={:?}", info_result);

    let owner_sid = Sid::copy_raw(owner.Owner);

    println!("Owner SID={:?}", owner_sid.lookup_local_account());

    Ok(())
}
