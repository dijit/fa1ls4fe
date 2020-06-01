# FailSafe - a slack bot that invokes other slack bots

<img src="https://i.imgur.com/IHCn8Bn.png" alt="failsafe icon" height="50"/> <img src="https://i.imgur.com/yYZN1D5.png" alt="failsafe font" height="30"/> <img src="https://i.imgur.com/mpLlW9u.png" alt="failsafe icon" height="50"/>

Zeta is complicated; failsafe is not.

Failsafes job is to ensure that zeta is running.

To accomplish this goal we:

1) Copy a script over to our "working directory"
2) Execute a script on the machine where zeta can run.
  * Script checks to see if the process+db is running
  * If it's running return early
  * If not running: start a new process in tmux.


## Usage

```
# runtime needs this:
$ export SLACK_API_TOKEN=xxxx

# compiler needs this:
$ export SSH_USER=xxxx
$ export SSH_HOST=xxxx
$ export SSH_PASS=xxxx
$ cargo run
```
