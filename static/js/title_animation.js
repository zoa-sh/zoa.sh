function animateTitle() {
  i >= message.length - 1 ? (i = 0) : i++,
    (document.title = message[i]),
    setTimeout("animateTitle()", 200);
}

var message = [
    "~$ ./ZOA.SH ⛧ ▁▂▃▄▅▆▇ ",
    "~# ./ZOA.SH ⛧ ▂▁▂▃▄▅▆",
    "~> ./ZOA.SH ⛥ ▃▂▁▂▃▄▅",
    "~$ ./ZOA.SH ⛥ ▄▃▂▁▂▃▄",
    "~# ./ZOA.SH ⛧ ▅▄▃▂▁▂▃",
    "~> ./ZOA.SH ⛧ ▆▅▄▃▂▁▂",
    "~$ ./ZOA.SH ⛥ ▇▆▅▄▃▂▁",
    "~# ./ZOA.SH ⛥ ▆▇▆▅▄▃▂",
    "~> ./ZOA.SH ⛧ ▅▆▇▆▅▄▃",
    "~$ ./ZOA.SH ⛧ ▄▅▆▇▆▅▄",
    "~# ./ZOA.SH ⛥ ▃▄▅▆▇▆▅",
    "~> ./ZOA.SH ⛥ ▂▃▄▅▆▇▆",
  ],
  i = 0;
animateTitle();
