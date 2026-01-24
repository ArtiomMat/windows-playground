use std::env;

use windows::Win32::Foundation::*;
use windows::Win32::Security::*;
use windows::Win32::System::Threading::*;
use windows::core::*;

use los32::*;

mod los32;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let pid: u32;

    if args.len() > 1 {
        pid = args[1].trim().parse().unwrap_or(0);
    } else {
        println!("Usage: {} <PID>", args[0]);
        return Err(WIN32_ERROR(0).into());
    }

    let process = ProcessHandle::open(PROCESS_QUERY_INFORMATION, false, pid)?;
    let token = process.open_token(TOKEN_ALL_ACCESS)?;
    let owner = token.get_owner()?;

    let owner_sid = Sid::copy_raw(owner.Owner);

    println!("Owner SID={:?}", owner_sid.lookup_local_account_sid());

    Ok(())
}
