// let today = new Date();
// let seconds = today.getSeconds();
// let minutes = today.getMinutes();
// let hours = today.getHours();
let today = new Date();
let seconds = today.getSeconds();
let minute = today.getMinutes();
let hours = today.getHours();

let ansec = anime({
	targets: '.second',
	width: '100%',
	duration: 1000 * 60,
	loop: true,
	autoplay: false,
	easing: 'linear',
});
ansec.seek(ansec.duration * (seconds / 60))
ansec.play()
let anhr = anime({
	targets: '.hour',
	width: '100%',
	duration: 1000 * 60 * 60 * 12,
	easing: 'linear',
	autoplay: false,
	loop: true
});
anhr.seek(anhr.duration * ((hours % 12) / 12))
anhr.play()
let anmi = anime({
	targets: '.minute',
	width: '100%',
	duration: 1000 * 60 * 60,
	easing: 'linear',
	autoplay: false,
	loop: true
});
anmi.seek(anmi.duration * (minutes / 60));
anmi.play()
setInterval(function() {
	if (seconds == 59) {
		seconds = 0;
		if (minutes == 59) {
			minutes = 0;
			if (hours == 23) {
				hours = 0;
			} else {
				hours++;
			}
			// document.querySelector('#hour').innerHTML = `hour: ${hours}`
		} else {
			minutes++;
		}
		// document.querySelector('#minute').innerHTML = `minute: ${minutes}`
	} else {
		seconds++;
	}
	document.querySelector('#time').innerHTML = `${hours}:${minutes}:${seconds}`
}, 1000)