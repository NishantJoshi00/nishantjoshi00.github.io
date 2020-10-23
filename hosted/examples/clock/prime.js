function prime() {
	let a = []
	for (let i = 100; i >= 2; i--) {
		let isprime = true;
		for (let j = 2; j <= i / 2; j++) {
			if (i % j == 0 && i != j) {
				isprime = false;
				break;
			}
		}
		if (isprime) {
			a.push(i)
			console.log(i)
		}
	}
	console.log(`count: ${a.length}`);
}