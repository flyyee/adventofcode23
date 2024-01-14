const fs = require("fs");
let input = fs.readFileSync(
	"/home/kali/projects/aoc/rust23/day18/testcase.txt",
	"utf8"
);

let pos = [0, 0];
let path_length = 0;
let vertices = [];
vertices.push(JSON.parse(JSON.stringify(pos)));

input = input.split("\n");

for (let step of input) {
	step = step.split(" ");
	let offset = parseInt(step[2].substring(2, step[2].length - 2), 0x10);
	let direction_idx = parseInt(
		step[2].substring(step[2].length - 2, step[2].length - 1),
		0x10
	);
	switch (direction_idx) {
		case 3: //"U":
			pos[0] -= offset;
			break;
		case 1: //"D":
			pos[0] += offset;
			break;
		case 2: //"L":
			pos[1] -= offset;
			break;
		case 0: //"R":
			pos[1] += offset;
			break;
	}
	path_length += offset;
	vertices.push(JSON.parse(JSON.stringify(pos)));
}

// remove last element, which is (0, 0)
vertices.pop();

function shoelace(points) {
	let area = 0;
	for (let i = 0; i < points.length; i++) {
		area +=
			(points[i][1] + points[(i + 1) % points.length][1]) *
			(points[i][0] - points[(i + 1) % points.length][0]);
	}
	return Math.abs(area) / 2;
}

let area = shoelace(vertices);
let internal = area - (path_length / 2 - 1);
let ans = internal + path_length;
console.log(ans);
