let mouse = { pX:0, pY:0, X: 0, Y: 0 };
let target = {
	X: 0, Y: 0, radius: 0
};
const sens = .29;
let difficulty = 1;
let score = 0;
let tracker = { hit: 0, miss: 0 }
function setup() {
	createCanvas(displayWidth, displayHeight);
	fullscreen();
	cursor(CROSS);
	mouseX = width / 2;
	mouseY = height / 2;
	target.X = width / 2;
	target.Y = height / 2;
	target.radius = 30 * 5 / difficulty; 

	frameRate(120);
}
  
function draw() {
	background(0);
	strokeWeight(2);
	stroke('white');
	ellipse(target.X, target.Y, target.radius, target.radius);
	target.radius = 30 * 5 / difficulty;
	line(pmouseX, pmouseY, mouseX, mouseY);
	stroke('white');
	fill('white')
	text(Math.round(tracker.hit / tracker.miss, 2),10,10,2 );
}


function mouseClicked() {
	if ((mouseX > target.X - target.radius / 2 && mouseX < target.X + target.radius /2) && (mouseY > target.Y - target.radius /2 && mouseY < target.Y + target.radius /2)) {
		target.X = random(100, width - 100);
		target.Y = random(100, height - 100);
		tracker.hit += 1
		difficulty += .05
	} else {
		tracker.miss += 1
		difficulty -= .05
	}
	print(JSON.stringify(tracker));
	return false;
  }	