#!/usr/bin/env bash
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
