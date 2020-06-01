extern crate ssh;
extern crate const_env;

use self::const_env::from_env;
use self::ssh::*;
use std::io::Read;
use std::io::Write;


#[from_env("SSH_HOST")]
const HOST: &'static str = "placeholder_hostname";
#[from_env("SSH_USER")]
const USER: &'static str = "placeholder_username";
#[from_env("SSH_PASS")]
const PASS: &'static str = "placeholder_password";

pub fn send_script() {
    /* Goal here is to upload the shell script we will execute later
     * We could read this in from a file, but that would mean
     * that I have to ship failsafe as more than just a single binary
     */
    let script: &'static str = r#"#!/usr/bin/env bash
set -u
set -o noglob
set -o pipefail

workingdir='/Users/jharasym/projects/external/errbot'

function start_zeta() {
    cd "${workingdir}";
    pg_ctl -D database status >/dev/null || \
        pg_ctl -D database start -l pgstartup.log
    /usr/local/bin/tmux new -d \
        "venv/bin/errbot -c config.py";
    echo -n "Started Instance";
}



#/Users/jharasym/projects/external/errbot/venv/bin/errbot -c /Users/jharasym/projects/external/errbot/config.py
ps aux | grep -v grep | grep -q "$workingdir/venv/bin/errbot -c $workingdir/config.py"
if [[ $? != 0 ]] ; then
    ps aux | grep -v grep | grep -q 'errbot -c config.py'
    if [[ $? != 0 ]] ; then
        lsof ${workingdir}/venv/bin/python > /dev/null;
        if [[ $? != 0 ]] ; then
            start_zeta;
            #>&2 echo "Not running; not programmed to start"
            #exit 1;
        else
            echo -n 'Zeta running (unknown start)'
            #lsof ${workingdir}/venv/bin/python;
        fi
    else
        echo -n 'Zeta running (manual start)'
    fi
else
    echo -n 'Zeta running (automated start)'
fi
"#;
    let mut session=Session::new().unwrap();
    session.set_host(&HOST).unwrap();
    session.parse_config(None).unwrap();
    session.connect().unwrap();
    session.set_username(&USER).unwrap();
    //session.userauth_publickey_auto(None).unwrap();
    match session.userauth_password(&PASS) {
        Ok(_) => {},
        Err(_) => return,
    }
    {
     let mut scp = session.scp_new(
             WRITE, 
             "/Users/jharasym/projects/external/errbot/")
         .unwrap();
     scp.init().unwrap();
     let buf=script.as_bytes().to_vec();
     scp.push_file("ensure_alive.sh",buf.len(),0o700).unwrap();
     scp.write(&buf).unwrap();
    }

}

pub fn try_start() -> (bool, String) {
    /* Goal of this function is to SSH to my mac
     * And run a special idempotent shell/python shell shit
     * to make sure zeta is running and responding.
     * if this function returns, we better make damned sure 
     * zeta is running. Panicking here is not an option.
     */
    let mut success = false;
    let mut session=Session::new().unwrap();
    session.set_host(&HOST).unwrap();
    session.set_username(&USER).unwrap();
    session.parse_config(None).unwrap();
    match session.connect() {
        Ok(_) => {},
        Err(error) => return (false, error.to_string()),
    }

    //println!("{:?}",session.is_server_known());

    match session.userauth_password(&PASS) {
        Ok(_) => {},
        Err(error) => return (false, error.to_string()),
    }
    {
        let mut s=session.channel_new().unwrap();
        s.open_session().unwrap();
        s.request_exec(
            b"/Users/jharasym/projects/external/errbot/ensure_alive.sh")
            .unwrap();
        let exit = s.get_exit_status().unwrap();
        match exit {
            0 => success=true,
            _ => println!("[✗] Script exit status is: {:?}", exit),
        }
        s.send_eof().unwrap();
        let mut buf=Vec::new();
        if success {
            s.stdout().read_to_end(&mut buf).unwrap();
        } else {
            // FIXME: this will crash if there's nothing in the buffer.
            match s.stderr().read_to_end(&mut buf) {
                Ok(_) => {},
                Err(error) => println!("Got error while trying to read stderr: {:?}", error),
            }
        }
        return (
            success,
            format!("{:?}", std::str::from_utf8(&buf).unwrap())
        );
    }

}
