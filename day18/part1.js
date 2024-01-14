const fs = require("fs");
let input = fs.readFileSync(
	"/home/kali/projects/aoc/rust23/day18/testcase.txt",
	"utf8"
);

let pos = [0, 0];
let path = [];
path.push(JSON.parse(JSON.stringify(pos)));

input = input.split("\n");
let path_x_min = 1e9;
let path_y_min = 1e9;
let path_x_max = -1e9;
let path_y_max = -1e9;

for (let step of input) {
	step = step.split(" ");
	for (let i = 0; i < parseInt(step[1]); i++) {
		switch (step[0]) {
			case "U":
				pos[0]--;
				break;
			case "D":
				pos[0]++;
				break;
			case "L":
				pos[1]--;
				break;
			case "R":
				pos[1]++;
				break;
		}
		path.push(JSON.parse(JSON.stringify(pos)));
		path_x_max = Math.max(pos[1], path_x_max);
		path_y_max = Math.max(pos[0], path_y_max);
		path_x_min = Math.min(pos[1], path_x_min);
		path_y_min = Math.min(pos[0], path_y_min);
	}
}

// remove last element, which is (0, 0)
path.pop();

// let x_segments = {};
// for (let i = 0; i < path.length; i++) {
// 	const idx = 0;
// 	const tile = path[i];
// 	if (
// 		i > 0 &&
// 		i < path.length - 1 &&
// 		path[i - 1][idx] == tile[idx] &&
// 		path[i + 1][idx] == tile[idx]
// 	) {
// 		continue;
// 	}

// 	// First of a segment
// 	if (i == 0 || path[i - 1][idx] != tile[idx]) {
// 		const key = tile[idx];
// 		if (!(key in x_segments)) {
// 			x_segments[key] = [[tile[1 - idx], i]];
// 		} else {
// 			x_segments[key].push([tile[1 - idx], i]);
// 		}
// 	}

// 	// Last of a segment
// 	if (i == path.length - 1 || path[i + 1][idx] != tile[idx]) {
// 		const key = tile[idx];
// 		let first_i = x_segments[key].at(-1).pop();
// 		x_segments[key].at(-1).push(tile[1 - idx]);
// 		x_segments[key].at(-1).sort();
// 	}
// }

// for (let segment in x_segments) {
// 	x_segments[segment].sort();
// }

// let inside_count = 0;
// for (let y = path_y_min; y <= path_y_max; y++) {
// 	for (let x = path_x_min; x <= path_x_max; x++) {
// 		// Check how many cuts in x_segments[y]
// 		let segment_i = 0;
// 		for (let segment of x_segments[y]) {
// 			if (x <= segment[1]) {
// 				break;
// 			}
// 			segment_i++;
// 		}
// 		if (segment_i == x_segments[y].length) {
// 			continue;
// 		}
// 		if (x >= x_segments[y][segment_i][0]) {
// 			continue;
// 		}

// 		if (segment_i % 2 == 0) {
// 			continue;
// 		}

// 		inside_count++;
// 	}
// }

// let ans = inside_count + path.length;
// console.log(ans);

function shoelace(points) {
	let area = 0;
	for (let i = 0; i < points.length; i++) {
		area +=
			(points[i][1] + points[(i + 1) % points.length][1]) *
			(points[i][0] - points[(i + 1) % points.length][0]);
	}
	return Math.abs(area) / 2;
}

let area = shoelace(path);
let internal = area - (path.length / 2 - 1);
let ans = internal + path.length;
console.log(ans);
