# RHCTRL

Rust Hosts Controller


# Usage


```bash
$ cat /etc/hosts
127.0.0.1 twitter.com #rhctrl M:22-23 T:9-18 W:9-18 R:9-18 F:9-20 S:allow U:allow

# Scan /etc/hosts every 5 minutes and write it back to /etc/hosts
$ rhctrl --source /etc/hosts --destination /etc/hosts --interval 5
```

Define inline time intervals in /etc/hosts when given line should be commented out.

# Why?

I was too lazy to change that by hand.

# TODO:

- [ ] manual temporary block/unblock
- [ ] better error handling
- [ ] consider moving this into its own line to capture multiple lines under one group
