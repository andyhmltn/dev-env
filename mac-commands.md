Disable deep sleep when plugged in to power
---
```
	sudo pmset -a hibernatemode 0
	sudo pmset -a standby 0
	sudo pmset -a powernap 0
	sudo pmset -a tcpkeepalive 1
```
